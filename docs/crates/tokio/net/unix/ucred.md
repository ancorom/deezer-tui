**tokio > net > unix > ucred**

# Module: net::unix::ucred

## Contents

**Structs**

- [`UCred`](#ucred) - Credentials of a process.

---

## tokio::net::unix::ucred::UCred

*Struct*

Credentials of a process.

**Methods:**

- `fn uid(self: &Self) -> unix::uid_t` - Gets UID (user ID) of the process.
- `fn gid(self: &Self) -> unix::gid_t` - Gets GID (group ID) of the process.
- `fn pid(self: &Self) -> Option<unix::pid_t>` - Gets PID (process ID) of the process.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



