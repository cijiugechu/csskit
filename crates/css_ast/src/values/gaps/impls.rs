use super::{ColumnRuleStyleValue, RowRuleStyleValue};
use crate::{GapAutoRuleList, GapRuleList};
use css_parse::{Cursor, Diagnostic, Parse, Parser, Result as ParseResult};

fn parse_gap_rule_or_auto<'a, I, T>(
	p: &mut Parser<'a, I>,
	from_rule: fn(GapRuleList<'a>) -> T,
	from_auto: fn(GapAutoRuleList<'a>) -> T,
) -> ParseResult<T>
where
	I: Iterator<Item = Cursor> + Clone,
{
	let start = p.peek_n(1);
	let checkpoint = p.checkpoint();
	if let Ok(value) = p.parse::<GapRuleList<'a>>() {
		return Ok(from_rule(value));
	}
	p.rewind(checkpoint);
	if let Ok(value) = p.parse::<GapAutoRuleList<'a>>() {
		return Ok(from_auto(value));
	}
	Err(Diagnostic::new(start, Diagnostic::unexpected))
}

impl<'a> Parse<'a> for ColumnRuleStyleValue<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		parse_gap_rule_or_auto(p, Self::GapRuleList, Self::GapAutoRuleList)
	}
}

impl<'a> Parse<'a> for RowRuleStyleValue<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		parse_gap_rule_or_auto(p, Self::GapRuleList, Self::GapAutoRuleList)
	}
}

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "1px solid red");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "1px solid red, repeat(2, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "repeat(auto, 1px solid red)");
		assert_parse!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "1px solid red, repeat(auto, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, RowRuleStyleValue, "repeat(auto, 1px solid red), 2px dashed green");
		assert_parse!(CssAtomSet::ATOMS, RuleStyleValue, "1px solid red, repeat(2, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, RuleStyleValue, "repeat(auto, 1px solid red), 2px dashed green");
		assert_parse!(CssAtomSet::ATOMS, RuleStyleValue, "1px solid red, repeat(auto, 2px dashed green)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "repeat(auto, 1px solid red), repeat(auto, 2px dashed green)");
		assert_parse_error!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, RowRuleStyleValue, "repeat(auto, 1px solid red), repeat(auto, 2px dashed green)");
		assert_parse_error!(CssAtomSet::ATOMS, RuleStyleValue, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, RuleStyleValue, "1px solid red,");
	}
}
