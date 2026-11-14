# cpal

Low-level library for audio input and output, written in Rust.

For higher-level audio playback and capture, consider
[Rodio](https://github.com/RustAudio/rodio) or similar libraries.

# How to use cpal

Here are some concepts cpal exposes:

- A [`Host`] provides access to the available audio devices on the system.
  Some platforms have more than one host available, but every platform supported by CPAL has at
  least one [default_host] that is guaranteed to be available.
- A [`Device`] is an audio device that may have any number of input and
  output streams.
- A [`Stream`] is an open flow of audio data. Input streams allow you to
  receive audio data, output streams allow you to play audio data. You must choose which
  [Device] will run your stream before you can create one. Often, a default device can be
  retrieved via the [Host].

The first step is to initialise the [`Host`]:

```
use cpal::traits::HostTrait;
let host = cpal::default_host();
```

Then choose an available [`Device`]. The easiest way is to use the default input or output
`Device` via the [`default_input_device()`] or [`default_output_device()`] methods on `host`.

Alternatively, you can enumerate all the available devices with the [`devices()`] method.
Beware that the `default_*_device()` functions return an `Option<Device>` in case no device
is available for that stream type on the system.

```no_run
# use cpal::traits::HostTrait;
# let host = cpal::default_host();
let device = host.default_output_device().expect("no output device available");
```

Before we can create a stream, we must decide what the configuration of the audio stream is
going to be.
You can query all the supported configurations with the
[`supported_input_configs()`] and [`supported_output_configs()`] methods.
These produce a list of [`SupportedStreamConfigRange`] structs which can later be turned into
actual [`SupportedStreamConfig`] structs.

If you don't want to query the list of configs,
you can also build your own [`StreamConfig`] manually, but doing so could lead to an error when
building the stream if the config is not supported by the device.

> **Note**: the `supported_input/output_configs()` methods
> could return an error for example if the device has been disconnected.

```no_run
use cpal::traits::{DeviceTrait, HostTrait};
# let host = cpal::default_host();
# let device = host.default_output_device().unwrap();
let mut supported_configs_range = device.supported_output_configs()
    .expect("error while querying configs");
let supported_config = supported_configs_range.next()
    .expect("no supported config?!")
    .with_max_sample_rate();
```

Now that we have everything for the stream, we are ready to create it from our selected device:

```no_run
use cpal::Data;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
# let host = cpal::default_host();
# let device = host.default_output_device().unwrap();
# let config = device.default_output_config().unwrap().into();
let stream = device.build_output_stream(
    config,
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        // react to stream events and read or write stream data here.
    },
    move |err| {
        // react to errors here.
    },
    None // Timeout for stream initialization: None = wait indefinitely,
         // Some(Duration) = time to wait for the backend to initialize the stream.
);
```

Streams are returned in a paused state. Once the stream has been started with
[`Stream::play`](traits::StreamTrait::play), the selected audio device will periodically call
the data callback that was passed to the function. For input streams, the callback receives
`&`[`Data`] containing captured audio samples. For output streams, the callback receives
`&mut`[`Data`] to be filled with audio samples for playback.

> **Note**: Creating and running a stream will *not* block the thread. On modern platforms, the
> given callback is called by a dedicated, high-priority thread responsible for delivering
> audio data to the system's audio device in a timely manner. On older platforms that only
> provide a blocking API (e.g. ALSA), CPAL will create a thread in order to consistently
> provide non-blocking behaviour (currently this is a thread per stream, but this may change to
> use a single thread for all streams). *If this is an issue for your platform or design,
> please share your issue and use-case with the CPAL team on the GitHub issue tracker for
> consideration.*

In this example, we simply fill the given output buffer with silence.

```no_run
use cpal::{Data, Sample, SampleFormat, FromSample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
# let host = cpal::default_host();
# let device = host.default_output_device().unwrap();
# let supported_config = device.default_output_config().unwrap();
let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
let sample_format = supported_config.sample_format();
let config = supported_config.into();
let stream = match sample_format {
    SampleFormat::F32 => device.build_output_stream(config, write_silence::<f32>, err_fn, None),
    SampleFormat::I16 => device.build_output_stream(config, write_silence::<i16>, err_fn, None),
    SampleFormat::U16 => device.build_output_stream(config, write_silence::<u16>, err_fn, None),
    sample_format => panic!("Unsupported sample format '{sample_format}'")
}.unwrap();

fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
    for sample in data.iter_mut() {
        *sample = Sample::EQUILIBRIUM;
    }
}
```

Streams are always returned in a paused state, so we must call
[`Stream::play`](traits::StreamTrait::play) to start running the data callback.

```no_run
# use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
# let host = cpal::default_host();
# let device = host.default_output_device().unwrap();
# let supported_config = device.default_output_config().unwrap();
# let sample_format = supported_config.sample_format();
# let config = supported_config.into();
# let data_fn = move |_data: &mut cpal::Data, _: &cpal::OutputCallbackInfo| {};
# let err_fn = move |_err| {};
# let stream = device.build_output_stream_raw(config, sample_format, data_fn, err_fn, None).unwrap();
stream.play().unwrap();
```

Some devices support pausing the audio stream. This can be useful for saving energy in moments
of silence.

```no_run
# use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
# let host = cpal::default_host();
# let device = host.default_output_device().unwrap();
# let supported_config = device.default_output_config().unwrap();
# let sample_format = supported_config.sample_format();
# let config = supported_config.into();
# let data_fn = move |_data: &mut cpal::Data, _: &cpal::OutputCallbackInfo| {};
# let err_fn = move |_err| {};
# let stream = device.build_output_stream_raw(config, sample_format, data_fn, err_fn, None).unwrap();
stream.pause().unwrap();
```

[`default_input_device()`]: traits::HostTrait::default_input_device
[`default_output_device()`]: traits::HostTrait::default_output_device
[`devices()`]: traits::HostTrait::devices
[`supported_input_configs()`]: traits::DeviceTrait::supported_input_configs
[`supported_output_configs()`]: traits::DeviceTrait::supported_output_configs

## Modules

### [`cpal`](cpal.md)

*2 constants, 2 enums, 2 macros, 3 modules, 5 structs, 6 type aliases*

### [`Data`](Data.md)

*2 functions*

### [`SupportedStreamConfig`](SupportedStreamConfig.md)

*1 function*

### [`SupportedStreamConfigRange`](SupportedStreamConfigRange.md)

*1 function*

### [`device_description`](device_description.md)

*2 structs, 3 enums*

### [`error`](error.md)

*1 enum, 1 struct*

### [`host::alsa`](host/alsa.md)

*3 structs*

### [`platform::platform_impl`](platform/platform_impl.md)

*1 constant, 1 enum, 3 functions, 6 structs*

### [`sample_format`](sample_format.md)

*1 enum, 1 trait*

### [`sample_format::SampleFormat`](sample_format/SampleFormat.md)

*3 functions*

### [`timestamp`](timestamp.md)

*5 structs*

### [`timestamp::StreamInstant`](timestamp/StreamInstant.md)

*2 functions*

### [`traits`](traits.md)

*3 traits*



## Project usage (deezer-tui) — Phase 2 output
- Explicit 44.1 kHz (default changed to 48k in 0.18): `device.supported_output_configs().find_map(|r| r.try_with_sample_rate(cpal::SAMPLE_RATE_CD))` where `SAMPLE_RATE_CD = 44100`.
- Sample format: `SampleFormat::F32`, stereo, callback buffer is `&[f32]`.
- RT callback: read from an `rtrb::Consumer<f32>` via `pop()` in a loop (non-blocking), fill `0.0` on `PopError::Empty`, multiply by an atomic volume knob (`AtomicU32` bits via `f32::from_bits`).
- Source: `src/audio/output.rs`.

### Verified working patterns (Phase 2, `src/audio/output.rs`)
- `default_output_device()` returns **`Option<Device>`** → `.ok_or(cpal::Error::new(cpal::ErrorKind::DeviceNotAvailable))?`.
- `supported_output_configs()` returns **`Result<SupportedOutputConfigs, Error>`** (an iterator, not a list) → unwrap with `.expect("…")`. It yields `SupportedStreamConfigRange`.
- Select a concrete stream config: `supported_output_configs().expect("…").find_map(|r| r.try_with_sample_rate(cpal::SAMPLE_RATE_CD)).expect("…")` gives a `SupportedStreamConfig`.
- `build_output_stream(config, cb, err_cb, None)` expects a **`cpal::StreamConfig`**, not a `SupportedStreamConfig` → call `.into()` on the `SupportedStreamConfig` (impl `Into<StreamConfig>`).
- Real-time callback signature is `FnMut(&mut [f32], &CallbackInfo)` returning **`()`** (NOT `Ok(())`).
- Full config-selection:
  ```rust
  let config: cpal::StreamConfig = device
      .supported_output_configs()
      .expect("error while querying configs")
      .find_map(|r| r.try_with_sample_rate(cpal::SAMPLE_RATE_CD))
      .expect("device does not support 44.1 kHz")
      .into();
  ```
- The `Consumer<f32>` is **moved into** the callback so the callback owns it and never blocks.