#!/usr/bin/env rust

//! ## Example MarkDown/Liquid input
//!
//! ```markdown
//! ---
//! # YAML comment
//! title: Super awesome post
//! description: A post about some topic
//! ---
//!
//! Content of post before Liquid code
//!
//! {% if collection %}
//! <ul>
//!   {% for item in collection %}
//! <li>{{ item }}</li>
//!   {% endfor %}
//! </ul>
//! {% endif %}
//!
//! Content of post after Liquid code
//! ```
//!
//! ## Expected parser result
//!
//! ```text
//! - document
//!   - front_matter > yaml_content: "# YAML comment\ntitle: Super awesome post\ndescription: A post about some topic\n"
//!   - content__block: "\n\nContent of post before Liquid code\n\n"
//!   - liquid__block__control_flow__branch__outside_iteration > liquid__block__control_flow__branch__outside_iteration__plain
//!     - liquid__block_subsection__control_flow__branch__outside_iteration__plain__start
//!       - liquid__tag__open__plain: "{%"
//!       - liquid__key_word__control_flow__branch: "if"
//!       - liquid__code__control_flow__branch__comparison__expression > liquid__code__control_flow__branch__comparison__expression__pair
//!         - liquid__code__control_flow__branch__comparison__operator__word > liquid__type > object > object__word: "collection"
//!         - liquid__code__control_flow__branch__comparison__operator > greater: ">"
//!         - liquid__code__control_flow__branch__comparison__operator__word > liquid__type > number: "0"
//!       - liquid__tag__close__plain: "%}"
//!       - content__block: "\n<ul>\n  "
//!       - liquid__block__iteration__for > liquid__block__iteration__for__plain
//!         - liquid__block_subsection__iteration__for__plain__start
//!           - liquid__tag__open__plain: "{%"
//!           - liquid__key_word__iteration__for: "for"
//!           - object__word: "item"
//!           - liquid__code__iteration__target > object > object__word: "collection"
//!           - liquid__tag__close__plain: "%}"
//!           - content__block: "\n<li>{{ item }}</li>\n  "
//!         - liquid__tag__plain__end
//!           - liquid__tag__open__plain: "{%"
//!           - liquid__code__end: "endfor"
//!           - liquid__tag__close__plain: "%}"
//!       - content__block: "\n</ul>\n"
//!     - liquid__tag__plain__end
//!       - liquid__tag__open__plain: "{%"
//!       - liquid__code__end: "endif"
//!       - liquid__tag__close__plain: "%}"
//!   - content__block: "\n\nContent of post after Liquid code\n"
//!   - EOI: ""
//! ```

// TODO: Sort out why documentation says to have following `use` but compiler complains
// use pest::Parser;
use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "grammars/liquid.pest"]
pub struct LiquidParser;

