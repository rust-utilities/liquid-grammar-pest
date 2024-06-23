#!/usr/bin/env rust

use std::io::Result;
use std::{cmp, fs};

use clap::{CommandFactory, Parser as ClapParser};
use clap_complete::Shell;
use colored::Colorize;

use liquid_grammar_pest::{LiquidParser, Rule};
use pest::Parser;

#[derive(ClapParser, Debug)]
#[clap(author, version)]
#[clap(about, verbatim_doc_comment)]
#[clap(arg_required_else_help = true)]
/// CLI application arguments for search-direct-messages
///
/// ## Developers may wish to review
///
/// - https://github.com/clap-rs/clap/blob/v3.0.14/examples/derive_ref/README.md#arg-types
/// - https://github.com/clap-rs/clap/issues/3198
pub struct Args {
	/// Path to input file
	///
	/// ## Example check file does not cause Liquid parser errors
	///
	/// ```
	/// cargo run --example check-file -- \
	///   --input-file "some-post.md"
	/// ```
	#[arg(long, verbatim_doc_comment, value_hint = clap::ValueHint::FilePath)]
	pub input_file: Option<String>,

	/// Attempt to output shell completions
	///
	/// ## Example
	///
	/// ```
	/// cargo run --example check-file -- \
	///   --build-completions bash
	/// ```
	#[arg(long, verbatim_doc_comment, required = false)]
	#[clap(value_enum)]
	pub build_completions: Option<Shell>,
}

/// Standardize formatting when printing pair rule for print-based debugging
///
/// Note: this injects newlines before and after `pair`
fn format_debug_pair(pair: pest::iterators::Pair<Rule>, label: &str) -> String {
	format!(
		"<!-- Start {} -->\n{}\n<!-- End {} -->",
		label,
		pair.as_str(),
		label,
	)
}

/// Match content and Liquid code
///
/// TODO: maybe consider detecting, and consider recursion, when inner pair exists
fn check_content_or_code<F>(pair: pest::iterators::Pair<Rule>, callback: F) -> String
where
	F: Fn(pest::iterators::Pair<Rule>, &str) -> String,
{
	// eprintln!("check_content_or_code -- Pair -> {:?}", pair);

	match pair.as_rule() {
		// Agnostic to iteration
		Rule::content__block => callback(pair, "Content"),
		Rule::liquid__block__iteration__for => callback(pair, "`for` block"),
		Rule::liquid__block__comment => callback(pair, "`comment` block"),
		Rule::liquid__block__raw => callback(pair, "`raw` block"),
		// Outside iteration
		Rule::liquid__block__control_flow__branch__outside_iteration => {
			callback(pair, "`if` block outside iteration")
		}
		Rule::liquid__block__control_flow__case__outside_iteration => {
			callback(pair, "`case` block outside iteration")
		}
		Rule::liquid__block__iteration__tablerow__outside_iteration => {
			callback(pair, "`tablerow` block outside iteration")
		}
		Rule::liquid__tag__liquid__outside_iteration => {
			callback(pair, "`liquid` tag outside iteration")
		}
		Rule::liquid__block__capture__outside_iteration => {
			callback(pair, "`capture` block outside iteration")
		}
		// Within iteration
		Rule::liquid__tag__iteration__for__continue => {
			callback(pair, "`continue` tag within iteration")
		}
		Rule::liquid__tag__iteration__for__break => callback(pair, "`break` tag within iteration"),
		Rule::liquid__block__control_flow__branch__within_iteration => {
			callback(pair, "`if` block within iteration")
		}
		Rule::liquid__block__control_flow__case__within_iteration => {
			callback(pair, "`case` block within iteration")
		}
		Rule::liquid__block__iteration__tablerow__within_iteration => {
			callback(pair, "`tablerow` block within iteration")
		}
		Rule::liquid__tag__liquid__within_iteration => {
			callback(pair, "`liquid` tag within iteration")
		}
		Rule::liquid__block__capture__within_iteration => {
			callback(pair, "`capture` block within iteration")
		}
		// Done messed-up if we get here
		_ => panic!("Unexpected rule pair -> {:?}", pair),
	}
}

/// Get line and column number from Pest error variants
fn extract_line_column_numbers_from_error<R>(error: pest::error::Error<R>) -> (usize, usize) {
	match error.line_col {
		pest::error::LineColLocation::Pos((line, column)) => {
			// eprintln!("(line, column) -> ({:?}, {:?})", line, column);
			(line, column)
		}
		pest::error::LineColLocation::Span(
			(start_line, start_column),
			(_end_line, _end_column),
		) => {
			// eprintln!(
			// 	"(start_line, start_column) -> ({:?}, {:?})",
			// 	start_line, start_column
			// );
			// eprintln!(
			// 	"(_end_line, _end_column) -> ({:?}, {:?})",
			// 	_end_line, _end_column
			// );
			(start_line, start_column)
		}
	}
}

