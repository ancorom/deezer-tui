# tokio

A runtime for writing reliable network applications without compromising speed.

Tokio is an event-driven, non-blocking I/O platform for writing asynchronous
applications with the Rust programming language. At a high level, it
provides a few major components:

* Tools for [working with asynchronous tasks][tasks], including
  [synchronization primitives and channels][sync] and [timeouts, sleeps, and
  intervals][time].
* APIs for [performing asynchronous I/O][io], including [TCP and UDP][net] sockets,
  [filesystem][fs] operations, and [process] and [signal] management.
* A [runtime] for executing asynchronous code, including a task scheduler,
  an I/O driver backed by the operating system's event queue (`epoll`, `kqueue`,
  `IOCP`, etc...), and a high performance timer.

Guide level documentation is found on the [website].

[tasks]: #working-with-tasks
[sync]: crate::sync
[time]: crate::time
[io]: #asynchronous-io
[net]: crate::net
[fs]: crate::fs
[process]: crate::process
[signal]: crate::signal
[fs]: crate::fs
[runtime]: crate::runtime
[website]: https://tokio.rs/tokio/tutorial

# A Tour of Tokio

Tokio consists of a number of modules that provide a range of functionality
essential for implementing asynchronous applications in Rust. In this
section, we will take a brief tour of Tokio, summarizing the major APIs and
their uses.

The easiest way to get started is to enable all features. Do this by
enabling the `full` feature flag:

```toml
tokio = { version = "1", features = ["full"] }
```

### Authoring applications

Tokio is great for writing applications and most users in this case shouldn't
worry too much about what features they should pick. If you're unsure, we suggest
going with `full` to ensure that you don't run into any road blocks while you're
building your application.

#### Example

This example shows the quickest way to get started with Tokio.

```toml
tokio = { version = "1", features = ["full"] }
```

### Authoring libraries

As a library author your goal should be to provide the lightest weight crate
that is based on Tokio. To achieve this you should ensure that you only enable
the features you need. This allows users to pick up your crate without having
to enable unnecessary features.

#### Example

This example shows how you may want to import features for a library that just
needs to `tokio::spawn` and use a `TcpStream`.

```toml
tokio = { version = "1", features = ["rt", "net"] }
```

## Working With Tasks

Asynchronous programs in Rust are based around lightweight, non-blocking
units of execution called [_tasks_][tasks]. The [`tokio::task`] module provides
important tools for working with tasks:

* The [`spawn`] function and [`JoinHandle`] type, for scheduling a new task
  on the Tokio runtime and awaiting the output of a spawned task, respectively,
* Functions for [running blocking operations][blocking] in an asynchronous
  task context.

The [`tokio::task`] module is present only when the "rt" feature flag
is enabled.

[tasks]: task/index.html#what-are-tasks
[`tokio::task`]: crate::task
[`spawn`]: crate::task::spawn()
[`JoinHandle`]: crate::task::JoinHandle
[blocking]: task/index.html#blocking-and-yielding

The [`tokio::sync`] module contains synchronization primitives to use when
needing to communicate or share data. These include:

* channels ([`oneshot`], [`mpsc`], [`watch`], and [`broadcast`]), for sending values
  between tasks,
* a non-blocking [`Mutex`], for controlling access to a shared, mutable
  value,
* an asynchronous [`Barrier`] type, for multiple tasks to synchronize before
  beginning a computation.

The `tokio::sync` module is present only when the "sync" feature flag is
enabled.

[`tokio::sync`]: crate::sync
[`Mutex`]: crate::sync::Mutex
[`Barrier`]: crate::sync::Barrier
[`oneshot`]: crate::sync::oneshot
[`mpsc`]: crate::sync::mpsc
[`watch`]: crate::sync::watch
[`broadcast`]: crate::sync::broadcast

The [`tokio::time`] module provides utilities for tracking time and
scheduling work. This includes functions for setting [timeouts][timeout] for
tasks, [sleeping][sleep] work to run in the future, or [repeating an operation at an
interval][interval].

In order to use `tokio::time`, the "time" feature flag must be enabled.

[`tokio::time`]: crate::time
[sleep]: crate::time::sleep()
[interval]: crate::time::interval()
[timeout]: crate::time::timeout()

