//! Inspect the Decimal arguments that will be passed to fluent-typed.
use fluent_typed_decimal::{Decimal, NumberFormatter, PluralRuleType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let value: Decimal = "12.50".parse()?;

	for language in ["en", "es", "ru", "ar-EG-u-nu-arab"] {
		let formatter = NumberFormatter::try_new(&language.parse()?, Default::default())?;
		let number = formatter.localize(&value, PluralRuleType::Cardinal)?;
		println!("{language}: {} ({})", number.text(), number.selector());
	}

	Ok(())
}
