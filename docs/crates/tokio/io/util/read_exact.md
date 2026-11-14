**tokio > io > util > read_exact**

# Module: io::util::read_exact

## Contents

**Structs**

- [`ReadExact`](#readexact) - Creates a future which will read exactly enough bytes to fill `buf`,

---

## tokio::io::util::read_exact::ReadExact

*Struct*

Creates a future which will read exactly enough bytes to fill `buf`,
returning an error if EOF is hit sooner.

On success the number of bytes is returned

**Generic Parameters:**
- 'a
- A



