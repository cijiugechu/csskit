#![allow(unused)]
use super::prelude::*;

use crate::{Color, LineStyle, LineWidth, PositiveNonZeroInt};

// https://drafts.csswg.org/css-gaps-1/#typedef-gap-rule-list
// <gap-rule-list> = <gap-rule-or-repeat>#
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GapRuleList<'a> {
	pub items: Vec<'a, (GapRuleOrRepeat<'a>, Option<T![,]>)>,
}

// <gap-rule-or-repeat> = <gap-rule> | <gap-repeat-rule>
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum GapRuleOrRepeat<'a> {
	GapRule(GapRule<'a>),
	GapRepeatRule(GapRepeatRule<'a>),
}

// <gap-repeat-rule> = repeat( <integer [1,∞]> , <gap-rule># )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GapRepeatRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub count: PositiveNonZeroInt,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub comma: T![,],
	pub rules: Vec<'a, (GapRule<'a>, Option<T![,]>)>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: T![')'],
}

// <gap-rule> = <line-width> || <line-style> || <color>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GapRule<'a> {
	pub items: Vec<'a, GapRuleItem>,
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum GapRuleItem {
	LineWidth(LineWidth),
	LineStyle(LineStyle),
	Color(Color),
}

fn parse_comma_separated<'a, I, T>(p: &mut Parser<'a, I>) -> ParserResult<Vec<'a, (T, Option<T![,]>)>>
where
	I: Iterator<Item = Cursor> + Clone,
	T: Parse<'a> + Peek<'a>,
{
	let mut items = Vec::new_in(p.bump());
	loop {
		let item = p.parse::<T>()?;
		let comma = p.parse_if_peek::<T![,]>()?;
		items.push((item, comma));
		if items.last().and_then(|(_, c)| *c).is_none() {
			break;
		}
	}
	Ok(items)
}

impl<'a> Peek<'a> for GapRuleList<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		GapRuleOrRepeat::peek(p, c)
	}
}

impl<'a> Parse<'a> for GapRuleList<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let items = parse_comma_separated::<_, GapRuleOrRepeat<'a>>(p)?;
		Ok(Self { items })
	}
}

impl<'a> Peek<'a> for GapRepeatRule<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Repeat)
			&& PositiveNonZeroInt::peek(p, p.peek_n(2))
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
		let count = p.parse::<PositiveNonZeroInt>()?;
		let comma = p.parse::<T![,]>()?;
		let rules = parse_comma_separated::<_, GapRule<'a>>(p)?;
		let close = p.parse::<T![')']>()?;
		Ok(Self { name, count, comma, rules, close })
	}
}

impl<'a> Peek<'a> for GapRule<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if <T![Function]>::peek(p, c) && p.equals_atom(c, &CssAtomSet::Repeat) {
			return false;
		}
		GapRuleItem::peek(p, c)
	}
}

impl<'a> Parse<'a> for GapRule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if !Self::peek(p, p.peek_n(1)) {
			Err(Diagnostic::new(p.peek_n(1), Diagnostic::unexpected))?
		}

		let mut has_width = false;
		let mut has_style = false;
		let mut has_color = false;
		let mut items = Vec::new_in(p.bump());

		while GapRuleItem::peek(p, p.peek_n(1)) {
			let c = p.peek_n(1);
			let item = p.parse::<GapRuleItem>()?;
			match item {
				GapRuleItem::LineWidth(_) if has_width => Err(Diagnostic::new(c, Diagnostic::unexpected))?,
				GapRuleItem::LineStyle(_) if has_style => Err(Diagnostic::new(c, Diagnostic::unexpected))?,
				GapRuleItem::Color(_) if has_color => Err(Diagnostic::new(c, Diagnostic::unexpected))?,
				GapRuleItem::LineWidth(_) => has_width = true,
				GapRuleItem::LineStyle(_) => has_style = true,
				GapRuleItem::Color(_) => has_color = true,
			}
			items.push(item);
		}

		Ok(Self { items })
	}
}

impl<'a> css_parse::ToCursors for GapRuleList<'a> {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		for (item, comma) in &self.items {
			item.to_cursors(s);
			comma.to_cursors(s);
		}
	}
}

impl<'a> css_parse::ToCursors for GapRepeatRule<'a> {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		self.name.to_cursors(s);
		self.count.to_cursors(s);
		self.comma.to_cursors(s);
		for (rule, comma) in &self.rules {
			rule.to_cursors(s);
			comma.to_cursors(s);
		}
		self.close.to_cursors(s);
	}
}

impl<'a> css_parse::ToCursors for GapRule<'a> {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		for item in &self.items {
			item.to_cursors(s);
		}
	}
}

impl<'a> css_parse::ToSpan for GapRuleList<'a> {
	fn to_span(&self) -> css_parse::Span {
		let first = self.items.first().expect("gap-rule-list has at least one item").0.to_span();
		let last = self.items.last().expect("gap-rule-list has at least one item");
		let last_span = last.1.map_or_else(|| last.0.to_span(), |comma| comma.to_span());
		first + last_span
	}
}

impl<'a> css_parse::ToSpan for GapRepeatRule<'a> {
	fn to_span(&self) -> css_parse::Span {
		self.name.to_span() + self.close.to_span()
	}
}

impl<'a> css_parse::ToSpan for GapRule<'a> {
	fn to_span(&self) -> css_parse::Span {
		let first = self.items.first().expect("gap-rule has at least one item").to_span();
		let last = self.items.last().expect("gap-rule has at least one item").to_span();
		first + last
	}
}

impl<'a> css_parse::SemanticEq for GapRuleList<'a> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self == other
	}
}

impl<'a> css_parse::SemanticEq for GapRepeatRule<'a> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self == other
	}
}

impl<'a> css_parse::SemanticEq for GapRule<'a> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self == other
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
		assert_eq!(std::mem::size_of::<GapRuleOrRepeat>(), 80);
		assert_eq!(std::mem::size_of::<GapRepeatRule>(), 80);
		assert_eq!(std::mem::size_of::<GapRule>(), 32);
		assert_eq!(std::mem::size_of::<GapRuleItem>(), 140);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red, 2px dashed green, 3px dotted blue");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red, repeat(2, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "repeat(2, 1px solid red, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "solid 1px red");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(auto, 1px solid red)");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(0, 1px solid red)");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(2,)");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(2, repeat(2, 1px solid red))");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "1px solid red,");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "1px solid red solid");
	}
}
