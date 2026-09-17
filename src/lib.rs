#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod error;
mod formatter;
mod number;
mod precision;

pub use error::PrecisionError;
pub use formatter::{NumberFormatter, NumberOptions};
pub use number::{LocalizedNumber, plural_keyword};
pub use precision::Precision;

/// ICU4X's signed rounding modes, including ceiling and floor.
pub use fixed_decimal::SignedRoundingMode as RoundingMode;
/// Sign-independent rounding modes, wrapped by [`RoundingMode::Unsigned`].
pub use fixed_decimal::UnsignedRoundingMode;
/// The ICU4X decimal input; not `rust_decimal::Decimal`.
pub use icu_decimal::input::Decimal;
/// ICU4X's explicit float conversion precision, enabled by feature `float`.
#[cfg(feature = "float")]
pub use icu_decimal::input::FloatPrecision;
/// ICU4X's locale-sensitive grouping policy.
pub use icu_decimal::options::GroupingStrategy;
/// Locale and optional numbering-system preference, such as `ar-EG-u-nu-arab`.
pub use icu_locale_core::Locale;
/// ICU4X's six grammatical categories; these are not exact-number cases.
pub use icu_plurals::PluralCategory;
/// Select cardinal (quantity) or ordinal (position) rules.
pub use icu_plurals::PluralRuleType;
/// ICU data-loading failure returned while constructing a formatter.
pub use icu_provider::DataError;

#[cfg(test)]
mod tests;
