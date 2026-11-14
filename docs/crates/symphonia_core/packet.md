**symphonia_core > packet**

# Module: packet

## Contents

**Structs**

- [`Packet`](#packet) - A `Packet` contains a discrete amount of encoded data for a single codec bitstream. The exact
- [`PacketRef`](#packetref) - A non-owning equivalent to `Packet`.

---

## symphonia_core::packet::Packet

*Struct*

A `Packet` contains a discrete amount of encoded data for a single codec bitstream. The exact
amount of data is bounded, but not defined, and is dependant on the container and/or the
encapsulated codec.

# Timing

Packets carry various timing information to support the decoding process. Generally, this
timing information is read directly from the media container but it may also be synthesized by
a format reader if it is not explicitly signalled.

* **Presentation Timestamp (PTS):** The time relative to the start of the stream that the
  decoded packet should be presented.

* **Decode Timestamp (DTS):** The time relative to the start of the stream that the packet
  should be decoded. The DTS is usually the same as PTS for audio.

* **Duration:** The duration of all *valid* frames in the packet. Equal to the duration of all
  *decoded* frames if there is no trimming.

* **Trim Start/End:** The duration of frames that should be trimmed from the start/end of the
  decoded packet before presentation. The sum of the duration and trim start/end equals the
  duration of *decoded* frames.

Take note of the difference between *valid* and *decoded* frames. Valid frames are frames that
should be presented (played back) to the user, while decoded frames include any encoder delay
and/or padding frames. The latter are generally discarded by the decoder. The duration of all
*decoded* frames is also called the block duration.

# For Implementers

When synthesizing PTS, negative PTS should be used for encoder delay frames. However, this is
not strictly mandatory. Encoder delay and padding frames are ultimately signalled using the trim
fields. Regardless, the `dur` field must only be populated with the duration of *valid* frames,
while the sum of `dur`, `trim_start`, and `trim_end` must equal the amount of frames the decoder
would produce if no trimming occurs.

**Fields:**
- `track_id: u32` - The track ID of the track this packet belongs to.
- `pts: crate::units::Timestamp` - The presentation timestamp (PTS) of the packet in `TimeBase` units.+
- `dts: crate::units::Timestamp` - The decode timestamp (DTS) of the packet in `TimeBase` units.
- `dur: crate::units::Duration` - The duration of all *valid* frames in the packet in `TimeBase` units.
- `trim_start: crate::units::Duration` - The duration of *decoded* frames that should be trimmed from the start of the decoded
- `trim_end: crate::units::Duration` - The duration of *decoded* frames that should be trimmed from the end of the decoded
- `data: Box<[u8]>` - The packet data buffer.

**Methods:**

- `fn new<impl Into<Box<[u8]>>>(track_id: u32, pts: Timestamp, dur: Duration, data: impl Trait) -> Self` - Create a new untrimmed `Packet`.
- `fn block_dur(self: &Self) -> Duration` - Get the duration of all *decoded* frames in the packet in `TimeBase` units.
- `fn as_buf_reader(self: &Self) -> BufReader` - Get a `BufReader` to read the packet data buffer sequentially.
- `fn as_packet_ref(self: &Self) -> PacketRef` - Get a `PacketRef` borrowing this packet's data buffer.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## symphonia_core::packet::PacketRef

*Struct*

A non-owning equivalent to `Packet`.

`PacketRef` is the zero-copy, non-owning, equivalent of `Packet`. It is particularly useful when
packet data is already residing in an application-managed fixed buffer or when data is
passed in from external environments like C/C++ via FFI. This allows decoders to process
the data directly without forcing a heap allocation and deep copy.

See [`Packet`] for more details on the various timing and implementation details.

**Generic Parameters:**
- 'a

**Fields:**
- `track_id: u32` - The track ID of the track this packet belongs to.
- `pts: crate::units::Timestamp` - The presentation timestamp (PTS) of the packet in `TimeBase` units.+
- `dts: crate::units::Timestamp` - The decode timestamp (DTS) of the packet in `TimeBase` units.
- `dur: crate::units::Duration` - The duration of all *valid* frames in the packet in `TimeBase` units.
- `trim_start: crate::units::Duration` - The duration of *decoded* frames that should be trimmed from the start of the decoded
- `trim_end: crate::units::Duration` - The duration of *decoded* frames that should be trimmed from the end of the decoded
- `data: &'a [u8]` - The packet data buffer.

**Methods:**

- `fn new(track_id: u32, pts: Timestamp, dur: Duration, data: &'a [u8]) -> Self` - Create a new untrimmed `PacketRef`.
- `fn block_dur(self: &Self) -> Duration` - Get the duration of all *decoded* frames in the packet in `TimeBase` units.
- `fn as_buf_reader(self: &Self) -> BufReader` - Get a `BufReader` to read the packet data buffer sequentially.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **From**
  - `fn from(packet: &'a Packet) -> Self`



