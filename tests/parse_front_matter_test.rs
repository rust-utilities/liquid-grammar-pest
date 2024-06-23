#!/usr/bin/env rust

use liquid_grammar_pest::{LiquidParser, Rule};
use pest::Parser;

#[test]
fn front_matter() {
	let input = r#"---
title: Super cool post
description: Something about some thing
---"#;

	let pairs = LiquidParser::parse(Rule::front_matter, input)
		.expect("Failed to parse")
		.next()
		.unwrap();

	assert_eq!(pairs.as_rule(), Rule::front_matter);
	assert_eq!(pairs.as_str(), input);
}