Finally, Tokio provides a _runtime_ for executing asynchronous tasks. Most
applications can use the [`#[tokio::main]`][main] macro to run their code on the
Tokio runtime. However, this macro provides only basic configuration options. As
an alternative, the [`tokio::runtime`] module provides more powerful APIs for configuring
and managing runtimes. You should use that module if the `#[tokio::main]` macro doesn't
provide the functionality you need.

Using the runtime requires the "rt" or "rt-multi-thread" feature flags, to
enable the current-thread [single-threaded scheduler][rt] and the [multi-thread
scheduler][rt-multi-thread], respectively. See the [`runtime` module
documentation][rt-features] for details. In addition, the "macros" feature
flag enables the `#[tokio::main]` and `#[tokio::test]` attributes.

[main]: attr.main.html
[`tokio::runtime`]: crate::runtime
[`Builder`]: crate::runtime::Builder
[`Runtime`]: crate::runtime::Runtime
[rt]: runtime/index.html#current-thread-scheduler
[rt-multi-thread]: runtime/index.html#multi-thread-scheduler
[rt-features]: runtime/index.html#runtime-scheduler

## CPU-bound tasks and blocking code

Tokio is able to concurrently run many tasks on a few threads by repeatedly
swapping the currently running task on each thread. However, this kind of
swapping can only happen at `.await` points, so code that spends a long time
without reaching an `.await` will prevent other tasks from running. To
combat this, Tokio provides two kinds of threads: Core threads and blocking threads.

The core threads are where all asynchronous code runs, and Tokio will by default
spawn one for each CPU core. You can use the environment variable `TOKIO_WORKER_THREADS`
to override the default value.

The blocking threads are spawned on demand, can be used to run blocking code
that would otherwise block other tasks from running and are kept alive when
not used for a certain amount of time which can be configured with [`thread_keep_alive`].
Since it is not possible for Tokio to swap out blocking tasks, like it
can do with asynchronous code, the upper limit on the number of blocking
threads is very large. These limits can be configured on the [`Builder`].

To spawn a blocking task, you should use the [`spawn_blocking`] function.

[`Builder`]: crate::runtime::Builder
[`spawn_blocking`]: crate::task::spawn_blocking()
[`thread_keep_alive`]: crate::runtime::Builder::thread_keep_alive()

```
# #[cfg(not(target_family = "wasm"))]
# {
#[tokio::main]
async fn main() {
    // This is running on a core thread.

    let blocking_task = tokio::task::spawn_blocking(|| {
        // This is running on a blocking thread.
        // Blocking here is ok.
    });

    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
    blocking_task.await.unwrap();
}
# }
```

If your code is CPU-bound and you wish to limit the number of threads used
to run it, you should use a separate thread pool dedicated to CPU bound tasks.
For example, you could consider using the [rayon] library for CPU-bound
tasks. It is also possible to create an extra Tokio runtime dedicated to
CPU-bound tasks, but if you do this, you should be careful that the extra
runtime runs _only_ CPU-bound tasks, as IO-bound tasks on that runtime
will behave poorly.

Hint: If using rayon, you can use a [`oneshot`] channel to send the result back
to Tokio when the rayon task finishes.

[rayon]: https://docs.rs/rayon
[`oneshot`]: crate::sync::oneshot

## Asynchronous IO

As well as scheduling and running tasks, Tokio provides everything you need
to perform input and output asynchronously.

The [`tokio::io`] module provides Tokio's asynchronous core I/O primitives,
the [`AsyncRead`], [`AsyncWrite`], and [`AsyncBufRead`] traits. In addition,
when the "io-util" feature flag is enabled, it also provides combinators and
functions for working with these traits, forming as an asynchronous
counterpart to [`std::io`].

Tokio also includes APIs for performing various kinds of I/O and interacting
with the operating system asynchronously. These include:

* [`tokio::net`], which contains non-blocking versions of [TCP], [UDP], and
  [Unix Domain Sockets][UDS] (enabled by the "net" feature flag),
* [`tokio::fs`], similar to [`std::fs`] but for performing filesystem I/O
  asynchronously (enabled by the "fs" feature flag),
* [`tokio::signal`], for asynchronously handling Unix and Windows OS signals
  (enabled by the "signal" feature flag),
* [`tokio::process`], for spawning and managing child processes (enabled by
  the "process" feature flag).

