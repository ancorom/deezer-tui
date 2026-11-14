**symphonia_core > io > bit > private**

# Module: io::bit::private

## Contents

**Traits**

- [`FetchBitsLtr`](#fetchbitsltr)
- [`FetchBitsRtl`](#fetchbitsrtl)

---

## symphonia_core::io::bit::private::FetchBitsLtr

*Trait*

**Methods:**

- `fetch_bits`: Discard any remaining bits in the source and fetch 1 or more new bits.
- `fetch_bits_partial`: Fetch 0 or more new bits, and append them after the remaining bits.
- `get_bits`: Get all the bits in the source.
- `num_bits_left`: Get the number of bits left in the source.
- `consume_bits`: Consume `num` bits from the source.



## symphonia_core::io::bit::private::FetchBitsRtl

*Trait*

**Methods:**

- `fetch_bits`: Discard any remaining bits in the source and fetch 1 or more new bits.
- `fetch_bits_partial`: Fetch 0 or more new bits, and append them after the remaining bits.
- `get_bits`: Get all the bits in the source.
- `num_bits_left`: Get the number of bits left in the source.
- `consume_bits`: Consume `num` bits from the source.



