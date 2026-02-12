#![allow(unused)]
use super::prelude::*;
use crate::{GapRule, GapRuleOrRepeat};

// https://drafts.csswg.org/css-gaps-1/#typedef-gap-auto-rule-list
// <gap-auto-rule-list> = <gap-rule-or-repeat>#? , <gap-auto-repeat-rule> , <gap-rule-or-repeat>#?
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GapAutoRuleList<'a> {
	pub items: Vec<'a, (GapAutoRuleListItem<'a>, Option<T![,]>)>,
}

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum GapAutoRuleListItem<'a> {
	GapAutoRepeatRule(GapAutoRepeatRule<'a>),
	GapRuleOrRepeat(GapRuleOrRepeat<'a>),
}

// <gap-auto-repeat-rule> = repeat( auto , <gap-rule># )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GapAutoRepeatRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub count: T![Ident],
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub comma: T![,],
	pub rules: Vec<'a, (GapRule<'a>, Option<T![,]>)>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: T![')'],
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

impl<'a> Peek<'a> for GapAutoRepeatRule<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if !<T![Function]>::peek(p, c) || !p.equals_atom(c, &CssAtomSet::Repeat) {
			return false;
		}
		let c2 = p.peek_n(2);
		let c3 = p.peek_n(3);
		<T![Ident]>::peek(p, c2) && p.equals_atom(c2, &CssAtomSet::Auto) && <T![,]>::peek(p, c3)
	}
}

impl<'a> Parse<'a> for GapAutoRepeatRule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let name = p.parse::<T![Function]>()?;
		if !p.equals_atom(name.into(), &CssAtomSet::Repeat) {
			Err(Diagnostic::new(name.into(), Diagnostic::unexpected_ident))?
		}
		let count = p.parse::<T![Ident]>()?;
		if !p.equals_atom(count.into(), &CssAtomSet::Auto) {
			Err(Diagnostic::new(count.into(), Diagnostic::unexpected_ident))?
		}
		let comma = p.parse::<T![,]>()?;
		let rules = parse_comma_separated::<_, GapRule<'a>>(p)?;
		let close = p.parse::<T![')']>()?;
		Ok(Self { name, count, comma, rules, close })
	}
}

impl<'a> Peek<'a> for GapAutoRuleListItem<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		GapAutoRepeatRule::peek(p, c) || GapRuleOrRepeat::peek(p, c)
	}
}

impl<'a> Parse<'a> for GapAutoRuleListItem<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if GapAutoRepeatRule::peek(p, p.peek_n(1)) {
			return p.parse::<GapAutoRepeatRule<'a>>().map(Self::GapAutoRepeatRule);
		}
		p.parse::<GapRuleOrRepeat<'a>>().map(Self::GapRuleOrRepeat)
	}
}

impl<'a> Peek<'a> for GapAutoRuleList<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		GapAutoRuleListItem::peek(p, c)
	}
}

impl<'a> Parse<'a> for GapAutoRuleList<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let start = p.peek_n(1);
		let items = parse_comma_separated::<_, GapAutoRuleListItem<'a>>(p)?;
		let auto_count = items
			.iter()
			.filter(|(item, _)| matches!(item, GapAutoRuleListItem::GapAutoRepeatRule(_)))
			.count();

		if auto_count != 1 {
			Err(Diagnostic::new(start, Diagnostic::unexpected))?
		}

		Ok(Self { items })
	}
}

impl<'a> css_parse::ToCursors for GapAutoRuleList<'a> {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		for (item, comma) in &self.items {
			item.to_cursors(s);
			comma.to_cursors(s);
		}
	}
}

impl<'a> css_parse::ToCursors for GapAutoRepeatRule<'a> {
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

impl<'a> css_parse::ToSpan for GapAutoRuleList<'a> {
	fn to_span(&self) -> css_parse::Span {
		let first = self.items.first().expect("gap-auto-rule-list has at least one item").0.to_span();
		let last = self.items.last().expect("gap-auto-rule-list has at least one item");
		let last_span = last.1.map_or_else(|| last.0.to_span(), |comma| comma.to_span());
		first + last_span
	}
}

impl<'a> css_parse::ToSpan for GapAutoRepeatRule<'a> {
	fn to_span(&self) -> css_parse::Span {
		self.name.to_span() + self.close.to_span()
	}
}

impl<'a> css_parse::SemanticEq for GapAutoRuleList<'a> {
	fn semantic_eq(&self, other: &Self) -> bool {
		self == other
	}
}

impl<'a> css_parse::SemanticEq for GapAutoRepeatRule<'a> {
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
		assert_eq!(std::mem::size_of::<GapAutoRuleList>(), 32);
		assert_eq!(std::mem::size_of::<GapAutoRuleListItem>(), 88);
		assert_eq!(std::mem::size_of::<GapAutoRepeatRule>(), 80);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GapAutoRuleList, "repeat(auto, 1px solid red)");
		assert_parse!(CssAtomSet::ATOMS, GapAutoRuleList, "1px solid red, repeat(auto, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapAutoRuleList, "repeat(auto, 1px solid red), 2px dashed green");
		assert_parse!(CssAtomSet::ATOMS, GapAutoRuleList, "1px solid red, repeat(auto, 2px dashed green), 3px dotted blue");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, GapAutoRuleList, "1px solid red, 2px dashed green");
		assert_parse_error!(
			CssAtomSet::ATOMS,
			GapAutoRuleList,
			"repeat(auto, 1px solid red), repeat(auto, 2px dashed green)"
		);
		assert_parse_error!(CssAtomSet::ATOMS, GapAutoRuleList, "repeat(2, 1px solid red)");
		assert_parse_error!(CssAtomSet::ATOMS, GapAutoRuleList, "repeat(auto,)");
	}
}