[`tokio::io`]: crate::io
[`AsyncRead`]: crate::io::AsyncRead
[`AsyncWrite`]: crate::io::AsyncWrite
[`AsyncBufRead`]: crate::io::AsyncBufRead
[`std::io`]: std::io
[`tokio::net`]: crate::net
[TCP]: crate::net::tcp
[UDP]: crate::net::UdpSocket
[UDS]: crate::net::unix
[`tokio::fs`]: crate::fs
[`std::fs`]: std::fs
[`tokio::signal`]: crate::signal
[`tokio::process`]: crate::process

# Examples

A simple TCP echo server:

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
# }
```

# Feature flags

Tokio uses a set of [feature flags] to reduce the amount of compiled code. It
is possible to just enable certain features over others. By default, Tokio
does not enable any features but allows one to enable a subset for their use
case. Below is a list of the available feature flags. You may also notice
above each function, struct and trait there is listed one or more feature flags
that are required for that item to be used. If you are new to Tokio it is
recommended that you use the `full` feature flag which will enable all public APIs.
Beware though that this will pull in many extra dependencies that you may not
need.

- `full`: Enables all features listed below except `test-util` and unstable features.
- `rt`: Enables `tokio::spawn`, the current-thread scheduler,
  and non-scheduler utilities.
- `rt-multi-thread`: Enables the heavier, multi-threaded, work-stealing scheduler.
- `io-util`: Enables the IO based `Ext` traits.
- `io-std`: Enable `Stdout`, `Stdin` and `Stderr` types.
- `net`: Enables `tokio::net` types such as `TcpStream`, `UnixStream` and
  `UdpSocket`, as well as (on Unix-like systems) `AsyncFd` and (on
  FreeBSD) `PollAio`.
- `time`: Enables `tokio::time` types and allows the schedulers to enable
  the built-in timer.
- `process`: Enables `tokio::process` types.
- `macros`: Enables `#[tokio::main]` and `#[tokio::test]` macros.
- `sync`: Enables all `tokio::sync` types.
- `signal`: Enables all `tokio::signal` types.
- `fs`: Enables `tokio::fs` types.
- `test-util`: Enables testing based infrastructure for the Tokio runtime.
- `parking_lot`: As a potential optimization, use the [`parking_lot`] crate's
  synchronization primitives internally. Also, this
  dependency is necessary to construct some of our primitives
  in a `const` context. `MSRV` may increase according to the
  [`parking_lot`] release in use.

_Note: `AsyncRead` and `AsyncWrite` traits do not require any features and are
always available._

## Unstable features

Some feature flags are only available when specifying the `tokio_unstable` flag:

- `tracing`: Enables tracing events.
- `schedule-latency`: Allows measurement of task scheduling latencies.
- `io-uring`: Enables `io-uring` (Linux only).
- `taskdump`: Enables `taskdump` (Linux only).

Likewise, this flag enables access to unstable APIs.

This flag enables **unstable** features. The public API of these features
may break in 1.x releases. To enable these features, the `--cfg
tokio_unstable` argument must be passed to `rustc` when compiling. This
serves to explicitly opt-in to features which may break semver conventions,
since Cargo [does not yet directly support such opt-ins][unstable features].

You can specify it in your project's `.cargo/config.toml` file:

```toml
[build]
rustflags = ["--cfg", "tokio_unstable"]
```

<div class="warning">
The <code>[build]</code> section does <strong>not</strong> go in a
<code>Cargo.toml</code> file. Instead it must be placed in the Cargo config
file <code>.cargo/config.toml</code>.
</div>

Alternatively, you can specify it with an environment variable:

```sh
## Many *nix shells:
export RUSTFLAGS="--cfg tokio_unstable"
cargo build
```

```powershell
## Windows PowerShell:
$Env:RUSTFLAGS="--cfg tokio_unstable"
cargo build
```

