#!/usr/bin/env rust

use liquid_grammar_pest::{LiquidParser, Rule};
use pest::Parser;

#[test]
fn assign() {
	let input = r#"{% assign name = "value" %}"#;
	let pairs = LiquidParser::parse(Rule::liquid__tag__assign, input)
		.expect("Failed to parse")
		.next()
		.unwrap();

	assert_eq!(pairs.as_rule(), Rule::liquid__tag__assign);
	assert_eq!(pairs.as_str(), input);
}
