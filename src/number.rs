//! Decimal text and matching grammar prepared for fluent-typed String arguments.
use crate::PluralCategory;

/// A fluent-typed argument pair prepared for one locale and precision policy.
///
/// Pass [`Self::text`] and [`Self::selector`] to the corresponding generated
/// String parameters. This does not implement native Fluent numeric conversion.
///
/// Recompute after value, language, numbering-system or formatting changes.
/// Do not capture this snapshot permanently in a language-switchable UI binding;
/// retain the original numeric input instead. No native Fluent number conversion
/// is provided: a category keyword is not an exact numeric selector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalizedNumber {
	pub(crate) text: String,
	pub(crate) category: PluralCategory,
}

impl LocalizedNumber {
	/// Locale-formatted number, without units or surrounding prose.
	pub fn text(&self) -> &str {
		&self.text
	}

	/// Grammatical category computed from the same prepared Decimal as the text.
	pub fn category(&self) -> PluralCategory {
		self.category
	}

	/// CLDR keyword for a string selector, such as `one` or `few`.
	///
	/// This is not the displayed number, a translated word, or a numeric `[0]` case.
	pub fn selector(&self) -> &'static str {
		plural_keyword(self.category)
	}

	/// Consume this snapshot into its owned text and typed category.
	pub fn into_parts(self) -> (String, PluralCategory) {
		(self.text, self.category)
	}
}

/// Map a typed ICU category to its stable CLDR string-selector keyword.
///
/// This does not select a category from a number or parse localized digits.
pub const fn plural_keyword(category: PluralCategory) -> &'static str {
	match category {
		PluralCategory::Zero => "zero",
		PluralCategory::One => "one",
		PluralCategory::Two => "two",
		PluralCategory::Few => "few",
		PluralCategory::Many => "many",
		PluralCategory::Other => "other",
	}
}
