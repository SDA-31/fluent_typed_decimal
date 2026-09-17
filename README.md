# fluent_typed_decimal

[![crates.io](https://img.shields.io/crates/v/fluent_typed_decimal)](https://crates.io/crates/fluent_typed_decimal)
[![docs.rs](https://img.shields.io/docsrs/fluent_typed_decimal)](https://docs.rs/fluent_typed_decimal/latest/fluent_typed_decimal/)
[![CI](https://img.shields.io/github/actions/workflow/status/SDA-31/fluent_typed_decimal/ci.yml?branch=main&label=CI&logo=github)](https://github.com/SDA-31/fluent_typed_decimal/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/crates/msrv/fluent_typed_decimal)](https://crates.io/crates/fluent_typed_decimal)
[![License](https://img.shields.io/crates/l/fluent_typed_decimal)](https://github.com/SDA-31/fluent_typed_decimal/blob/main/LICENSE)

Decimal argument support for
[fluent-typed](https://github.com/human-solutions/fluent-typed)
([API documentation](https://docs.rs/fluent-typed)). This crate
prepares localized number text and a matching plural selector for its typed
String parameters. [ICU4X](https://github.com/unicode-org/icu4x) supplies formatting
and grammatical rules; fluent-typed keeps translation loading, typed accessors
and Fluent message resolution.

This is an argument-preparation adapter, not a replacement translation engine,
a new native Fluent numeric type or a general-purpose formatting framework.
The boundary is ordinary strings, so the library does not need a runtime
dependency on fluent-typed. Compatibility is checked with both its generated
accessors and its lower-level runtime. There is no automatic engine integration
or custom generated parameter type. The caller chooses the locale explicitly.

## Install and responsibilities

```toml
[dependencies]
fluent_typed_decimal = "0.1.0"
```

Rust 1.95 or newer is required. The default feature set is empty; enable `float`
only if you need ICU's explicit `f64` to Decimal conversion. Locale data is
compiled into the ICU dependencies; the library does not read files or the OS locale.
Keep the usual fluent-typed dependencies and build script in your application:
this crate prepares arguments, not translations.

| Component | Responsibility |
| --- | --- |
| ICU Decimal / fixed_decimal | Decimal representation, rounding and zero padding |
| ICU DecimalFormatter / PluralRules | Localized number text and language-specific grammatical categories |
| This crate's `NumberFormatter` | One precision policy, reusable ICU instances and checked limits |
| This crate's `LocalizedNumber` | Owned text and category snapshot; it does not contain the original Decimal |
| fluent-typed / Fluent | Generated String accessors, branch matching and sentence assembly |

[API documentation](https://docs.rs/fluent_typed_decimal/latest/fluent_typed_decimal/) ·
[Generated consumer](https://github.com/SDA-31/fluent_typed_decimal/tree/main/examples/typed) ·
[Lower-level consumer](https://github.com/SDA-31/fluent_typed_decimal/blob/main/examples/fluent.rs)

## Minimal use

```rust
use fluent_typed_decimal::{Decimal, NumberFormatter, PluralCategory, PluralRuleType};

let locale = "en".parse()?;
let formatter = NumberFormatter::try_new(&locale, Default::default())?;
let number = formatter.localize(&"1.0".parse::<Decimal>()?, PluralRuleType::Cardinal)?;
assert_eq!(number.text(), "1.0");
assert_eq!(number.category(), PluralCategory::Other);
assert_eq!(number.selector(), "other");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The default preserves input precision: `1` and `1.0` can have different grammar.
Construct integers directly with `Decimal::from`, never through `f64`. Fractional
input can be parsed from an invariant decimal string. With feature `float`, use
`Decimal::try_from_f64` with an explicit `FloatPrecision`; NaN/infinity are errors.
Do not parse already-localized display strings as numeric inputs.

## Precision and separate arguments

```rust
use fluent_typed_decimal::{
    NumberFormatter, NumberOptions, PluralRuleType, Precision,
    RoundingMode, UnsignedRoundingMode, plural_keyword,
};

let mut options = NumberOptions::default();
options.precision = Precision::fixed(
    1, RoundingMode::Unsigned(UnsignedRoundingMode::HalfEven),
)?;
let formatter = NumberFormatter::try_new(&"en".parse()?, options)?;
let input = "0.96".parse()?;

// Explicit calls: both apply the same policy.
let text = formatter.format(&input)?;
let category = formatter.category(&input, PluralRuleType::Cardinal)?;
assert_eq!(text, "1.0");
assert_eq!(plural_keyword(category), "other");

// Preferred when both are needed: preparation happens only once.
let pair = formatter.localize(&input, PluralRuleType::Cardinal)?;
assert_eq!(pair.text(), text);
assert_eq!(pair.category(), category);
# Ok::<(), Box<dyn std::error::Error>>(())
```

`Precision::fraction_range(minimum, maximum, rounding)` rounds to the maximum,
trims trailing zeros and pads to the minimum. The input is never mutated.
Default grouping follows the locale; set `NumberOptions::grouping` to
`GroupingStrategy::Never` to disable it. Use a locale such as `ar-EG-u-nu-arab`
or `ar-EG-u-nu-latn` to select digits without changing the grammar language.
ICU's compiled-data fallback applies; the constructor does not enforce an
application-specific locale allowlist or promise every numbering system exists.

## Passing arguments to fluent-typed

Pass `pair.text()` and `pair.selector()` to two **String** arguments:

```ftl
# $value (String) - Locale-formatted number.
# $plural (String) - CLDR category keyword.
remaining = { $plural ->
    [one] { $value } item remaining
   *[other] { $value } items remaining
}
empty = Nothing remains. Add an item to continue.
```

`(String)` on the selector is important for fluent-typed: plural-looking variants
otherwise infer a numeric parameter. A category string chooses a literal branch;
it does not become a Fluent number and cannot select numeric `[0]` or `[1]`.
Native Fluent numeric selectors remain a separate API. Required application
states use separate messages (`empty`), selected from domain state rather than
rounded display text. Other languages may use different plural branch sets.
Keep units and whole sentences in translations, not in this crate.

For a String selector, Fluent compares literal keys: `"few"` selects `[few]`.
It does not compute another plural category. Unknown or missing keys fall back
to the branch marked `*`; `other` is a conventional category name, not the
fallback mechanism itself. A misspelled `[five]` is an ordinary String key and
will not match `"few"`. String argument typing does not validate the spelling of
category keys. Numeric `[1]` and String `[one]` are not interchangeable.

The checkout's `examples/typed` consumer uses upstream fluent-typed generation
in an explicit `build.rs`. Its accessor call is:

```rust,ignore
let number = formatter.localize(&value, PluralRuleType::Cardinal)?;
let message = translations.msg_remaining(number.selector(), number.text());
```

The generated method has two String parameters: `plural` and `value`. Their
order is determined by fluent-typed, not by this adapter. No custom argument type,
handwritten accessor or modified generator is needed. Use the same language for
the formatter and translations; numbering-system extensions may change digits.

`pair.text()` borrows `&str` from the owned snapshot; `pair.selector()` maps its
typed `PluralCategory` to a static keyword such as `"few"`. Neither getter
formats or clones a string. The generated accessor borrows these arguments for
the call and returns the assembled `String`. `LocalizedNumber` itself is not
passed to Fluent and has no implicit conversion into `FluentNumber`.

The example above is excluded from standalone doctests because its types are
generated from consumer-owned FTL. The repository's typed consumer compiles and
tests that call on every supported CI platform; other self-contained examples
on this page are executable doctests.

## Arabic digits, mixed text and RTL

Number formatting, text direction and UI layout are separate responsibilities.
For example, `ar-EG-u-nu-arab` selects Arabic-Indic digits; `ar-EG-u-nu-latn`
selects Latin digits while keeping Arabic grammatical rules. The adapter does
not reverse strings or choose text alignment from the digit system.

Fluent's default isolation marks protect interpolated text in mixed-direction
sentences. This adapter returns number text without adding those marks; the
translation engine adds them at interpolation. Preserve that behavior unless
your renderer deliberately handles isolation itself. Correct visual bidi order,
Arabic glyph shaping, suitable fonts and mirrored UI layout remain renderer and
application work. Supporting Arabic digits is not a claim of complete RTL UI.

## Lifecycle, precision limits and scope

- Reuse a `NumberFormatter` per locale/options; it retains ICU formatting and both
  cardinal/ordinal rules. There is no hidden unbounded cache.
- `LocalizedNumber` is an owned snapshot. On value, locale, numbering-system or
  precision changes, recompute from the original value. UI movement/fading alone
  does not require formatting again. Deferred bindings must not capture stale text.
  Keep the source Decimal and recompute using the active translation language;
  the adapter cannot detect a mismatch with the catalog or switch locales for you.
- ICU 2.x converts at most 18 fractional digits into plural operands. Selection
  returns a typed error above that **after** precision preparation, even for
  trailing zeros. Explicitly round first, or use text-only `format`, which has no
  additional plural limit. This is not unlimited-precision plural arithmetic.
- Large integer display keeps all Decimal digits; plural selection uses ICU's
  large-integer operand reduction, not a lossy `f64` cast. Leading zero padding is
  removed only for plural operands; visible fractional zeros are preserved.
- Locale parsing and formatting failures are explicit. A rounding carry beyond
  Decimal's maximum magnitude returns an error instead of zero. The adapter does not
  provide currency/units, compact notation, plural ranges, font shaping or RTL
  layout. Renderers/translation engines remain responsible for bidi isolation.
- This package is MIT; it does not change the license of a consuming application.

From this repository's root, run
`cargo run --manifest-path examples/typed/Cargo.toml` for the generated typed
consumer, or `cargo run --example fluent` for the lower-level API.
`cargo run --example numbers` shows argument preparation on its own.
Run `cargo test --all-features` and
`cargo test --manifest-path examples/typed/Cargo.toml` to check both packages.
The typed consumer lives in the repository, not the library's package archive.
It uses a path dependency to exercise this checkout; application installations
use the registry dependency shown above.

## Verification and license

CI checks Linux, Windows and macOS on stable Rust, plus Linux on Rust 1.95.0.
It compiles and tests both the adapter and upstream-generated EN/ES/RU/AR consumer,
including preserved fractional zeros, rounding, large integers, Arabic digits,
cardinal/ordinal categories and Fluent's literal-key/default-branch behavior.
Formatting, Clippy, Rustdoc and package verification run separately. There is no
automatic publication job or registry token in CI.

[MIT](https://github.com/SDA-31/fluent_typed_decimal/blob/main/LICENSE). This covers
the library and its examples, not a consuming application or translation assets
supplied by that application. Dependency licenses remain their own.
