//! Prepared precision, rather than an unrounded input, determines grammar.
use super::*;

#[test]
fn rounding_and_visible_zeros_affect_both_text_and_plural() {
	let input: Decimal = "0.96".parse().unwrap();
	let integer = formatter("en", Precision::fixed(0, HALF_EVEN).unwrap());
	let fractional = formatter("en", Precision::fixed(1, HALF_EVEN).unwrap());
	let a = integer.localize(&input, PluralRuleType::Cardinal).unwrap();
	let b = fractional
		.localize(&input, PluralRuleType::Cardinal)
		.unwrap();

	assert_eq!((a.text(), a.category()), ("1", PluralCategory::One));
	assert_eq!((b.text(), b.category()), ("1.0", PluralCategory::Other));
	assert_eq!(input.to_string(), "0.96");
}

#[test]
fn fraction_range_trims_then_pads_to_minimum() {
	let formatter = formatter("en", Precision::fraction_range(1, 3, HALF_EVEN).unwrap());

	for (input, expected) in [
		("1", "1.0"),
		("1.2000", "1.2"),
		("1.2346", "1.235"),
		("1.9999", "2.0"),
	] {
		assert_eq!(formatter.format(&input.parse().unwrap()).unwrap(), expected);
	}
}

#[test]
fn half_even_ceiling_floor_and_negative_values_follow_explicit_policy() {
	for (mode, cases) in [
		(HALF_EVEN, [("2.5", "2"), ("3.5", "4"), ("-2.5", "-2")]),
		(
			RoundingMode::Ceil,
			[("1.1", "2"), ("-1.1", "-1"), ("-1.9", "-1")],
		),
		(
			RoundingMode::Floor,
			[("1.1", "1"), ("-1.1", "-2"), ("-1.9", "-2")],
		),
	] {
		let formatter = formatter("en", Precision::fixed(0, mode).unwrap());

		for (input, expected) in cases {
			assert_eq!(formatter.format(&input.parse().unwrap()).unwrap(), expected);
		}
	}
}

#[test]
fn preserve_keeps_negative_zero_and_fractional_padding() {
	let formatter = formatter("en", Precision::preserve());
	let number = formatter
		.localize(&"-0.00".parse().unwrap(), PluralRuleType::Cardinal)
		.unwrap();

	assert_eq!(number.text(), "-0.00");
	assert_eq!(number.category(), PluralCategory::Other);
}

#[test]
fn rounded_zero_does_not_mutate_domain_state() {
	let input: Decimal = "0.04".parse().unwrap();
	let formatter = formatter("en", Precision::fixed(1, HALF_EVEN).unwrap());

	assert_eq!(formatter.format(&input).unwrap(), "0.0");
	assert!(!input.is_zero());
}

#[test]
fn invalid_precision_is_reported_before_constructing_a_formatter() {
	assert!(matches!(
		Precision::fraction_range(3, 2, HALF_EVEN),
		Err(crate::PrecisionError::InvalidRange {
			minimum: 3,
			maximum: 2
		})
	));
	assert!(matches!(
		Precision::fixed(u16::MAX, HALF_EVEN),
		Err(crate::PrecisionError::MagnitudeOutOfRange { .. })
	));
}

#[cfg(feature = "float")]
#[test]
fn float_conversion_is_explicit_and_rejects_non_finite_values() {
	use crate::FloatPrecision;
	let value = Decimal::try_from_f64(12.5, FloatPrecision::Magnitude(-2)).unwrap();
	let formatter = formatter("en", Precision::preserve());

	assert_eq!(formatter.format(&value).unwrap(), "12.50");

	for input in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
		assert!(Decimal::try_from_f64(input, FloatPrecision::Magnitude(-2)).is_err());
	}
}
