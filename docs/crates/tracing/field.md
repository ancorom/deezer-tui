**tracing > field**

# Module: field

## Contents

**Traits**

- [`AsField`](#asfield) - Trait implemented to allow a type to be used as a field key.

---

## tracing::field::AsField

*Trait*

Trait implemented to allow a type to be used as a field key.

<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: Although this is implemented for both the
<a href="./struct.Field.html"><code>Field</code></a> type <em>and</em> any
type that can be borrowed as an <code>&str</code>, only <code>Field</code>
allows <em>O</em>(1) access.
Indexing a field with a string results in an iterative search that performs
string comparisons. Thus, if possible, once the key for a field is known, it
should be used whenever possible.
</pre>

**Methods:**

- `as_field`: Attempts to convert `&self` into a `Field` with the specified `metadata`.



