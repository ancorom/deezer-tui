**toml > de > parser > dearray**

# Module: de::parser::dearray

## Contents

**Structs**

- [`DeArray`](#dearray) - Type representing a TOML array, payload of the `DeValue::Array` variant

---

## toml::de::parser::dearray::DeArray

*Struct*

Type representing a TOML array, payload of the `DeValue::Array` variant

**Generic Parameters:**
- 'i

**Methods:**

- `fn new() -> Self` - Constructs a new, empty `DeArray`.
- `fn push(self: & mut Self, value: Spanned<DeValue<'i>>)` - Appends an element to the back of a collection.

**Trait Implementations:**

- **Borrow**
  - `fn borrow(self: &Self) -> &[Spanned<DeValue<'i>>]`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut [Spanned<DeValue<'i>>]`
- **Debug**
  - `fn fmt(self: &Self, formatter: & mut core::fmt::Formatter) -> core::fmt::Result`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[Spanned<DeValue<'i>>]`
- **FromIterator**
  - `fn from_iter<I>(iter: I) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Index**
  - `fn index(self: &Self, index: I) -> &<Self as >::Output`
- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut [Spanned<DeValue<'i>>]`
- **Deref**
  - `fn deref(self: &Self) -> &[Spanned<DeValue<'i>>]`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut [Spanned<DeValue<'i>>]`
- **Default**
  - `fn default() -> Self`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`



