**symphonia_core > packet > builder**

# Module: packet::builder

## Contents

**Structs**

- [`HasBuf`](#hasbuf)
- [`HasBufRef`](#hasbufref)
- [`HasDur`](#hasdur)
- [`HasPts`](#haspts)
- [`HasTrackId`](#hastrackid)
- [`NoBuf`](#nobuf)
- [`NoDur`](#nodur)
- [`NoPts`](#nopts)
- [`NoTrackId`](#notrackid)
- [`PacketBuilder`](#packetbuilder) - A builder for creating owning or non-owning packets.

---

## symphonia_core::packet::builder::HasBuf

*Struct*

**Tuple Struct**: `()`



## symphonia_core::packet::builder::HasBufRef

*Struct*

**Generic Parameters:**
- 'a

**Tuple Struct**: `()`



## symphonia_core::packet::builder::HasDur

*Struct*

**Tuple Struct**: `()`



## symphonia_core::packet::builder::HasPts

*Struct*

**Tuple Struct**: `()`



## symphonia_core::packet::builder::HasTrackId

*Struct*

**Tuple Struct**: `()`



## symphonia_core::packet::builder::NoBuf

*Struct*

**Unit Struct**



## symphonia_core::packet::builder::NoDur

*Struct*

**Unit Struct**



## symphonia_core::packet::builder::NoPts

*Struct*

**Unit Struct**



## symphonia_core::packet::builder::NoTrackId

*Struct*

**Unit Struct**



## symphonia_core::packet::builder::PacketBuilder

*Struct*

A builder for creating owning or non-owning packets.

See [`Packet`] for a detailed description of all packet fields.

The track ID, PTS, duration, and data fields are mandatory and must be provided before a
packet can be built.

**Generic Parameters:**
- T
- P
- D
- B

**Methods:**

- `fn build_packet_ref(self: Self) -> PacketRef<'a>` - Build a non-owning packet.
- `fn track_id(self: Self, track_id: u32) -> PacketBuilder<HasTrackId, P, D, B>` - Provide the track ID.
- `fn pts(self: Self, pts: Timestamp) -> PacketBuilder<T, HasPts, D, B>` - Provide the presentation timestamp (PTS).
- `fn data<impl Into<Box<[u8]>>>(self: Self, buf: impl Trait) -> PacketBuilder<T, P, D, HasBuf>` - Provide the packet's data buffer.
- `fn data_by_ref<'a>(self: Self, buf: &'a [u8]) -> PacketBuilder<T, P, D, HasBufRef<'a>>` - Provide the packet's data buffer as a non-owning reference.
- `fn dts(self: Self, dts: Timestamp) -> Self` - Provide the decode timestamp (DTS).
- `fn trim_start(self: Self, trim_start: Duration) -> Self` - Provide the trim start duration.
- `fn trim_end(self: Self, trim_end: Duration) -> Self` - Provide the trim end duration.
- `fn new() -> Self` - Create the packet builder.
- `fn trimmed_dur(self: Self, block_dur: Duration, end_pts: Option<Timestamp>) -> PacketBuilder<T, HasPts, HasDur, B>` - Provide the packet's duration and calculate the trim fields.
- `fn build(self: Self) -> Packet` - Build the packet.
- `fn dur(self: Self, dur: Duration) -> PacketBuilder<T, P, HasDur, B>` - Provide the packet's duration including delay and padding frames.

**Trait Implementations:**

- **Default**
  - `fn default() -> Self`