/// Attempt to pretty-print parser error with some context
///
/// Note: for Liquid blocks this works best when code is all on one line, for example the following
/// reports error at the end of `endif %`...
///
/// ```liquid
/// {% if foo == false %} WaT {% endif %
/// ```
///
/// ... Where as the next example reports error after `false %}`
///
/// ```liquid
/// {% if foo == false %}
/// WaT
/// {% endif %
/// ```
fn format_error<R>(
	error: pest::error::Error<R>,
	input_file: &str,
	file_content: &String,
) -> String {
	let (mut last_bad_line, mut last_bad_column) = extract_line_column_numbers_from_error(error);
	while last_bad_line > 1 {
		last_bad_line -= 1;
		let file_content_split: Vec<_> = file_content.split("\n").collect();
		let file_content_truncated = file_content_split[0..last_bad_line].join("\n");
		match LiquidParser::parse(Rule::document, &file_content_truncated) {
			Ok(_pairs) => {
				let colored_info_bar = "|".cyan();
				let colored_bad_line_number = (last_bad_line + 1).to_string().cyan();
				let length_of_line_number = format!("{}", (last_bad_line + 1)).len();
				let error_prefix = format!(
					"{} {}",
					colored_bad_line_number,
					colored_info_bar.to_string()
				);
				let context_prefix = format!("{} |", " ".repeat(length_of_line_number));
				let context_prefix_length = context_prefix.len();

				let colored_context_prefix = format!(
					"{} {}",
					" ".repeat(length_of_line_number),
					colored_info_bar.to_string()
				);

				let mut error_lines = vec![
					"Error: failed to parse input as Liquid"
						.red()
						.bold()
						.to_string(),
					format!(
						"{} {} {}:{}:{}",
						" ".repeat(length_of_line_number - 1),
						"-->".cyan().to_string(),
						input_file,
						last_bad_line + 1,
						last_bad_column
					),
				];

				file_content_split[cmp::max(0, last_bad_line - 3)..last_bad_line]
					.into_iter()
					.for_each(|line| {
						error_lines.push(format!("{} {}", colored_context_prefix, line));
					});

				error_lines.push(format!(
					"{} {}",
					error_prefix, file_content_split[last_bad_line]
				));

				error_lines.push(
					format!(
						"{}^ Maybe somewhere around here",
						" ".repeat(last_bad_column + context_prefix_length)
					)
					.red()
					.bold()
					.to_string(),
				);

				return error_lines.join("\n");
			}
			Err(error) => {
				(last_bad_line, last_bad_column) = extract_line_column_numbers_from_error(error);
			}
		}
	}

	panic!(
		"File cannot be parsed! (last_bad_line, last_bad_column) -> ({}, {})",
		last_bad_line, last_bad_column
	);
}

///
fn main() -> Result<()> {
	let args = Args::parse();

	// Display tab-completion configuration for given shell then exit
	if let Some(shell) = args.build_completions {
		// ## Resources for further reading
		//
		// - https://github.com/clap-rs/clap/blob/master/clap_complete/examples/completion-derive.rs
		// - https://github.com/clap-rs/clap/discussions/3671
		// - https://github.com/clap-rs/clap/discussions/2417
		println!("#!/usr/bin/env {}", shell.to_string().to_lowercase());
		let mut cmd = Args::command();
		let name = cmd.get_name().to_string();
		clap_complete::generate(shell, &mut cmd, &name, &mut std::io::stdout());
		std::process::exit(0);
	}

	// Ensure required CLI values are present
	let input_file = args.input_file.expect("Undefined value for: --input-file");

	// Read file into String
	let file_content = fs::read_to_string(input_file.clone())?;

	match LiquidParser::parse(Rule::document, &file_content) {
		Err(error) => {
			// eprintln!("Error: {:?}", error);
			eprintln!("{}", format_error(error, &input_file, &file_content));
		}
		Ok(mut pairs) => {
			for pair in pairs.next().unwrap().into_inner() {
				match pair.as_rule() {
					Rule::front_matter => {
						println!("{}", format_debug_pair(pair, "FrontMatter"));
					}
					Rule::EOI => {
						println!("<!-- End of Input -->");
					}
					_ => {
						println!("{}", check_content_or_code(pair, format_debug_pair));
					}
				}
			}
		}
	};

	Ok(())
}
