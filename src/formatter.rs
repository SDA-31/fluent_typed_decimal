//! ICU-backed Decimal argument preparation for fluent-typed String parameters.
use crate::{
	DataError, Decimal, GroupingStrategy, Locale, LocalizedNumber, PluralCategory, PluralRuleType,
	Precision, PrecisionError,
};
use icu_decimal::DecimalFormatter;
use icu_plurals::{PluralOperands, PluralRules};

/// ICU 2.x Decimal-to-plural conversion reads at most this many fraction digits.
/// Reject longer prepared fractions instead of silently changing their meaning.
const MAX_PLURAL_FRACTION_DIGITS: u16 = 18;

/// Immutable presentation policy applied by a [`NumberFormatter`].
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct NumberOptions {
	/// Rounding and visible fraction digits; defaults to preserving the input.
	pub precision: Precision,
	/// Locale-sensitive grouping; defaults to [`GroupingStrategy::Auto`].
	pub grouping: GroupingStrategy,
}

/// Prepare Decimal text and grammatical selectors for fluent-typed arguments.
///
/// Construct once per locale/options and reuse across values. There is no global
/// language, string cache, file watcher, Fluent dependency or Bevy resource here.
/// Locale fallback and supported numbering systems follow ICU's compiled data.
/// An extension like `ar-u-nu-latn` changes digits, not Arabic grammar.
/// Pass the text and selector to separate String parameters of a generated
/// fluent-typed accessor; message loading and resolution remain upstream.
#[derive(Debug)]
pub struct NumberFormatter {
	decimal: DecimalFormatter,
	cardinal: PluralRules,
	ordinal: PluralRules,
	options: NumberOptions,
}

impl NumberFormatter {
	/// Load reusable ICU services using the supplied locale and compiled data.
	///
	/// # Errors
	/// Returns ICU's [`DataError`] if required formatter or rule data cannot load.
	/// Locale parsing is explicit and belongs to the caller; this never reads the OS.
	pub fn try_new(locale: &Locale, options: NumberOptions) -> Result<Self, DataError> {
		Ok(Self {
			decimal: DecimalFormatter::try_new(locale.into(), options.grouping.into())?,
			cardinal: PluralRules::try_new_cardinal(locale.into())?,
			ordinal: PluralRules::try_new_ordinal(locale.into())?,
			options,
		})
	}

	/// Format localized text only, without the plural operand precision limit.
	///
	/// Units and prose belong to the caller's translation system. The original
	/// Decimal is unchanged; this applies the formatter's precision policy.
	///
	/// # Errors
	/// Returns [`PrecisionError::RoundingOverflow`] if rounding exceeds Decimal's
	/// representable magnitude. No additional plural precision limit applies.
	pub fn format(&self, value: &Decimal) -> Result<String, PrecisionError> {
		let prepared = self.options.precision.prepare(value)?;

		Ok(self.decimal.format_to_string(&prepared))
	}

	/// Select grammar using the same precision policy as [`Self::format`].
	///
	/// For paired text/category output, prefer [`Self::localize`] to prepare once.
	///
	/// # Errors
	/// Rejects more than 18 visible fraction digits after preparation, including
	/// trailing zeros, because ICU's operand conversion would truncate them.
	/// Also rejects rounding overflow in the prepared Decimal.
	pub fn category(
		&self,
		value: &Decimal,
		kind: PluralRuleType,
	) -> Result<PluralCategory, PrecisionError> {
		let prepared = self.options.precision.prepare(value)?;

		self.category_prepared(&prepared, kind)
	}

	/// Prepare once, then produce a fluent-typed argument pair: text and selector.
	///
	/// # Errors
	/// Returns [`PrecisionError::PluralFractionTooLong`] instead of silently
	/// truncating a prepared fraction longer than 18 digits. No pair is returned
	/// on failure; [`Self::format`] remains available for text-only output.
	/// Also returns [`PrecisionError::RoundingOverflow`] for magnitude overflow.
	pub fn localize(
		&self,
		value: &Decimal,
		kind: PluralRuleType,
	) -> Result<LocalizedNumber, PrecisionError> {
		let prepared = self.options.precision.prepare(value)?;
		let category = self.category_prepared(&prepared, kind)?;
		let text = self.decimal.format_to_string(&prepared);

		Ok(LocalizedNumber { text, category })
	}

	fn category_prepared(
		&self,
		value: &Decimal,
		kind: PluralRuleType,
	) -> Result<PluralCategory, PrecisionError> {
		let digits = value.magnitude_range().start().unsigned_abs();

		if digits > MAX_PLURAL_FRACTION_DIGITS {
			return Err(PrecisionError::PluralFractionTooLong {
				digits,
				maximum: MAX_PLURAL_FRACTION_DIGITS,
			});
		}

		let rules = match kind {
			PluralRuleType::Ordinal => &self.ordinal,
			_ => &self.cardinal,
		};

		// Leading padding is presentation, not magnitude. ICU's large-integer
		// operand marker uses the visible upper magnitude, so remove only padding
		// before selection; keep trailing zeros because they affect grammar.
		let operands = if *value.magnitude_range().end() > value.nonzero_magnitude_start().max(0) {
			let mut unpadded = value.clone();
			unpadded.trim_start();
			PluralOperands::from(&unpadded)
		} else {
			PluralOperands::from(value)
		};

		Ok(rules.category_for(operands))
	}
}
