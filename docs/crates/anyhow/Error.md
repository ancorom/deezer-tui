**anyhow > Error**

# Module: Error

## Contents

**Functions**

- [`chain`](#chain) - An iterator of the chain of source errors contained by this Error.
- [`from_boxed`](#from_boxed) - Construct an error object from a type-erased standard library error.

---

## anyhow::Error::chain

*Function*

An iterator of the chain of source errors contained by this Error.

This iterator will visit every error in the cause chain of this error
object, beginning with the error that this error object was created
from.

# Example

```
use anyhow::Error;
use std::io;

pub fn underlying_io_error_kind(error: &Error) -> Option<io::ErrorKind> {
    for cause in error.chain() {
        if let Some(io_error) = cause.downcast_ref::<io::Error>() {
            return Some(io_error.kind());
        }
    }
    None
}
```

```rust
fn chain(self: &Self) -> Chain
```



## anyhow::Error::from_boxed

*Function*

Construct an error object from a type-erased standard library error.

This is mostly useful for interop with other error libraries.

# Example

Here is a skeleton of a library that provides its own error abstraction.
The pair of `From` impls provide bidirectional support for `?`
conversion between `Report` and `anyhow::Error`.

```
use std::error::Error as StdError;

pub struct Report {/* ... */}

impl<E> From<E> for Report
where
    E: Into<anyhow::Error>,
    Result<(), E>: anyhow::Context<(), E>,
{
    fn from(error: E) -> Self {
        let anyhow_error: anyhow::Error = error.into();
        let boxed_error: Box<dyn StdError + Send + Sync + 'static> = anyhow_error.into();
        Report::from_boxed(boxed_error)
    }
}

impl From<Report> for anyhow::Error {
    fn from(report: Report) -> Self {
        let boxed_error: Box<dyn StdError + Send + Sync + 'static> = report.into_boxed();
        anyhow::Error::from_boxed(boxed_error)
    }
}

impl Report {
    fn from_boxed(boxed_error: Box<dyn StdError + Send + Sync + 'static>) -> Self {
        todo!()
    }
    fn into_boxed(self) -> Box<dyn StdError + Send + Sync + 'static> {
        todo!()
    }
}

// Example usage: can use `?` in both directions.
fn a() -> anyhow::Result<()> {
    b()?;
    Ok(())
}
fn b() -> Result<(), Report> {
    a()?;
    Ok(())
}
```

```rust
fn from_boxed(boxed_error: Box<dyn StdError>) -> Self
```



