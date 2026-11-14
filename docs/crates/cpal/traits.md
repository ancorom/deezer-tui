**cpal > traits**

# Module: traits

## Contents

**Traits**

- [`DeviceTrait`](#devicetrait) - A device that is capable of audio input and/or output.
- [`HostTrait`](#hosttrait) - A [`Host`] provides access to the available audio devices on the system.
- [`StreamTrait`](#streamtrait) - A stream created from [`Device`](DeviceTrait), with methods to control it.

---

## cpal::traits::DeviceTrait

*Trait*

A device that is capable of audio input and/or output.

Please note that `Device`s may become invalid if they get disconnected. Therefore, all the
methods that involve a device return a `Result` allowing the user to handle this case.

**Methods:**

- `SupportedInputConfigs`: The iterator type yielding supported input stream formats.
- `SupportedOutputConfigs`: The iterator type yielding supported output stream formats.
- `Stream`: The stream type created by [`build_input_stream_raw`] and [`build_output_stream_raw`].
- `description`: Structured description of the device with metadata.
- `id`: The ID of the device.
- `supports_input`: True if the device supports audio input, otherwise false
- `supports_output`: True if the device supports audio output, otherwise false
- `supported_input_configs`: An iterator yielding input stream configurations that are supported by the device.
- `supported_output_configs`: An iterator yielding output stream configurations that are supported by the device.
- `default_input_config`: The default input stream configuration for the device.
- `default_output_config`: The default output stream configuration for the device.
- `build_input_stream`: Create an input stream.
- `build_output_stream`: Create an output stream.
- `build_input_stream_raw`: Create a dynamically typed input stream.
- `build_output_stream_raw`: Create a dynamically typed output stream.



## cpal::traits::HostTrait

*Trait*

A [`Host`] provides access to the available audio devices on the system.

Each platform may have a number of available hosts depending on the system, each with their own
pros and cons.

For example, WASAPI is the standard audio host API that ships with the Windows operating
system. However, due to historical limitations with respect to performance and flexibility,
Steinberg created the ASIO API providing better audio device support for pro audio and
low-latency applications. As a result, it is common for some devices and device capabilities to
only be available via ASIO, while others are only available via WASAPI.

Another great example is the Linux platform. While the ALSA host API is the lowest-level API
available to almost all distributions of Linux, its flexibility is limited as it requires that
each process have exclusive access to the devices with which they establish streams. PulseAudio
is another popular host API that aims to solve this issue by providing user-space mixing,
however it has its own limitations w.r.t. low-latency and high-performance audio applications.
JACK is yet another host API that is more suitable to pro-audio applications, however it is
less readily available by default in many Linux distributions and is known to be tricky to
set up.

[`Host`]: crate::Host

**Methods:**

- `Devices`: The type used for enumerating available devices by the host.
- `Device`: The `Device` type yielded by the host.
- `is_available`: Whether or not the host is available on the system.
- `devices`: An iterator yielding all [`Device`](DeviceTrait)s currently available to the host on the system.
- `device_by_id`: Fetches a [`Device`](DeviceTrait) based on a [`DeviceId`] if available
- `default_input_device`: The default input audio device on the system.
- `default_output_device`: The default output audio device on the system.
- `input_devices`: An iterator yielding all `Device`s currently available to the system that support one or more
- `output_devices`: An iterator yielding all `Device`s currently available to the system that support one or more



## cpal::traits::StreamTrait

*Trait*

A stream created from [`Device`](DeviceTrait), with methods to control it.

**Methods:**

- `play`: Start the stream.
- `pause`: Pause the stream. Some devices support suspending at the hardware level (saving energy);
- `buffer_size`: Returns the backend's best available estimate of the number of frames per callback.
- `now`: Returns a [`StreamInstant`] representing the current moment on the stream's clock.



