# digest

This crate provides traits which describe functionality of cryptographic hash
functions and Message Authentication algorithms.

Traits in this repository are organized into the following levels:

- **High-level convenience traits**: [`Digest`], [`DynDigest`], [`Mac`].
  Wrappers around lower-level traits for most common use-cases. Users should
  usually prefer using these traits.
- **Mid-level traits**: [`Update`], [`FixedOutput`], [`FixedOutputReset`],
  [`ExtendableOutput`], [`ExtendableOutputReset`], [`XofReader`],
  [`VariableOutput`], [`Reset`], [`KeyInit`], and [`InnerInit`]. These
  traits atomically describe available functionality of an algorithm.
- **Marker traits**: [`HashMarker`], [`MacMarker`]. Used to distinguish
  different algorithm classes.
- **Low-level traits** defined in the [`core_api`] module. These traits
  operate at a block-level and do not contain any built-in buffering.
  They are intended to be implemented by low-level algorithm providers only.
  Usually they should not be used in application-level code.

Additionally hash functions implement traits from the standard library:
[`Default`], [`Clone`], [`Write`][std::io::Write]. The latter is
feature-gated behind `std` feature, which is usually enabled by default
by hash implementation crates.

## Modules

### [`digest`](digest.md)

*1 macro, 1 module, 2 structs, 8 traits*

### [`core_api`](core_api.md)

*1 enum, 1 type alias, 6 traits*

### [`core_api::ct_variable`](core_api/ct_variable.md)

*2 structs*

### [`core_api::rt_variable`](core_api/rt_variable.md)

*1 struct*

### [`core_api::wrapper`](core_api/wrapper.md)

*1 struct, 1 trait*

### [`core_api::wrapper::sealed`](core_api/wrapper/sealed.md)

*1 trait*

### [`core_api::xof_reader`](core_api/xof_reader.md)

*1 struct*

### [`digest`](digest.md)

*3 traits*

