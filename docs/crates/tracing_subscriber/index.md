# tracing_subscriber

Utilities for implementing and composing [`tracing`] subscribers.

[`tracing`] is a framework for instrumenting Rust programs to collect
scoped, structured, and async-aware diagnostics. The [`Subscriber`] trait
represents the functionality necessary to collect this trace data. This
crate contains tools for composing subscribers out of smaller units of
behaviour, and batteries-included implementations of common subscriber
functionality.

`tracing-subscriber` is intended for use by both `Subscriber` authors and
application authors using `tracing` to instrument their applications.

*Compiler support: [requires `rustc` 1.65+][msrv]*

[msrv]: #supported-rust-versions

## `Layer`s and `Filter`s

The most important component of the `tracing-subscriber` API is the
[`Layer`] trait, which provides a composable abstraction for building
[`Subscriber`]s. Like the [`Subscriber`] trait, a [`Layer`] defines a
particular behavior for collecting trace data. Unlike [`Subscriber`]s,
which implement a *complete* strategy for how trace data is collected,
[`Layer`]s provide *modular* implementations of specific behaviors.
Therefore, they can be [composed together] to form a [`Subscriber`] which is
capable of recording traces in a variety of ways. See the [`layer` module's
documentation][layer] for details on using [`Layer`]s.

In addition, the [`Filter`] trait defines an interface for filtering what
spans and events are recorded by a particular layer. This allows different
[`Layer`]s to handle separate subsets of the trace data emitted by a
program. See the [documentation on per-layer filtering][plf] for more
information on using [`Filter`]s.

[`Layer`]: crate::layer::Layer
[composed together]: crate::layer#composing-layers
[layer]: crate::layer
[`Filter`]: crate::layer::Filter
[plf]: crate::layer#per-layer-filtering

## Included Subscribers

The following `Subscriber`s are provided for application authors:

- [`fmt`] - Formats and logs tracing data (requires the `fmt` feature flag)

## Feature Flags

- `std`: Enables APIs that depend on the Rust standard library
  (enabled by default).
- `alloc`: Depend on [`liballoc`] (enabled by "std").
- `env-filter`: Enables the [`EnvFilter`] type, which implements filtering
  similar to the [`env_logger` crate]. **Requires "std"**.
- `fmt`: Enables the [`fmt`] module, which provides a subscriber
  implementation for printing formatted representations of trace events.
  Enabled by default. **Requires "registry" and "std"**.
- `ansi`: Enables `fmt` support for ANSI terminal colors. Enabled by
  default.
- `registry`: enables the [`registry`] module. Enabled by default.
  **Requires "std"**.
- `json`: Enables `fmt` support for JSON output. In JSON output, the ANSI
  feature does nothing. **Requires "fmt" and "std"**.
- `local-time`: Enables local time formatting when using the [`time`
  crate]'s timestamp formatters with the `fmt` subscriber.

[`registry`]: mod@registry

### Optional Dependencies

- [`tracing-log`]: Enables better formatting for events emitted by `log`
  macros in the `fmt` subscriber. Enabled by default.
- [`time`][`time` crate]: Enables support for using the [`time` crate] for timestamp
  formatting in the `fmt` subscriber.
- [`smallvec`]: Causes the `EnvFilter` type to use the `smallvec` crate (rather
  than `Vec`) as a performance optimization. Enabled by default.
- [`parking_lot`]: Use the `parking_lot` crate's `RwLock` implementation
  rather than the Rust standard library's implementation.

### `no_std` Support

In embedded systems and other bare-metal applications, `tracing` can be
used without requiring the Rust standard library, although some features are
disabled. Although most of the APIs provided by `tracing-subscriber`, such
as [`fmt`] and [`EnvFilter`], require the standard library, some
functionality, such as the [`Layer`] trait, can still be used in
`no_std` environments.

The dependency on the standard library is controlled by two crate feature
flags, "std", which enables the dependency on [`libstd`], and "alloc", which
enables the dependency on [`liballoc`] (and is enabled by the "std"
feature). These features are enabled by default, but `no_std` users can
disable them using:

```toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false }
```

Additional APIs are available when [`liballoc`] is available. To enable
`liballoc` but not `std`, use:

```toml
# Cargo.toml
tracing-subscriber = { version = "0.3", default-features = false, features = ["alloc"] }
```

### Unstable Features

These feature flags enable **unstable** features. The public API may break in 0.1.x
releases. To enable these features, the `--cfg tracing_unstable` must be passed to
`rustc` when compiling.

The following unstable feature flags are currently available:

* `valuable`: Enables support for serializing values recorded using the
  [`valuable`] crate as structured JSON in the [`format::Json`] formatter.

#### Enabling Unstable Features

The easiest way to set the `tracing_unstable` cfg is to use the `RUSTFLAGS`
env variable when running `cargo` commands:

