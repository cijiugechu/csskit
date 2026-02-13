#![allow(unused)]
use super::prelude::*;

use crate::{AutoOr, Color, LineStyle, LineWidth, PositiveNonZeroInt};
use css_parse::{CommaSeparated, Optionals3};

// https://drafts.csswg.org/css-gaps-1/#typedef-gap-rule-list
// <gap-rule-list> = <gap-rule-or-repeat>#
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GapRuleList<'a>(pub CommaSeparated<'a, GapRuleOrRepeat<'a>>);

// <gap-rule-or-repeat> = <gap-rule> | <gap-repeat-rule>
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum GapRuleOrRepeat<'a> {
	GapRule(GapRule),
	GapRepeatRule(GapRepeatRule<'a>),
}

// <gap-repeat-rule> = repeat( <integer [1,∞]> , <gap-rule># )
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GapRepeatRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub count: AutoOr<PositiveNonZeroInt>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub comma: T![,],
	pub rules: CommaSeparated<'a, GapRule>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: T![')'],
}

// <gap-rule> = <line-width> || <line-style> || <color>
pub type GapRule = Optionals3<LineWidth, LineStyle, Color>;

impl<'a> Peek<'a> for GapRepeatRule<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Repeat)
			&& AutoOr::<PositiveNonZeroInt>::peek(p, p.peek_n(2))
			&& <T![,]>::peek(p, p.peek_n(3))
	}
}

impl<'a> Parse<'a> for GapRepeatRule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let name = p.parse::<T![Function]>()?;
		if !p.equals_atom(name.into(), &CssAtomSet::Repeat) {
			Err(Diagnostic::new(name.into(), Diagnostic::unexpected_ident))?
		}
		let count = p.parse::<AutoOr<PositiveNonZeroInt>>()?;
		let comma = p.parse::<T![,]>()?;
		let rules = p.parse::<CommaSeparated<'a, GapRule>>()?;
		let close = p.parse::<T![')']>()?;
		Ok(Self { name, count, comma, rules, close })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<GapRuleList>(), 32);
		assert_eq!(std::mem::size_of::<GapRuleOrRepeat>(), 176);
		assert_eq!(std::mem::size_of::<GapRepeatRule>(), 88);
		assert_eq!(std::mem::size_of::<GapRule>(), 172);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red, 2px dashed green, 3px dotted blue");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red, repeat(2, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red, repeat(auto, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "repeat(2, 1px solid red, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "repeat(auto, 1px solid red, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "solid 1px red");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(none, 1px solid red)");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(0, 1px solid red)");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(2,)");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(2, repeat(2, 1px solid red))");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "1px solid red,");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "1px solid red solid");
	}
}
