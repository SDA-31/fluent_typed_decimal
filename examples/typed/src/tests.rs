//! Compile and exercise the actual generated String signatures for every locale.
use super::texts;
use fluent_typed_decimal::{NumberFormatter, PluralRuleType};

#[test]
fn generated_accessors_accept_decimal_strings_with_language_specific_branches() {
	for (language, locale, input, expected) in [
		(
			texts::L10n::En,
			"en",
			"1",
			"\u{2068}1\u{2069} item remaining",
		),
		(
			texts::L10n::En,
			"en",
			"1.0",
			"\u{2068}1.0\u{2069} items remaining",
		),
		(
			texts::L10n::Es,
			"es",
			"2",
			"Quedan \u{2068}2\u{2069} elementos",
		),
		(
			texts::L10n::Ru,
			"ru",
			"2",
			"Осталось \u{2068}2\u{2069} предмета",
		),
		(
			texts::L10n::Ar,
			"ar-EG-u-nu-arab",
			"3",
			"تبقت \u{2068}٣\u{2069} عناصر",
		),
	] {
		let translations = language.load();
		let formatter =
			NumberFormatter::try_new(&locale.parse().unwrap(), Default::default()).unwrap();
		let number = formatter
			.localize(&input.parse().unwrap(), PluralRuleType::Cardinal)
			.unwrap();

		assert_eq!(
			translations.msg_remaining(number.selector(), number.text()),
			expected
		);
		assert!(!translations.msg_empty().is_empty());
	}
}
