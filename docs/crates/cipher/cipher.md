**cipher**

# Module: cipher

## Contents

**Macros**

- [`impl_simple_block_encdec`](#impl_simple_block_encdec) - Implement simple block backend

**Traits**

- [`IvState`](#ivstate) - Trait for loading current IV state.

---

## cipher::IvState

*Trait*

Trait for loading current IV state.

**Methods:**

- `iv_state`: Returns current IV state.



## cipher::impl_simple_block_encdec

*Declarative Macro*

Implement simple block backend

```rust
macro_rules! impl_simple_block_encdec {
    (
        <$($N:ident$(:$b0:ident$(+$b:ident)*)?),*>
        $cipher:ident, $block_size:ty, $state:ident, $block:ident,
        encrypt: $enc_block:block
        decrypt: $dec_block:block
    ) => { ... };
    (
        $cipher:ident, $block_size:ty, $state:ident, $block:ident,
        encrypt: $enc_block:block
        decrypt: $dec_block:block
    ) => { ... };
}
```



