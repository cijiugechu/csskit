use super::{ColumnRuleStyleValue, RowRuleStyleValue};
use crate::GapRuleList;
use css_parse::{Cursor, Parse, Parser, Result as ParseResult};

impl<'a> Parse<'a> for ColumnRuleStyleValue<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<GapRuleList<'a>>().map(Self::GapRuleList)
	}
}

impl<'a> Parse<'a> for RowRuleStyleValue<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<GapRuleList<'a>>().map(Self::GapRuleList)
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
		assert_parse!(
			CssAtomSet::ATOMS,
			ColumnRuleStyleValue,
			"repeat(auto, 1px solid red), repeat(auto, 2px dashed green)"
		);
		assert_parse_error!(CssAtomSet::ATOMS, ColumnRuleStyleValue, "repeat(auto,)");
		assert_parse!(
			CssAtomSet::ATOMS,
			RowRuleStyleValue,
			"repeat(auto, 1px solid red), repeat(auto, 2px dashed green)"
		);
		assert_parse_error!(CssAtomSet::ATOMS, RuleStyleValue, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, RuleStyleValue, "1px solid red,");
	}
}
