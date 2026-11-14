**cpal > device_description**

# Module: device_description

## Contents

**Structs**

- [`DeviceDescription`](#devicedescription) - Describes an audio device with structured metadata.
- [`DeviceDescriptionBuilder`](#devicedescriptionbuilder) - Builder for constructing a `DeviceDescription`.

**Enums**

- [`DeviceDirection`](#devicedirection) - The direction(s) that a device supports.
- [`DeviceType`](#devicetype) - Categorization of audio device types.
- [`InterfaceType`](#interfacetype) - How the device connects to the system (interface/connection type).

---

## cpal::device_description::DeviceDescription

*Struct*

Describes an audio device with structured metadata.

This type provides structured information about an audio device beyond just its name.
Availability depends on the host implementation and platform capabilities.

**Methods:**

- `fn name(self: &Self) -> &str` - Returns the human-readable device name.
- `fn manufacturer(self: &Self) -> Option<&str>` - Returns the manufacturer/vendor name if available.
- `fn driver(self: &Self) -> Option<&str>` - Returns the driver name if available.
- `fn device_type(self: &Self) -> DeviceType` - Returns the device type categorization.
- `fn interface_type(self: &Self) -> InterfaceType` - Returns the interface/connection type.
- `fn direction(self: &Self) -> DeviceDirection` - Returns the device direction.
- `fn supports_input(self: &Self) -> bool` - Returns whether this device supports audio input (capture).
- `fn supports_output(self: &Self) -> bool` - Returns whether this device supports audio output (playback).
- `fn address(self: &Self) -> Option<&str>` - Returns the physical address or connection identifier if available.
- `fn extended(self: &Self) -> impl Trait` - Returns additional description lines with detailed information.

**Traits:** Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::device_description::DeviceDescriptionBuilder

*Struct*

Builder for constructing a `DeviceDescription`.

This is primarily used by host implementations and custom hosts
to gradually build up device descriptions with available metadata.

**Methods:**

- `fn new<impl AsRef<str>>(name: impl Trait) -> Self` - Creates a new builder with the device name (required).
- `fn manufacturer<impl AsRef<str>>(self: Self, manufacturer: impl Trait) -> Self` - Sets the manufacturer name.
- `fn driver<impl AsRef<str>>(self: Self, driver: impl Trait) -> Self` - Sets the driver name.
- `fn device_type(self: Self, device_type: DeviceType) -> Self` - Sets the device type.
- `fn interface_type(self: Self, interface_type: InterfaceType) -> Self` - Sets the interface type.
- `fn direction(self: Self, direction: DeviceDirection) -> Self` - Sets the device direction.
- `fn address<impl AsRef<str>>(self: Self, address: impl Trait) -> Self` - Sets the physical address.
- `fn extended<S, impl IntoIterator<Item = S>>(self: Self, lines: impl Trait) -> Self` - Sets the description lines, replacing any previously added lines.
- `fn add_extended_line<impl AsRef<str>>(self: Self, line: impl Trait) -> Self` - Adds a single description line.
- `fn build(self: Self) -> DeviceDescription` - Builds the [`DeviceDescription`].

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## cpal::device_description::DeviceDirection

*Enum*

The direction(s) that a device supports.

**Variants:**
- `Input` - Input only (capture/recording)
- `Output` - Output only (playback/rendering)
- `Duplex` - Both input and output
- `Unknown` - Direction unknown or not yet determined

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(direction: alsa::Direction) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## cpal::device_description::DeviceType

*Enum*

Categorization of audio device types.

This describes the kind of audio device (speaker, microphone, headset, etc.)
regardless of how it connects to the system.

**Variants:**
- `Speaker` - Speaker (built-in or external)
- `Microphone` - Microphone (built-in or external)
- `Headphones` - Headphones (audio output only)
- `Headset` - Headset (combined headphones + microphone)
- `Earpiece` - Earpiece (phone-style speaker, typically for voice calls)
- `Handset` - Handset (telephone-style handset with speaker and microphone)
- `HearingAid` - Hearing aid device
- `Dock` - Docking station audio
- `Tuner` - Radio/TV tuner
- `Virtual` - Virtual/loopback device (software audio routing)
- `Unknown` - Unknown or unclassified device type

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## cpal::device_description::InterfaceType

*Enum*

How the device connects to the system (interface/connection type).

This describes the physical or logical connection between the audio device
and the computer system.

**Variants:**
- `BuiltIn` - Built-in to the system (integrated audio chipset)
- `Usb` - USB connection
- `Bluetooth` - Bluetooth wireless connection
- `Pci` - PCI or PCIe card (internal sound card)
- `FireWire` - FireWire connection (IEEE 1394)
- `Thunderbolt` - Thunderbolt connection
- `Hdmi` - HDMI connection
- `Line` - Line-level analog connection (line in/out, aux)
- `Spdif` - S/PDIF digital audio interface
- `Network` - Network connection (Dante, AVB, AirPlay, IP audio, etc.)
- `Virtual` - Virtual/loopback connection (software audio routing, not physical hardware)
- `DisplayPort` - DisplayPort audio
- `Aggregate` - Aggregate device (combines multiple devices)
- `Unknown` - Unknown connection type

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



