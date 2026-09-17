//! An ordinary Fluent consumer: the number library has no Fluent runtime dependency.
use fluent_typed::prelude::{FluentArgs, L10nBundle};
use localized_numbers::{Decimal, NumberFormatter, PluralRuleType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	for (language, locale, source) in [
		(
			"en",
			"en",
			include_str!("assets/localizations/translations/en/numbers.ftl"),
		),
		(
			"es",
			"es",
			include_str!("assets/localizations/translations/es/numbers.ftl"),
		),
		(
			"ru",
			"ru",
			include_str!("assets/localizations/translations/ru/numbers.ftl"),
		),
		(
			"ar",
			"ar-EG-u-nu-arab",
			include_str!("assets/localizations/translations/ar/numbers.ftl"),
		),
	] {
		let formatter = NumberFormatter::try_new(&locale.parse()?, Default::default())?;
		let bundle = L10nBundle::new(language, source.as_bytes())?;

		for count in [0, 1, 3] {
			// Application state chooses an explicit message, not a plural branch.
			let message = if count == 0 {
				bundle.msg("empty", None)?
			} else {
				let number = formatter.localize(&Decimal::from(count), PluralRuleType::Cardinal)?;
				let mut args = FluentArgs::new();
				args.set("value", number.text());
				args.set("plural", number.selector());
				bundle.msg("remaining", Some(args))?
			};
			println!("{language}: {message}");
		}
	}

	Ok(())
}