[unstable features]: https://internals.rust-lang.org/t/feature-request-unstable-opt-in-non-transitive-crate-features/16193#why-not-a-crate-feature-2
[feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

# Supported platforms

Tokio currently guarantees support for the following platforms:

 * Linux
 * Windows
 * Android (API level 21)
 * macOS
 * iOS
 * FreeBSD

Tokio will continue to support these platforms in the future. However,
future releases may change requirements such as the minimum required libc
version on Linux, the API level on Android, or the supported FreeBSD
release.

Beyond the above platforms, Tokio is intended to work on all platforms
supported by the mio crate. You can find a longer list [in mio's
documentation][mio-supported]. However, these additional platforms may
become unsupported in the future.

Note that Wine is considered to be a different platform from Windows. See
mio's documentation for more information on Wine support.

[mio-supported]: https://crates.io/crates/mio#platforms

## `WASM` support

Tokio has some limited support for Wasm platforms.

Many of Tokio's feature flags are restricted on Wasm, and unsupported
combinations of feature flags will fail to build. However, all Wasm targets
can be built with the following features:

 * `sync`
 * `macros`
 * `io-util`
 * `rt`
 * `time`

Enabling any other feature (including `full`) will cause a compilation
failure. Furthermore, some operations available under these feature flags
may panic if they are unsupported. For example:

* Using timers will panic on Wasm targets that do not support blocking the
  thread.
* If the runtime becomes indefinitely idle (e.g., the program triggers a
  deadlock), then this will panic on most Wasm targets.
* Operations such as `spawn_blocking` that involve spawning threads will
  panic on Wasm targets that do not support threads.

All Wasm targets are still experimental, and breaking behavior changes can
occur in an effort to make Tokio use native Wasm operations. For instance,
the behavior of timers could be changed from panicking or blocking the
thread to starting a JavaScript timer. As another example, the
`spawn_blocking` method could be changed from starting a new thread to
creating a new cooperatively scheduled context of execution, if the Wasm
target supports such contexts. As a third example, the `#[tokio::main]` or
`#[tokio::test]` macros could be changed to work better with the Wasm
environment.

### `WASI` support

The `wasm32-wasip1` and `wasm32-wasip2` targets support the above features.
Timers work correctly as blocking the thread is supported.

Under the `tokio_unstable` flag, these targets support the use
of `tokio::net`. On `wasm32-wasip1`, not all methods are available on the
networking types as this target does not support the creation of new
sockets from within `WASM`. Because of this, sockets must currently be
created via the `FromRawFd` trait on `wasm32-wasip1`. The `wasm32-wasip2`
target does not have this limitation.

### Emscripten support

The `wasm32-unknown-emscripten` target supports the single-threaded runtime
with the `rt`, `time`, `sync`, `macros`, `fs`, `io-util`, `io-std`, and
`test-util` features. The `rt-multi-thread` feature is additionally
supported when building with Emscripten pthreads (`-pthread`). The `net`,
`process`, and `signal` features are not supported.

Emscripten's filesystem is synchronous, so `tokio::fs` and `io-std` run
their operations inline on the calling thread rather than on the blocking
pool, in pthreads builds too.

When the build links WebAssembly JavaScript Promise Integration (JSPI), a
wait that would block suspends on the host event loop rather than blocking.
This requires Emscripten 6.0.10 or later. Builds without JSPI will spin on
timed parks, and panic otherwise.

Suspension requires the current export to have been wrapped with
`WebAssembly.promising`. A wait from any other activation throws
`WebAssembly.SuspendError`. That is a foreign exception rather than a Rust
panic, so it can't be caught by `catch_unwind`.

## Modules

### [`tokio`](tokio.md)

*10 modules, 5 macros*

### [`fs::canonicalize`](fs/canonicalize.md)

*1 function*

### [`fs::copy`](fs/copy.md)

*1 function*

### [`fs::create_dir`](fs/create_dir.md)

*1 function*

### [`fs::create_dir_all`](fs/create_dir_all.md)

*1 function*

### [`fs::dir_builder`](fs/dir_builder.md)

*1 struct*

### [`fs::file`](fs/file.md)

*1 struct*

### [`fs::file::File`](fs/file/File.md)

*5 functions*

### [`fs::file_times`](fs/file_times.md)

*1 struct*

### [`fs::hard_link`](fs/hard_link.md)

*1 function*

### [`fs::metadata`](fs/metadata.md)

*1 function*

### [`fs::open_options`](fs/open_options.md)

*1 struct*

### [`fs::open_options::OpenOptions`](fs/open_options/OpenOptions.md)

*6 functions*

### [`fs::read`](fs/read.md)

*1 function*

### [`fs::read_dir`](fs/read_dir.md)

*1 function, 2 structs*

### [`fs::read_link`](fs/read_link.md)

*1 function*

### [`fs::read_to_string`](fs/read_to_string.md)

*1 function*

### [`fs::remove_dir`](fs/remove_dir.md)

*1 function*

### [`fs::remove_dir_all`](fs/remove_dir_all.md)

*1 function*

### [`fs::remove_file`](fs/remove_file.md)

*1 function*

### [`fs::rename`](fs/rename.md)

*1 function*

### [`fs::set_permissions`](fs/set_permissions.md)

*1 function*

### [`fs::symlink`](fs/symlink.md)

*1 function*

### [`fs::symlink_metadata`](fs/symlink_metadata.md)

*1 function*

### [`fs::try_exists`](fs/try_exists.md)

*1 function*

### [`fs::write`](fs/write.md)

*1 function*

### [`future::maybe_done`](future/maybe_done.md)

*1 enum, 1 function*

### [`io`](io.md)

*1 module*

### [`io::async_buf_read`](io/async_buf_read.md)

*1 trait*

### [`io::async_fd`](io/async_fd.md)

*5 structs*

### [`io::async_fd::AsyncFd`](io/async_fd/AsyncFd.md)

*15 functions*

### [`io::async_fd::AsyncFdReadyGuard`](io/async_fd/AsyncFdReadyGuard.md)

*3 functions*

### [`io::async_fd::AsyncFdReadyMutGuard`](io/async_fd/AsyncFdReadyMutGuard.md)

*2 functions*

### [`io::async_read`](io/async_read.md)

*1 trait*

### [`io::async_seek`](io/async_seek.md)

*1 trait*

### [`io::async_write`](io/async_write.md)

*1 trait*

### [`io::interest`](io/interest.md)

*1 struct*

### [`io::join`](io/join.md)

*1 function, 1 struct*

### [`io::read_buf`](io/read_buf.md)

*1 struct*

### [`io::read_buf::ReadBuf`](io/read_buf/ReadBuf.md)

*1 function*

### [`io::ready`](io/ready.md)

*1 struct*

### [`io::seek`](io/seek.md)

*1 struct*

### [`io::split`](io/split.md)

*1 function, 2 structs*

### [`io::split::ReadHalf`](io/split/ReadHalf.md)

*2 functions*

### [`io::stderr`](io/stderr.md)

*1 function, 1 struct*

### [`io::stdin`](io/stdin.md)

*1 function, 1 struct*

### [`io::stdout`](io/stdout.md)

*1 function, 1 struct*

### [`io::util::async_buf_read_ext`](io/util/async_buf_read_ext.md)

*1 trait*

### [`io::util::async_read_ext`](io/util/async_read_ext.md)

*1 trait*

### [`io::util::async_seek_ext`](io/util/async_seek_ext.md)

*1 trait*

### [`io::util::async_write_ext`](io/util/async_write_ext.md)

*1 trait*

### [`io::util::buf_reader`](io/util/buf_reader.md)

*1 struct*

### [`io::util::buf_stream`](io/util/buf_stream.md)

*1 struct*

### [`io::util::buf_writer`](io/util/buf_writer.md)

*1 struct*

### [`io::util::chain`](io/util/chain.md)

*1 struct*

### [`io::util::copy`](io/util/copy.md)

*1 function*

### [`io::util::copy_bidirectional`](io/util/copy_bidirectional.md)

*2 functions*

### [`io::util::copy_buf`](io/util/copy_buf.md)

*1 function*

### [`io::util::empty`](io/util/empty.md)

*1 function, 1 struct*

### [`io::util::fill_buf`](io/util/fill_buf.md)

*1 struct*

### [`io::util::flush`](io/util/flush.md)

*1 struct*

### [`io::util::lines`](io/util/lines.md)

*1 struct*

### [`io::util::lines::Lines`](io/util/lines/Lines.md)

*1 function*

### [`io::util::mem`](io/util/mem.md)

*2 functions, 2 structs*

### [`io::util::mem::SimplexStream`](io/util/mem/SimplexStream.md)

*1 function*

### [`io::util::read`](io/util/read.md)

*1 struct*

### [`io::util::read_buf`](io/util/read_buf.md)

*1 struct*

### [`io::util::read_exact`](io/util/read_exact.md)

*1 struct*

### [`io::util::read_line`](io/util/read_line.md)

*1 struct*

### [`io::util::read_to_end`](io/util/read_to_end.md)

*1 struct*

### [`io::util::read_to_string`](io/util/read_to_string.md)

*1 struct*

### [`io::util::read_until`](io/util/read_until.md)

*1 struct*

### [`io::util::repeat`](io/util/repeat.md)

*1 function, 1 struct*

### [`io::util::shutdown`](io/util/shutdown.md)

*1 struct*

### [`io::util::sink`](io/util/sink.md)

*1 function, 1 struct*

### [`io::util::split`](io/util/split.md)

*1 struct*

### [`io::util::take`](io/util/take.md)

*1 struct*

### [`io::util::write`](io/util/write.md)

*1 struct*

### [`io::util::write_all`](io/util/write_all.md)

*1 struct*

### [`io::util::write_all_buf`](io/util/write_all_buf.md)

*1 struct*

### [`io::util::write_buf`](io/util/write_buf.md)

*1 struct*

### [`io::util::write_vectored`](io/util/write_vectored.md)

*1 struct*

### [`net`](net.md)

*2 modules*

### [`net::addr`](net/addr.md)

*1 trait*

### [`net::addr::sealed`](net/addr/sealed.md)

*1 struct*

### [`net::lookup_host`](net/lookup_host.md)

*1 function*

### [`net::tcp::listener`](net/tcp/listener.md)

*1 struct*

### [`net::tcp::listener::TcpListener`](net/tcp/listener/TcpListener.md)

*3 functions*

### [`net::tcp::socket`](net/tcp/socket.md)

*1 struct*

### [`net::tcp::socket::TcpSocket`](net/tcp/socket/TcpSocket.md)

*5 functions*

### [`net::tcp::split`](net/tcp/split.md)

*2 structs*

### [`net::tcp::split::ReadHalf`](net/tcp/split/ReadHalf.md)

*3 functions*

### [`net::tcp::split::WriteHalf`](net/tcp/split/WriteHalf.md)

*2 functions*

### [`net::tcp::split_owned`](net/tcp/split_owned.md)

*3 structs*

### [`net::tcp::split_owned::OwnedReadHalf`](net/tcp/split_owned/OwnedReadHalf.md)

*3 functions*

### [`net::tcp::split_owned::OwnedWriteHalf`](net/tcp/split_owned/OwnedWriteHalf.md)

*2 functions*

### [`net::tcp::stream`](net/tcp/stream.md)

*1 struct*

### [`net::tcp::stream::TcpStream`](net/tcp/stream/TcpStream.md)

*17 functions*

### [`net::udp`](net/udp.md)

*1 struct*

### [`net::udp::UdpSocket`](net/udp/UdpSocket.md)

*24 functions*

### [`net::unix`](net/unix.md)

*1 module, 3 type aliases*

### [`net::unix::listener`](net/unix/listener.md)

*1 struct*

### [`net::unix::listener::UnixListener`](net/unix/listener/UnixListener.md)

*3 functions*

### [`net::unix::pipe`](net/unix/pipe.md)

*1 function, 3 structs*

### [`net::unix::pipe::OpenOptions`](net/unix/pipe/OpenOptions.md)

*3 functions*

### [`net::unix::pipe::Receiver`](net/unix/pipe/Receiver.md)

*3 functions*

### [`net::unix::pipe::Sender`](net/unix/pipe/Sender.md)

*3 functions*

### [`net::unix::socket`](net/unix/socket.md)

*1 struct*

### [`net::unix::socket::UnixSocket`](net/unix/socket/UnixSocket.md)

*2 functions*

### [`net::unix::socketaddr`](net/unix/socketaddr.md)

*1 struct*

### [`net::unix::split`](net/unix/split.md)

*2 structs*

### [`net::unix::split::ReadHalf`](net/unix/split/ReadHalf.md)

*3 functions*

### [`net::unix::split::WriteHalf`](net/unix/split/WriteHalf.md)

*2 functions*

### [`net::unix::split_owned`](net/unix/split_owned.md)

*3 structs*

### [`net::unix::split_owned::OwnedReadHalf`](net/unix/split_owned/OwnedReadHalf.md)

*3 functions*

### [`net::unix::split_owned::OwnedWriteHalf`](net/unix/split_owned/OwnedWriteHalf.md)

*2 functions*

### [`net::unix::stream`](net/unix/stream.md)

*1 struct*

### [`net::unix::stream::UnixStream`](net/unix/stream/UnixStream.md)

*9 functions*

### [`net::unix::ucred`](net/unix/ucred.md)

*1 struct*

### [`process`](process.md)

*5 structs*

### [`process::Child`](process/Child.md)

*2 functions*

### [`process::Command`](process/Command.md)

*6 functions*

### [`runtime`](runtime.md)

*1 function*

### [`runtime::builder`](runtime/builder.md)

*1 struct*

### [`runtime::builder::Builder`](runtime/builder/Builder.md)

*13 functions*

### [`runtime::handle`](runtime/handle.md)

*3 structs*

### [`runtime::handle::Handle`](runtime/handle/Handle.md)

*5 functions*

### [`runtime::id`](runtime/id.md)

*1 struct*

### [`runtime::local_runtime::options`](runtime/local_runtime/options.md)

*1 struct*

### [`runtime::local_runtime::runtime`](runtime/local_runtime/runtime.md)

*1 struct*

### [`runtime::metrics::runtime`](runtime/metrics/runtime.md)

*1 struct*

### [`runtime::metrics::runtime::RuntimeMetrics`](runtime/metrics/runtime/RuntimeMetrics.md)

*1 function*

### [`runtime::runtime`](runtime/runtime.md)

*1 enum, 1 function, 1 struct*

### [`runtime::runtime::Runtime`](runtime/runtime/Runtime.md)

*9 functions*

### [`runtime::task::abort`](runtime/task/abort.md)

*1 struct*

### [`runtime::task::abort::AbortHandle`](runtime/task/abort/AbortHandle.md)

*2 functions*

### [`runtime::task::error`](runtime/task/error.md)

*1 struct*

### [`runtime::task::error::JoinError`](runtime/task/error/JoinError.md)

*2 functions*

### [`runtime::task::id`](runtime/task/id.md)

*1 struct, 2 functions*

### [`runtime::task::join`](runtime/task/join.md)

*1 struct*

### [`runtime::task::join::JoinHandle`](runtime/task/join/JoinHandle.md)

*3 functions*

### [`signal`](signal.md)

*1 module*

### [`signal::ctrl_c`](signal/ctrl_c.md)

*1 function*

### [`signal::unix`](signal/unix.md)

*1 function, 2 structs*

### [`signal::unix::Signal`](signal/unix/Signal.md)

*1 function*

### [`sync`](sync.md)

*5 modules*

### [`sync::barrier`](sync/barrier.md)

*2 structs*

### [`sync::barrier::Barrier`](sync/barrier/Barrier.md)

*1 function*

### [`sync::barrier::BarrierWaitResult`](sync/barrier/BarrierWaitResult.md)

*1 function*

### [`sync::batch_semaphore`](sync/batch_semaphore.md)

*1 enum, 1 struct*

### [`sync::broadcast`](sync/broadcast.md)

*1 function, 1 module, 3 structs*

### [`sync::broadcast::Receiver`](sync/broadcast/Receiver.md)

*3 functions*

### [`sync::broadcast::Sender`](sync/broadcast/Sender.md)

*2 functions*

### [`sync::broadcast::WeakSender`](sync/broadcast/WeakSender.md)

*1 function*

### [`sync::broadcast::error`](sync/broadcast/error.md)

*1 struct, 2 enums*

### [`sync::mpsc`](sync/mpsc.md)

*1 module*

### [`sync::mpsc::bounded`](sync/mpsc/bounded.md)

*1 function, 6 structs*

### [`sync::mpsc::bounded::OwnedPermit`](sync/mpsc/bounded/OwnedPermit.md)

*1 function*

### [`sync::mpsc::bounded::Permit`](sync/mpsc/bounded/Permit.md)

*1 function*

### [`sync::mpsc::bounded::Receiver`](sync/mpsc/bounded/Receiver.md)

*8 functions*

### [`sync::mpsc::bounded::Sender`](sync/mpsc/bounded/Sender.md)

*12 functions*

### [`sync::mpsc::bounded::WeakSender`](sync/mpsc/bounded/WeakSender.md)

*1 function*

### [`sync::mpsc::error`](sync/mpsc/error.md)

*1 struct, 3 enums*

### [`sync::mpsc::unbounded`](sync/mpsc/unbounded.md)

*1 function, 3 structs*

### [`sync::mpsc::unbounded::UnboundedReceiver`](sync/mpsc/unbounded/UnboundedReceiver.md)

*5 functions*

### [`sync::mpsc::unbounded::WeakUnboundedSender`](sync/mpsc/unbounded/WeakUnboundedSender.md)

*1 function*

### [`sync::mutex`](sync/mutex.md)

*6 structs*

### [`sync::mutex::Mutex`](sync/mutex/Mutex.md)

*4 functions*

### [`sync::mutex::MutexGuard`](sync/mutex/MutexGuard.md)

*1 function*

### [`sync::mutex::OwnedMutexGuard`](sync/mutex/OwnedMutexGuard.md)

*1 function*

### [`sync::notify`](sync/notify.md)

*3 structs*

### [`sync::notify::Notified`](sync/notify/Notified.md)

*1 function*

### [`sync::notify::Notify`](sync/notify/Notify.md)

*5 functions*

### [`sync::once_cell`](sync/once_cell.md)

*1 enum, 1 struct*

### [`sync::once_cell::OnceCell`](sync/once_cell/OnceCell.md)

*3 functions*

### [`sync::oneshot`](sync/oneshot.md)

*1 function, 1 module, 2 structs*

### [`sync::oneshot::Receiver`](sync/oneshot/Receiver.md)

*3 functions*

### [`sync::oneshot::error`](sync/oneshot/error.md)

*1 enum, 1 struct*

### [`sync::rwlock`](sync/rwlock.md)

*1 struct*

### [`sync::rwlock::RwLock`](sync/rwlock/RwLock.md)

*7 functions*

### [`sync::rwlock::owned_read_guard`](sync/rwlock/owned_read_guard.md)

*1 struct*

### [`sync::rwlock::owned_write_guard`](sync/rwlock/owned_write_guard.md)

*1 struct*

### [`sync::rwlock::owned_write_guard::OwnedRwLockWriteGuard`](sync/rwlock/owned_write_guard/OwnedRwLockWriteGuard.md)

*1 function*

### [`sync::rwlock::owned_write_guard_mapped`](sync/rwlock/owned_write_guard_mapped.md)

*1 struct*

### [`sync::rwlock::read_guard`](sync/rwlock/read_guard.md)

*1 struct*

### [`sync::rwlock::write_guard`](sync/rwlock/write_guard.md)

*1 struct*

### [`sync::rwlock::write_guard::RwLockWriteGuard`](sync/rwlock/write_guard/RwLockWriteGuard.md)

*2 functions*

### [`sync::rwlock::write_guard_mapped`](sync/rwlock/write_guard_mapped.md)

*1 struct*

### [`sync::semaphore`](sync/semaphore.md)

*3 structs*

### [`sync::semaphore::Semaphore`](sync/semaphore/Semaphore.md)

*12 functions*

### [`sync::semaphore::SemaphorePermit`](sync/semaphore/SemaphorePermit.md)

*1 function*

### [`sync::set_once`](sync/set_once.md)

*2 structs*

### [`sync::set_once::SetOnce`](sync/set_once/SetOnce.md)

*3 functions*

### [`sync::watch`](sync/watch.md)

*1 function, 1 module, 3 structs*

### [`sync::watch::Receiver`](sync/watch/Receiver.md)

*4 functions*

### [`sync::watch::Sender`](sync/watch/Sender.md)

*6 functions*

### [`sync::watch::error`](sync/watch/error.md)

*2 structs*

### [`task`](task.md)

*2 modules*

### [`task::blocking`](task/blocking.md)

*2 functions*

### [`task::coop`](task/coop.md)

*2 structs, 3 functions*

### [`task::coop::RestoreOnPending`](task/coop/RestoreOnPending.md)

*1 function*

### [`task::coop::consume_budget`](task/coop/consume_budget.md)

*1 function*

### [`task::coop::unconstrained`](task/coop/unconstrained.md)

*1 function, 1 struct*

### [`task::join_set`](task/join_set.md)

*1 struct*

### [`task::join_set::JoinSet`](task/join_set/JoinSet.md)

*4 functions*

### [`task::local`](task/local.md)

*1 function, 2 structs*

### [`task::spawn`](task/spawn.md)

*1 function*

### [`task::task_local`](task/task_local.md)

*3 structs*

### [`task::task_local::LocalKey`](task/task_local/LocalKey.md)

*3 functions*

### [`task::yield_now`](task/yield_now.md)

*1 function*

### [`time`](time.md)

*1 module*

### [`time::error`](time/error.md)

*2 structs*

### [`time::instant`](time/instant.md)

*1 struct*

### [`time::interval`](time/interval.md)

*1 enum, 1 struct, 2 functions*

### [`time::interval::Interval`](time/interval/Interval.md)

*2 functions*

### [`time::sleep`](time/sleep.md)

*1 struct, 2 functions*

### [`time::timeout`](time/timeout.md)

*1 struct, 2 functions*

### [`time::timeout::Timeout`](time/timeout/Timeout.md)

*1 function*

