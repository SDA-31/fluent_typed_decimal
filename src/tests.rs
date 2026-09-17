//! Public API regressions; engine consumers belong in their own integration tests.
use crate::{
	Decimal, GroupingStrategy, NumberFormatter, NumberOptions, PluralCategory, PluralRuleType,
	Precision, RoundingMode, UnsignedRoundingMode,
};

mod fluent;
mod limits;
mod precision;

const HALF_EVEN: RoundingMode = RoundingMode::Unsigned(UnsignedRoundingMode::HalfEven);

fn formatter(language: &str, precision: Precision) -> NumberFormatter {
	NumberFormatter::try_new(
		&language.parse().unwrap(),
		NumberOptions {
			precision,
			grouping: GroupingStrategy::Never,
		},
	)
	.unwrap()
}

#[test]
fn cardinal_categories_cover_integer_fractional_and_negative_values() {
	use PluralCategory::{Few, Many, One, Other, Two, Zero};

	for (language, cases) in [
		(
			"en",
			vec![
				("0", Other),
				("1", One),
				("1.0", Other),
				("1.5", Other),
				("-1", One),
			],
		),
		(
			"ru",
			vec![
				("1", One),
				("2", Few),
				("5", Many),
				("11", Many),
				("21", One),
				("1.0", Other),
				("1.5", Other),
				("-2", Few),
			],
		),
		("es", vec![("1", One), ("2", Other), ("1.5", Other)]),
		(
			"ar",
			vec![
				("0", Zero),
				("1", One),
				("2", Two),
				("3", Few),
				("11", Many),
				("100", Other),
				("1.0", One),
				("1.5", Other),
			],
		),
		("he", vec![("1", One), ("2", Two), ("3", Other)]),
		("ja", vec![("0", Other), ("1", Other), ("2", Other)]),
	] {
		let formatter = formatter(language, Precision::preserve());

		for (input, expected) in cases {
			let number = formatter
				.localize(&input.parse().unwrap(), PluralRuleType::Cardinal)
				.unwrap();
			assert_eq!(number.category(), expected, "{language}: {input}");
		}
	}
}

#[test]
fn arabic_digit_preference_does_not_change_grammar() {
	let arabic = formatter("ar-EG-u-nu-arab", Precision::preserve());
	let latin = formatter("ar-EG-u-nu-latn", Precision::preserve());
	let value: Decimal = "3.00".parse().unwrap();
	let a = arabic.localize(&value, PluralRuleType::Cardinal).unwrap();
	let b = latin.localize(&value, PluralRuleType::Cardinal).unwrap();

	assert_eq!(a.text(), "٣٫٠٠");
	assert_eq!(b.text(), "3.00");
	assert_eq!(a.category(), PluralCategory::Few);
	assert_eq!(a.category(), b.category());
}

#[test]
fn ordinal_rules_are_independent_of_cardinal_rules() {
	use PluralCategory::{Few, One, Other, Two};
	let formatter = formatter("en", Precision::preserve());

	for (input, expected) in [
		(1, One),
		(2, Two),
		(3, Few),
		(4, Other),
		(11, Other),
		(12, Other),
		(13, Other),
		(21, One),
		(22, Two),
		(23, Few),
	] {
		assert_eq!(
			formatter
				.category(&Decimal::from(input), PluralRuleType::Ordinal)
				.unwrap(),
			expected
		);
	}

	assert_eq!(
		formatter
			.category(&Decimal::from(2), PluralRuleType::Cardinal)
			.unwrap(),
		Other
	);
}

#[test]
fn independent_formatters_do_not_share_global_language_state() {
	let value: Decimal = "1234.50".parse().unwrap();
	let english = NumberFormatter::try_new(&"en".parse().unwrap(), Default::default()).unwrap();
	let spanish = NumberFormatter::try_new(&"es".parse().unwrap(), Default::default()).unwrap();
	let arabic = formatter("ar-EG-u-nu-arab", Precision::preserve());

	assert_eq!(english.format(&value).unwrap(), "1,234.50");
	assert_eq!(spanish.format(&value).unwrap(), "1234,50");
	assert_eq!(arabic.format(&value).unwrap(), "١٢٣٤٫٥٠");
	assert_eq!(english.format(&value).unwrap(), "1,234.50");
}

#[test]
fn explicit_and_paired_calls_agree_and_snapshot_can_be_split() {
	let formatter = formatter("ru", Precision::fixed(1, HALF_EVEN).unwrap());
	let input: Decimal = "1.04".parse().unwrap();
	let number = formatter
		.localize(&input, PluralRuleType::Cardinal)
		.unwrap();
	assert_eq!(number.text(), formatter.format(&input).unwrap());
	assert_eq!(
		number.category(),
		formatter
			.category(&input, PluralRuleType::Cardinal)
			.unwrap()
	);
	assert_eq!(number.selector(), "other");
	assert_eq!(
		number.into_parts(),
		("1,0".to_owned(), PluralCategory::Other)
	);
}

#[test]
fn category_keywords_are_exhaustive_and_round_trip_through_icu() {
	for category in PluralCategory::all() {
		assert_eq!(
			PluralCategory::get_for_cldr_string(crate::plural_keyword(category)),
			Some(category)
		);
	}
}

#[test]
fn formatter_is_shareable_without_an_engine_resource() {
	fn assert_send_sync<T: Send + Sync>() {}

	assert_send_sync::<NumberFormatter>();
}
