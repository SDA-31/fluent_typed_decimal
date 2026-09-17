# localized_numbers

Prepare locale-formatted Decimal text and its matching plural category with
[ICU4X](https://github.com/unicode-org/icu4x). An independent **unpublished prototype**:
no Bevy, Fluent, code generation, filesystem access or global language state in
the library. `fluent-typed` is used only in consumer compatibility tests/examples.

## Minimal use

```rust
use localized_numbers::{Decimal, NumberFormatter, PluralCategory, PluralRuleType};

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
use localized_numbers::{
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

## Fluent is an optional consumer, not the number engine

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

## Lifecycle, precision limits and scope

- Reuse a `NumberFormatter` per locale/options; it retains ICU formatting and both
  cardinal/ordinal rules. There is no hidden unbounded cache.
- `LocalizedNumber` is an owned snapshot. On value, locale, numbering-system or
  precision changes, recompute from the original value. UI movement/fading alone
  does not require formatting again. Deferred bindings must not capture stale text.
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

Run `cargo test -p localized_numbers`, `cargo run -p localized_numbers --example numbers`
or `cargo run -p localized_numbers --example fluent`. Examples and tests are local;
no registry release, repository URL or remote publication is configured yet.
