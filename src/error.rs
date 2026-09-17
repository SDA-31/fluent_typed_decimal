//! Checked limits for numeric preparation and plural operands.
use std::fmt;

/// Invalid requested precision or an unsupported prepared plural operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PrecisionError {
	/// Rounding would carry past ICU Decimal's maximum integer magnitude.
	RoundingOverflow,
	/// The minimum visible fraction length exceeds the maximum.
	InvalidRange {
		/// Requested minimum number of fraction digits.
		minimum: u16,
		/// Requested maximum number of fraction digits.
		maximum: u16,
	},
	/// Fraction precision exceeds the supported signed magnitude range.
	MagnitudeOutOfRange {
		/// Requested number of fraction digits.
		digits: u16,
	},
	/// ICU plural conversion would truncate the prepared visible fraction.
	PluralFractionTooLong {
		/// Number of visible fraction digits after preparation.
		digits: u16,
		/// Maximum number supported by this adapter's ICU operand conversion.
		maximum: u16,
	},
}

impl fmt::Display for PrecisionError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::RoundingOverflow => {
				f.write_str("rounding exceeds the supported Decimal magnitude")
			}
			Self::InvalidRange { minimum, maximum } => write!(
				f,
				"minimum fraction digits ({minimum}) exceed maximum ({maximum})"
			),
			Self::MagnitudeOutOfRange { digits } => write!(
				f,
				"fraction precision {digits} exceeds the supported maximum {}",
				i16::MAX
			),
			Self::PluralFractionTooLong { digits, maximum } => write!(
				f,
				"plural selection supports at most {maximum} visible fraction digits, got {digits}; round explicitly or format text without a plural category"
			),
		}
	}
}

impl std::error::Error for PrecisionError {}
