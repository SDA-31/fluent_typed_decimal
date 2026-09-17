//! Apply one precision policy before either formatting or grammatical selection.
use crate::{Decimal, PrecisionError, RoundingMode, UnsignedRoundingMode};
use std::borrow::Cow;

/// Validated fraction precision shared by display text and plural selection.
///
/// The default preserves the input, including visible trailing zeros. Fraction
/// policies round, remove trailing zeros, then pad to their required minimum.
/// This type does not change the caller's Decimal or infer game state.
/// Both number text and plural category use the prepared value: rounding `1.2`
/// to `1` can change grammar. Required application states must still be selected
/// from domain data, not from rounded presentation text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Precision {
	fraction: Option<(i16, i16)>,
	rounding: RoundingMode,
}

impl Precision {
	/// Preserve the input's value and visible precision without rounding.
	/// This keeps `1` distinct from `1.0` for languages whose plural rules depend
	/// on visible fraction digits. It is also the default policy.
	pub const fn preserve() -> Self {
		Self {
			fraction: None,
			rounding: RoundingMode::Unsigned(UnsignedRoundingMode::HalfEven),
		}
	}

	/// Round and show exactly `digits` fraction digits, including trailing zeros.
	///
	/// # Errors
	/// Returns [`PrecisionError::MagnitudeOutOfRange`] above `i16::MAX` digits.
	/// Plural selection has an additional, checked 18-fraction-digit limit.
	pub fn fixed(digits: u16, rounding: RoundingMode) -> Result<Self, PrecisionError> {
		Self::fraction_range(digits, digits, rounding)
	}

	/// Round to `maximum` fraction digits and show at least `minimum` digits.
	///
	/// # Errors
	/// Rejects an inverted range or a maximum greater than `i16::MAX`. This
	/// validates formatting precision; plural operand limits are checked later.
	pub fn fraction_range(
		minimum: u16,
		maximum: u16,
		rounding: RoundingMode,
	) -> Result<Self, PrecisionError> {
		if minimum > maximum {
			return Err(PrecisionError::InvalidRange { minimum, maximum });
		}

		let maximum = i16::try_from(maximum)
			.map_err(|_| PrecisionError::MagnitudeOutOfRange { digits: maximum })?;

		Ok(Self {
			fraction: Some((minimum as i16, maximum)),
			rounding,
		})
	}

	pub(crate) fn prepare(self, value: &Decimal) -> Result<Cow<'_, Decimal>, PrecisionError> {
		let Some((minimum, maximum)) = self.fraction else {
			return Ok(Cow::Borrowed(value));
		};

		let mut prepared = value.clone();
		prepared.round_with_mode(-maximum, self.rounding);

		// Upstream signals rounding overflow by clearing the significant digits.
		// Fraction rounding cannot turn a nonzero integer part into zero normally.
		if prepared.is_zero() && !value.is_zero() && value.nonzero_magnitude_start() >= 0 {
			return Err(PrecisionError::RoundingOverflow);
		}

		prepared.trim_end();
		prepared.pad_end(-minimum);

		Ok(Cow::Owned(prepared))
	}
}

impl Default for Precision {
	fn default() -> Self {
		Self::preserve()
	}
}
