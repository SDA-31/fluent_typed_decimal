//! Decimal arguments passed directly to upstream fluent-typed generated methods.
use fluent_typed_decimal::{Decimal, NumberFormatter, PluralRuleType};

// Upstream emits unused loading helpers and a manual Default implementation.
// Keep these allowances limited to its generated module, not the adapter.
#[allow(dead_code, clippy::derivable_impls)]
mod texts {
	include!(concat!(env!("OUT_DIR"), "/translations.rs"));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	for (language, locale) in [
		(texts::L10n::En, "en"),
		(texts::L10n::Es, "es"),
		(texts::L10n::Ru, "ru"),
		(texts::L10n::Ar, "ar-EG-u-nu-arab"),
	] {
		let translations = language.load();
		let formatter = NumberFormatter::try_new(&locale.parse()?, Default::default())?;

		for count in [0, 1, 3] {
			// Required application states are selected before presentation rounding.
			let message = if count == 0 {
				translations.msg_empty()
			} else {
				let number = formatter.localize(&Decimal::from(count), PluralRuleType::Cardinal)?;
				translations.msg_remaining(number.selector(), number.text())
			};
			println!("{locale}: {message}");
		}
	}

	Ok(())
}

#[cfg(test)]
mod tests;
