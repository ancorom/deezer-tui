**tracing_subscriber > fmt > writer > EitherWriter**

# Module: fmt::writer::EitherWriter

## Contents

**Functions**

- [`none`](#none) - Returns a [disabled writer].
- [`some`](#some) - Returns an enabled writer of type `T`.

---

## tracing_subscriber::fmt::writer::EitherWriter::none

*Function*

Returns a [disabled writer].

Any bytes written to the returned writer are discarded.

This is equivalent to returning [`Option::None`].

[disabled writer]: std::io::sink

```rust
fn none() -> Self
```



## tracing_subscriber::fmt::writer::EitherWriter::some

*Function*

Returns an enabled writer of type `T`.

This is equivalent to returning [`Option::Some`].

```rust
fn some(t: T) -> Self
```