```shell
RUSTFLAGS="--cfg tracing_unstable" cargo build
```
Alternatively, the following can be added to the `.cargo/config` file in a
project to automatically enable the cfg flag for that project:

```toml
[build]
rustflags = ["--cfg", "tracing_unstable"]
```

[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
[`valuable`]: https://crates.io/crates/valuable
[`format::Json`]: crate::fmt::format::Json

## Supported Rust Versions

Tracing is built against the latest stable release. The minimum supported
version is 1.65. The current Tracing version is not guaranteed to build on
Rust versions earlier than the minimum supported version.

Tracing follows the same compiler support policies as the rest of the Tokio
project. The current stable Rust compiler and the three most recent minor
versions before it will always be supported. For example, if the current
stable compiler version is 1.69, the minimum supported version will not be
increased past 1.66, three minor versions prior. Increasing the minimum
supported compiler version is not considered a semver breaking change as
long as doing so complies with this policy.

[`Subscriber`]: tracing_core::subscriber::Subscriber
[`tracing`]: https://docs.rs/tracing/latest/tracing
[`EnvFilter`]: filter::EnvFilter
[`fmt`]: mod@fmt
[`tracing`]: https://crates.io/crates/tracing
[`tracing-log`]: https://crates.io/crates/tracing-log
[`smallvec`]: https://crates.io/crates/smallvec
[`env_logger` crate]: https://crates.io/crates/env_logger
[`parking_lot`]: https://crates.io/crates/parking_lot
[`time` crate]: https://crates.io/crates/time
[`liballoc`]: https://doc.rust-lang.org/alloc/index.html
[`libstd`]: https://doc.rust-lang.org/std/index.html

## Modules

### [`tracing_subscriber`](tracing_subscriber.md)

*1 function, 8 modules*

### [`field`](field.md)

*3 modules, 7 traits*

### [`field::debug`](field/debug.md)

*1 struct*

### [`field::delimited`](field/delimited.md)

*2 structs*

### [`field::display`](field/display.md)

*1 struct*

### [`filter`](filter.md)

*1 module*

### [`filter::directive`](filter/directive.md)

*1 struct*

### [`filter::filter_fn`](filter/filter_fn.md)

*2 functions, 2 structs*

### [`filter::filter_fn::DynFilterFn`](filter/filter_fn/DynFilterFn.md)

*1 function*

### [`filter::filter_fn::FilterFn`](filter/filter_fn/FilterFn.md)

*1 function*

### [`filter::layer_filters`](filter/layer_filters.md)

*1 module, 1 trait, 2 structs*

### [`filter::layer_filters::combinator`](filter/layer_filters/combinator.md)

*3 structs*

### [`filter::targets`](filter/targets.md)

*3 structs*

### [`filter::targets::Targets`](filter/targets/Targets.md)

*4 functions*

### [`fmt`](fmt.md)

*1 type alias, 2 structs, 3 modules, 4 functions*

### [`fmt::SubscriberBuilder`](fmt/SubscriberBuilder.md)

*6 functions*

### [`fmt::fmt_layer`](fmt/fmt_layer.md)

*3 structs*

### [`fmt::fmt_layer::FmtContext`](fmt/fmt_layer/FmtContext.md)

*1 function*

### [`fmt::fmt_layer::Layer`](fmt/fmt_layer/Layer.md)

*4 functions*

### [`fmt::format`](fmt/format.md)

*2 functions, 2 traits, 9 structs*

### [`fmt::format::Format`](fmt/format/Format.md)

*3 functions*

### [`fmt::format::Writer`](fmt/format/Writer.md)

*1 function*

### [`fmt::format::pretty`](fmt/format/pretty.md)

*3 structs*

### [`fmt::time`](fmt/time.md)

*1 trait, 2 functions, 2 structs*

### [`fmt::writer`](fmt/writer.md)

*1 enum, 1 type alias, 2 traits, 8 structs*

### [`fmt::writer::EitherWriter`](fmt/writer/EitherWriter.md)

*2 functions*

### [`layer`](layer.md)

*1 struct, 3 traits*

### [`layer::context`](layer/context.md)

*1 struct*

### [`layer::context::Context`](layer/context/Context.md)

*6 functions*

### [`layer::layered`](layer/layered.md)

*1 struct*

### [`registry`](registry.md)

*2 traits, 3 structs*

### [`registry::Scope`](registry/Scope.md)

*1 function*

### [`registry::SpanRef`](registry/SpanRef.md)

*1 function*

### [`registry::extensions`](registry/extensions.md)

*2 structs*

### [`registry::sharded`](registry/sharded.md)

*2 structs*

### [`reload`](reload.md)

*3 structs*

### [`reload::Handle`](reload/Handle.md)

*2 functions*

### [`sealed`](sealed.md)

*1 trait*

### [`util`](util.md)

*1 struct, 1 trait*

