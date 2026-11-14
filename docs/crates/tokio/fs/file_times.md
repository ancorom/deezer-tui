**tokio > fs > file_times**

# Module: fs::file_times

## Contents

**Structs**

- [`FileTimes`](#filetimes) - Timestamps for a file.

---

## tokio::fs::file_times::FileTimes

*Struct*

Timestamps for a file.

This is a specialized version of [`std::fs::FileTimes`] for usage with
[`File::set_times`](crate::fs::File::set_times). Unlike the standard library version,
this type records the timestamps itself, so a future `io-uring` file
implementation can consume them without converting through the standard
library type.

# Examples

```no_run
use tokio::fs::FileTimes;

# async fn dox() -> std::io::Result<()> {
let times = FileTimes::new()
    .set_accessed(std::time::SystemTime::now())
    .set_modified(std::time::SystemTime::now());
# Ok(())
# }
```

**Methods:**

- `fn new() -> Self` - Creates a new empty [`FileTimes`].
- `fn set_accessed(self: Self, t: SystemTime) -> Self` - Sets the last access time.
- `fn set_modified(self: Self, t: SystemTime) -> Self` - Sets the last modification time.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



