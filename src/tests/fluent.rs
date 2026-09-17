//! Compatibility with real Fluent string selectors; no new Fluent numeric type.
use super::*;
use fluent_typed::prelude::{FluentArgs, L10nBundle};

#[test]
fn localized_pair_drives_language_specific_branches_and_preserves_bidi_isolation() {
	for (language, locale, source, input, expected) in [
		(
			"en",
			"en",
			include_str!("../../examples/assets/localizations/translations/en/numbers.ftl"),
			"1",
			"\u{2068}1\u{2069} item remaining",
		),
		(
			"en",
			"en",
			include_str!("../../examples/assets/localizations/translations/en/numbers.ftl"),
			"1.0",
			"\u{2068}1.0\u{2069} items remaining",
		),
		(
			"es",
			"es",
			include_str!("../../examples/assets/localizations/translations/es/numbers.ftl"),
			"2",
			"Quedan \u{2068}2\u{2069} elementos",
		),
		(
			"ru",
			"ru",
			include_str!("../../examples/assets/localizations/translations/ru/numbers.ftl"),
			"2",
			"Осталось \u{2068}2\u{2069} предмета",
		),
		(
			"ar",
			"ar-EG-u-nu-arab",
			include_str!("../../examples/assets/localizations/translations/ar/numbers.ftl"),
			"3",
			"تبقت \u{2068}٣\u{2069} عناصر",
		),
	] {
		let formatter = formatter(locale, Precision::preserve());
		let number = formatter
			.localize(&input.parse().unwrap(), PluralRuleType::Cardinal)
			.unwrap();
		let bundle = L10nBundle::new(language, source.as_bytes()).unwrap();
		let mut args = FluentArgs::new();
		args.set("value", number.text());
		args.set("plural", number.selector());

		assert_eq!(bundle.msg("remaining", Some(args)).unwrap(), expected);
		assert!(bundle.msg("empty", None).is_ok());
	}
}

#[test]
fn literal_categories_and_native_exact_numeric_cases_remain_distinct() {
	let bundle = L10nBundle::new(
		"en",
		b"test = { $selector ->\n    [0] exact\n    [one] singular\n   *[other] fallback\n}\n",
	)
	.unwrap();

	for (selector, expected) in [
		("one", "singular"),
		("zero", "fallback"),
		("0", "fallback"),
		("unknown", "fallback"),
	] {
		let mut args = FluentArgs::new();
		args.set("selector", selector);
		assert_eq!(bundle.msg("test", Some(args)).unwrap(), expected);
	}

	let mut args = FluentArgs::new();
	args.set("selector", 0);
	assert_eq!(bundle.msg("test", Some(args)).unwrap(), "exact");
}
