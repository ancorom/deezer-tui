**reqwest > retry > classify**

# Module: retry::classify

## Contents

**Structs**

- [`ReqRep`](#reqrep) - A request/response result to inspect for possible retries.

**Enums**

- [`Action`](#action)

**Traits**

- [`Classify`](#classify)

---

## reqwest::retry::classify::Action

*Enum*

**Variants:**
- `Success`
- `Retryable`



## reqwest::retry::classify::Classify

*Trait*

**Methods:**

- `classify`



## reqwest::retry::classify::ReqRep

*Struct*

A request/response result to inspect for possible retries.

This is passed to a `classify` function.

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`



