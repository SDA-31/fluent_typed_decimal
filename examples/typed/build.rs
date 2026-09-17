//! Generate the example's typed accessors upstream; all output stays in target/.
use fluent_typed::{BuildOptions, FtlOutputOptions, LintLevel};
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let output = PathBuf::from(env::var_os("OUT_DIR").ok_or("Cargo must set OUT_DIR")?);
	let options = BuildOptions::default()
		.with_locales_folder("../assets/localizations/translations")
		.with_output_file_path(&output.join("translations.rs").to_string_lossy())
		.with_ftl_output(FtlOutputOptions::single_file(
			&output.join("translations.ftl").to_string_lossy(),
		))
		.with_lint_level(LintLevel::Strict);

	fluent_typed::try_build_from_locales_folder(options)?;

	Ok(())
}
