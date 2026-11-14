**symphonia > default**

# Module: default

## Contents

**Modules**

- [`codecs`](#codecs) - The `codecs` module re-exports all enabled Symphonia decoders.
- [`formats`](#formats) - The `formats` module re-exports all enabled Symphonia format readers.
- [`meta`](#meta) - The `meta` module re-exports all enabled Symphonia metadata readers.

**Functions**

- [`get_codecs`](#get_codecs) - Gets the default `CodecRegistry`. This registry pre-registers all the codecs selected by the
- [`get_probe`](#get_probe) - Gets the default `Probe`. This registry pre-registers all the formats selected by the
- [`register_enabled_codecs`](#register_enabled_codecs) - Registers all the codecs selected by the `feature` flags in the includer's `Cargo.toml` on
- [`register_enabled_formats`](#register_enabled_formats) - Registers all the formats selected by the `feature` flags in the includer's `Cargo.toml` on

---

## Module: codecs

The `codecs` module re-exports all enabled Symphonia decoders.



## Module: formats

The `formats` module re-exports all enabled Symphonia format readers.



## symphonia::default::get_codecs

*Function*

Gets the default `CodecRegistry`. This registry pre-registers all the codecs selected by the
`feature` flags in the includer's `Cargo.toml`. If `features` is not set, the default set of
Symphonia codecs is registered.

This function is lazy and does not instantiate the `CodecRegistry` until the first call to
this function.

```rust
fn get_codecs() -> &'static symphonia_core::codecs::registry::CodecRegistry
```



## symphonia::default::get_probe

*Function*

Gets the default `Probe`. This registry pre-registers all the formats selected by the
`feature` flags in the includer's `Cargo.toml`. If `features` is not set, the default set of
Symphonia formats is registered.

This function is lazy and does not instantiate the `Probe` until the first call to this
function.

```rust
fn get_probe() -> &'static symphonia_core::formats::probe::Probe
```



## Module: meta

The `meta` module re-exports all enabled Symphonia metadata readers.



## symphonia::default::register_enabled_codecs

*Function*

Registers all the codecs selected by the `feature` flags in the includer's `Cargo.toml` on
the provided `CodecRegistry`. If `features` is not set, the default set of Symphonia codecs
is registered.

Use this function to easily populate a custom registry with all enabled codecs.

```rust
fn register_enabled_codecs(registry: & mut symphonia_core::codecs::registry::CodecRegistry)
```



## symphonia::default::register_enabled_formats

*Function*

Registers all the formats selected by the `feature` flags in the includer's `Cargo.toml` on
the provided `Probe`. If `features` is not set, the default set of Symphonia formats is
registered.

Use this function to easily populate a custom probe with all enabled formats.

```rust
fn register_enabled_formats(probe: & mut symphonia_core::formats::probe::Probe)
```



