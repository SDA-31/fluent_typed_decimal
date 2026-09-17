//! Large integers and the boundary where ICU would silently truncate a fraction.
use super::*;
use crate::PrecisionError;

#[test]
fn all_u64_digits_survive_without_float_conversion() {
	let formatter = NumberFormatter::try_new(&"en".parse().unwrap(), Default::default()).unwrap();
	let value = Decimal::from(u64::MAX);
	let number = formatter
		.localize(&value, PluralRuleType::Cardinal)
		.unwrap();

	assert_eq!(number.text(), "18,446,744,073,709,551,615");
	assert_eq!(number.category(), PluralCategory::Other);
}

#[test]
fn large_integer_suffixes_do_not_become_exactly_one_or_zero() {
	let english = formatter("en", Precision::preserve());
	let russian = formatter("ru", Precision::preserve());
	let arabic = formatter("ar", Precision::preserve());

	for (value, ru_category) in [
		("1000000000000000000", PluralCategory::Many),
		("1000000000000000001", PluralCategory::One),
		("1000000000000000002", PluralCategory::Few),
	] {
		let input: Decimal = value.parse().unwrap();
		let number = english.localize(&input, PluralRuleType::Cardinal).unwrap();
		assert_eq!(number.text(), value);
		assert_eq!(number.category(), PluralCategory::Other);
		assert_eq!(
			russian.category(&input, PluralRuleType::Cardinal).unwrap(),
			ru_category
		);
		assert_eq!(
			arabic.category(&input, PluralRuleType::Cardinal).unwrap(),
			PluralCategory::Other
		);
	}
}

#[test]
fn leading_padding_is_not_a_large_integer_operand() {
	let formatter = formatter("en", Precision::preserve());

	for input in ["000000000000000000001", "000000000000000000001.0"] {
		let number = formatter
			.localize(&input.parse().unwrap(), PluralRuleType::Cardinal)
			.unwrap();
		assert_eq!(number.text(), input);
		assert_eq!(
			number.category(),
			if input.ends_with(".0") {
				PluralCategory::Other
			} else {
				PluralCategory::One
			}
		);
	}
}

#[test]
fn long_fractions_error_instead_of_losing_significant_digits_or_visible_zeros() {
	let formatter = formatter("en", Precision::preserve());

	for input in ["1.0000000000000000001", "1.0000000000000000000"] {
		let value: Decimal = input.parse().unwrap();
		assert_eq!(formatter.format(&value).unwrap(), input);
		assert_eq!(
			formatter.localize(&value, PluralRuleType::Cardinal),
			Err(PrecisionError::PluralFractionTooLong {
				digits: 19,
				maximum: 18
			})
		);
		assert_eq!(
			formatter.category(&value, PluralRuleType::Ordinal),
			Err(PrecisionError::PluralFractionTooLong {
				digits: 19,
				maximum: 18
			})
		);
	}
}

#[test]
fn eighteen_fraction_digits_work_and_explicit_rounding_can_reduce_longer_inputs() {
	let formatter = formatter("en", Precision::preserve());
	let value: Decimal = "1.000000000000000001".parse().unwrap();
	let number = formatter
		.localize(&value, PluralRuleType::Cardinal)
		.unwrap();
	assert_eq!(number.text(), "1.000000000000000001");
	assert_eq!(number.category(), PluralCategory::Other);

	let rounded = super::formatter("en", Precision::fixed(0, HALF_EVEN).unwrap());
	let value = "1.0000000000000000001".parse().unwrap();
	let number = rounded.localize(&value, PluralRuleType::Cardinal).unwrap();
	assert_eq!(number.text(), "1");
	assert_eq!(number.category(), PluralCategory::One);
}

#[test]
fn added_visible_zeros_are_checked_after_precision_preparation() {
	let formatter = formatter("en", Precision::fixed(19, HALF_EVEN).unwrap());
	assert_eq!(
		formatter.format(&Decimal::from(1)).unwrap(),
		"1.0000000000000000000"
	);
	assert!(matches!(
		formatter.localize(&Decimal::from(1), PluralRuleType::Cardinal),
		Err(PrecisionError::PluralFractionTooLong { .. })
	));
}

#[test]
fn rounding_overflow_is_an_error_not_a_zero_value() {
	let formatter = formatter("en", Precision::fixed(0, HALF_EVEN).unwrap());
	let source = format!("{}.5", "9".repeat(i16::MAX as usize + 1));
	let input: Decimal = source.parse().unwrap();

	assert_eq!(
		formatter.format(&input),
		Err(PrecisionError::RoundingOverflow)
	);
	assert_eq!(
		formatter.localize(&input, PluralRuleType::Cardinal),
		Err(PrecisionError::RoundingOverflow)
	);
	assert_eq!(input.to_string(), source);
}
