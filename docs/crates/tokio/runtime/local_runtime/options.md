**tokio > runtime > local_runtime > options**

# Module: runtime::local_runtime::options

## Contents

**Structs**

- [`LocalOptions`](#localoptions) - [`LocalRuntime`]-only config options

---

## tokio::runtime::local_runtime::options::LocalOptions

*Struct*

[`LocalRuntime`]-only config options

Currently, there are no such options, but in the future, things like `!Send + !Sync` hooks may
be added.

Use `LocalOptions::default()` to create the default set of options. This type is used with
[`Builder::build_local`].

[`Builder::build_local`]: crate::runtime::Builder::build_local
[`LocalRuntime`]: crate::runtime::LocalRuntime

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



