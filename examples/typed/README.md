# Generated fluent-typed Decimal arguments

This minimal consumer uses [fluent-typed](https://docs.rs/fluent-typed/) directly
to generate message accessors and `fluent_typed_decimal` to prepare their arguments.
There is no dependency on the modular generator or an engine.

From the repository root:

```sh
cargo run --manifest-path examples/typed/Cargo.toml
cargo test --manifest-path examples/typed/Cargo.toml
```

The example's explicit `build.rs` reads the shared FTL fixtures in
`examples/assets/localizations/translations/{en,es,ru,ar}/` and writes only to
Cargo `OUT_DIR` under target/. It requires rustfmt, used by upstream generation.
English defines the two `(String)` arguments; translated branches may differ.

`NumberFormatter::localize` returns our `LocalizedNumber`. The generated method
does not accept that type: it takes `number.selector()` for the `plural` argument
and `number.text()` for `value`. Both are borrowed string slices; Fluent matches
the selector literally and assembles the final sentence. Cardinal rules and number
formatting were already applied by ICU to the same prepared Decimal.

The example chooses an empty-state message from the raw count, before formatting.
It pairs each formatter with the corresponding translation language and keeps
Fluent's bidi isolation. It tests Arabic digits, not rendered RTL layout or fonts.
The test compiles actual generated methods and checks all four locales, including
the difference between English `1` and `1.0`.

The local path dependency tests this checkout. In your own application, use
`fluent_typed_decimal = "0.1.0"` and keep fluent-typed in the normal and build
dependency sections as shown in this example's manifest. No extra adapter build
script or generated Decimal parameter type is needed.

The example is MIT-licensed independently of its consuming application.
