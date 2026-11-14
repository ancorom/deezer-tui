**tokio**

# Module: tokio

## Contents

**Modules**

- [`fs`](#fs) - Asynchronous file utilities.
- [`io`](#io) - Traits, helpers, and type definitions for asynchronous I/O functionality.
- [`net`](#net) - TCP/UDP/Unix bindings for `tokio`.
- [`process`](#process) - An implementation of asynchronous process management for Tokio.
- [`runtime`](#runtime) - The Tokio runtime.
- [`signal`](#signal) - Asynchronous signal handling for Tokio.
- [`stream`](#stream) - Due to the `Stream` trait's inclusion in `std` landing later than Tokio's 1.0
- [`sync`](#sync) - Synchronization primitives for use in asynchronous contexts.
- [`task`](#task) - Asynchronous green-threads.
- [`time`](#time) - Utilities for tracking time.

**Macros**

- [`join`](#join) - Waits on multiple concurrent branches, returning when **all** branches
- [`pin`](#pin) - Pins a value on the stack.
- [`select`](#select) - Waits on multiple concurrent branches, returning when the **first** branch
- [`task_local`](#task_local) - Declares a new task-local key of type [`tokio::task::LocalKey`].
- [`try_join`](#try_join) - Waits on multiple concurrent branches, returning when **all** branches

---

## Module: fs

Asynchronous file utilities.

This module contains utility methods for working with the file system
asynchronously. This includes reading/writing to files, and working with
directories.

Be aware that most operating systems do not provide asynchronous file system
APIs. Because of that, Tokio will use ordinary blocking file operations
behind the scenes. This is done using the [`spawn_blocking`] threadpool to
run them in the background.

The `tokio::fs` module should only be used for ordinary files. Trying to use
it with e.g., a named pipe on Linux can result in surprising behavior,
such as hangs during runtime shutdown. For special files, you should use a
dedicated type such as [`tokio::net::unix::pipe`] or [`AsyncFd`] instead.

Tokio currently uses [`spawn_blocking`] on all platforms, but also uses
io_uring for file operations on Linux when compiled with `tokio_unstable`.
Operations that cannot be cancelled with [`spawn_blocking`] may possibly
be cancelled with io_uring.

# Cancellation

Cancelling a future from this module will stop waiting for the result, but
the underlying blocking operation will continue to run on the thread pool.
For example, cancelling a [`write()`] future after it has been
polled will still result in the data being written to disk.

# Usage

The easiest way to use this module is to use the utility functions that
operate on entire files:

 * [`tokio::fs::read`](fn@crate::fs::read)
 * [`tokio::fs::read_to_string`](fn@crate::fs::read_to_string)
 * [`tokio::fs::write`](fn@crate::fs::write)

The two `read` functions reads the entire file and returns its contents.
The `write` function takes the contents of the file and writes those
contents to the file. It overwrites the existing file, if any.

For example, to read the file:

```
# async fn dox() -> std::io::Result<()> {
let contents = tokio::fs::read_to_string("my_file.txt").await?;

println!("File has {} lines.", contents.lines().count());
# Ok(())
# }
```

To overwrite the file:

```
# async fn dox() -> std::io::Result<()> {
let contents = "First line.\nSecond line.\nThird line.\n";

tokio::fs::write("my_file.txt", contents.as_bytes()).await?;
# Ok(())
# }
```

## Using `File`

The main type for interacting with files is [`File`]. It can be used to read
from and write to a given file. This is done using the [`AsyncRead`] and
[`AsyncWrite`] traits. This type is generally used when you want to do
something more complex than just reading or writing the entire contents in
one go.

**Note:** It is important to use [`flush`] when writing to a Tokio
[`File`]. This is because calls to `write` will return before the write has
finished, and [`flush`] will wait for the write to finish. (The write will
happen even if you don't flush; it will just happen later.) This is
different from [`std::fs::File`], and is due to the fact that `File` uses
`spawn_blocking` behind the scenes.

For example, to count the number of lines in a file without loading the
entire file into memory:

```no_run
use tokio::fs::File;
use tokio::io::AsyncReadExt;

# async fn dox() -> std::io::Result<()> {
let mut file = File::open("my_file.txt").await?;

let mut chunk = vec![0; 4096];
let mut number_of_lines = 0;
loop {
    let len = file.read(&mut chunk).await?;
    if len == 0 {
        // Length of zero means end of file.
        break;
    }
    for &b in &chunk[..len] {
        if b == b'\n' {
            number_of_lines += 1;
        }
    }
}

println!("File has {} lines.", number_of_lines);
# Ok(())
# }
```

For example, to write a file line-by-line:

```no_run
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

# async fn dox() -> std::io::Result<()> {
let mut file = File::create("my_file.txt").await?;

file.write_all(b"First line.\n").await?;
file.write_all(b"Second line.\n").await?;
file.write_all(b"Third line.\n").await?;

// Remember to call `flush` after writing!
file.flush().await?;
# Ok(())
# }
```

## Tuning your file IO

Tokio's file uses [`spawn_blocking`] behind the scenes, and this has serious
performance consequences. To get good performance with file IO on Tokio, it
is recommended to batch your operations into as few `spawn_blocking` calls
as possible.

One example of this difference can be seen by comparing the two reading
examples above. The first example uses [`tokio::fs::read`], which reads the
entire file in a single `spawn_blocking` call, and then returns it. The
second example will read the file in chunks using many `spawn_blocking`
calls. This means that the second example will most likely be more expensive
for large files. (Of course, using chunks may be necessary for very large
files that don't fit in memory.)

The following examples will show some strategies for this:

When creating a file, write the data to a `String` or `Vec<u8>` and then
write the entire file in a single `spawn_blocking` call with
`tokio::fs::write`.

```no_run
# async fn dox() -> std::io::Result<()> {
let mut contents = String::new();

contents.push_str("First line.\n");
contents.push_str("Second line.\n");
contents.push_str("Third line.\n");

tokio::fs::write("my_file.txt", contents.as_bytes()).await?;
# Ok(())
# }
```

Use [`BufReader`] and [`BufWriter`] to buffer many small reads or writes
into a few large ones. This example will most likely only perform one
`spawn_blocking` call.

```no_run
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

# async fn dox() -> std::io::Result<()> {
let mut file = BufWriter::new(File::create("my_file.txt").await?);

file.write_all(b"First line.\n").await?;
file.write_all(b"Second line.\n").await?;
file.write_all(b"Third line.\n").await?;

// Due to the BufWriter, the actual write and spawn_blocking
// call happens when you flush.
file.flush().await?;
# Ok(())
# }
```

Manually use [`std::fs`] inside [`spawn_blocking`].

```no_run
use std::fs::File;
use std::io::{self, Write};
use tokio::task::spawn_blocking;

# async fn dox() -> std::io::Result<()> {
spawn_blocking(move || {
    let mut file = File::create("my_file.txt")?;

    file.write_all(b"First line.\n")?;
    file.write_all(b"Second line.\n")?;
    file.write_all(b"Third line.\n")?;

    // Unlike Tokio's file, the std::fs file does
    // not need flush.

    io::Result::Ok(())
}).await.unwrap()?;
# Ok(())
# }
```

It's also good to be aware of [`File::set_max_buf_size`], which controls the
maximum amount of bytes that Tokio's [`File`] will read or write in a single
[`spawn_blocking`] call. The default is two megabytes, but this is subject
to change.

[`spawn_blocking`]: fn@crate::task::spawn_blocking
[`AsyncRead`]: trait@crate::io::AsyncRead
[`AsyncWrite`]: trait@crate::io::AsyncWrite
[`BufReader`]: struct@crate::io::BufReader
[`BufWriter`]: struct@crate::io::BufWriter
[`tokio::net::unix::pipe`]: crate::net::unix::pipe
[`AsyncFd`]: crate::io::unix::AsyncFd
[`flush`]: crate::io::AsyncWriteExt::flush
[`tokio::fs::read`]: fn@crate::fs::read



## Module: io

Traits, helpers, and type definitions for asynchronous I/O functionality.

This module is the asynchronous version of `std::io`. Primarily, it
defines two traits, [`AsyncRead`] and [`AsyncWrite`], which are asynchronous
versions of the [`Read`] and [`Write`] traits in the standard library.

# `AsyncRead` and `AsyncWrite`

Like the standard library's [`Read`] and [`Write`] traits, [`AsyncRead`] and
[`AsyncWrite`] provide the most general interface for reading and writing
input and output. Unlike the standard library's traits, however, they are
_asynchronous_ &mdash; meaning that reading from or writing to a `tokio::io`
type will _yield_ to the Tokio scheduler when IO is not ready, rather than
blocking. This allows other tasks to run while waiting on IO.

Another difference is that `AsyncRead` and `AsyncWrite` only contain
core methods needed to provide asynchronous reading and writing
functionality. Instead, utility methods are defined in the [`AsyncReadExt`]
and [`AsyncWriteExt`] extension traits. These traits are automatically
implemented for all values that implement `AsyncRead` and `AsyncWrite`
respectively.

End users will rarely interact directly with `AsyncRead` and
`AsyncWrite`. Instead, they will use the async functions defined in the
extension traits. Library authors are expected to implement `AsyncRead`
and `AsyncWrite` in order to provide types that behave like byte streams.

Even with these differences, Tokio's `AsyncRead` and `AsyncWrite` traits
can be used in almost exactly the same manner as the standard library's
`Read` and `Write`. Most types in the standard library that implement `Read`
and `Write` have asynchronous equivalents in `tokio` that implement
`AsyncRead` and `AsyncWrite`, such as [`File`] and [`TcpStream`].

For example, the standard library documentation introduces `Read` by
[demonstrating][std_example] reading some bytes from a [`std::fs::File`]. We
can do the same with [`tokio::fs::File`][`File`]:

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = f.read(&mut buffer).await?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}
# }
```

[`File`]: crate::fs::File
[`TcpStream`]: crate::net::TcpStream
[`std::fs::File`]: std::fs::File
[std_example]: std::io#read-and-write

## Buffered Readers and Writers

Byte-based interfaces are unwieldy and can be inefficient, as we'd need to be
making near-constant calls to the operating system. To help with this,
`std::io` comes with [support for _buffered_ readers and writers][stdbuf],
and therefore, `tokio::io` does as well.

Tokio provides an async version of the [`std::io::BufRead`] trait,
[`AsyncBufRead`]; and async [`BufReader`] and [`BufWriter`] structs, which
wrap readers and writers. These wrappers use a buffer, reducing the number
of calls and providing nicer methods for accessing exactly what you want.

For example, [`BufReader`] works with the [`AsyncBufRead`] trait to add
extra methods to any async reader:

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::io::{self, BufReader, AsyncBufReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let f = File::open("foo.txt").await?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // read a line into buffer
    reader.read_line(&mut buffer).await?;

    println!("{}", buffer);
    Ok(())
}
# }
```

[`BufWriter`] doesn't add any new ways of writing; it just buffers every call
to [`write`](crate::io::AsyncWriteExt::write). However, you **must** flush
[`BufWriter`] to ensure that any buffered data is written.

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::io::{self, BufWriter, AsyncWriteExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let f = File::create("foo.txt").await?;
    {
        let mut writer = BufWriter::new(f);

        // Write a byte to the buffer.
        writer.write(&[42u8]).await?;

        // Flush the buffer before it goes out of scope.
        writer.flush().await?;

    } // Unless flushed or shut down, the contents of the buffer is discarded on drop.

    Ok(())
}
# }
```

[stdbuf]: std::io#bufreader-and-bufwriter
[`std::io::BufRead`]: std::io::BufRead
[`AsyncBufRead`]: crate::io::AsyncBufRead
[`BufReader`]: crate::io::BufReader
[`BufWriter`]: crate::io::BufWriter

## Implementing `AsyncRead` and `AsyncWrite`

Because they are traits, we can implement [`AsyncRead`] and [`AsyncWrite`] for
our own types, as well. Note that these traits must only be implemented for
non-blocking I/O types that integrate with the futures type system. In
other words, these types must never block the thread, and instead the
current task is notified when the I/O resource is ready.

## Conversion to and from Stream/Sink

It is often convenient to encapsulate the reading and writing of bytes in a
[`Stream`] or [`Sink`] of data.

Tokio provides simple wrappers for converting [`AsyncRead`] to [`Stream`]
and vice-versa in the [tokio-util] crate, see [`ReaderStream`] and
[`StreamReader`].

There are also utility traits that abstract the asynchronous buffering
necessary to write your own adaptors for encoding and decoding bytes to/from
your structured data, allowing to transform something that implements
[`AsyncRead`]/[`AsyncWrite`] into a [`Stream`]/[`Sink`], see [`Decoder`] and
[`Encoder`] in the [tokio-util::codec] module.

[tokio-util]: https://docs.rs/tokio-util
[tokio-util::codec]: https://docs.rs/tokio-util/latest/tokio_util/codec/index.html

# Standard input and output

Tokio provides asynchronous APIs to standard [input], [output], and [error].
These APIs are very similar to the ones provided by `std`, but they also
implement [`AsyncRead`] and [`AsyncWrite`].

Note that the standard input / output APIs  **must** be used from the
context of the Tokio runtime, as they require Tokio-specific features to
function. Calling these functions outside of a Tokio runtime will panic.

[input]: fn@stdin
[output]: fn@stdout
[error]: fn@stderr

# `std` re-exports

Additionally, [`Error`], [`ErrorKind`], [`Result`], and [`SeekFrom`] are
re-exported from `std::io` for ease of use.

[`AsyncRead`]: trait@AsyncRead
[`AsyncWrite`]: trait@AsyncWrite
[`AsyncReadExt`]: trait@AsyncReadExt
[`AsyncWriteExt`]: trait@AsyncWriteExt
["codec"]: https://docs.rs/tokio-util/latest/tokio_util/codec/index.html
[`Encoder`]: https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Encoder.html
[`Decoder`]: https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Decoder.html
[`ReaderStream`]: https://docs.rs/tokio-util/latest/tokio_util/io/struct.ReaderStream.html
[`StreamReader`]: https://docs.rs/tokio-util/latest/tokio_util/io/struct.StreamReader.html
[`Error`]: struct@Error
[`ErrorKind`]: enum@ErrorKind
[`Result`]: type@Result
[`Read`]: std::io::Read
[`SeekFrom`]: enum@SeekFrom
[`Sink`]: https://docs.rs/futures/0.3/futures/sink/trait.Sink.html
[`Stream`]: https://docs.rs/futures/0.3/futures/stream/trait.Stream.html
[`Write`]: std::io::Write



## tokio::join

*Declarative Macro*

Waits on multiple concurrent branches, returning when **all** branches
complete.

The `join!` macro must be used inside of async functions, closures, and
blocks.

The `join!` macro takes a list of async expressions and evaluates them
concurrently on the same task. Each async expression evaluates to a future
and the futures from each expression are multiplexed on the current task.

When working with async expressions returning `Result`, `join!` will wait
for **all** branches complete regardless if any complete with `Err`. Use
[`try_join!`] to return early when `Err` is encountered.

[`try_join!`]: crate::try_join

# Notes

The supplied futures are stored inline and do not require allocating a
`Vec`.

## Runtime characteristics

By running all async expressions on the current task, the expressions are
able to run **concurrently** but not in **parallel**. This means all
expressions are run on the same thread and if one branch blocks the thread,
all other expressions will be unable to continue. If parallelism is
required, spawn each async expression using [`tokio::spawn`] and pass the
join handle to `join!`.

[`tokio::spawn`]: crate::spawn

## Fairness

By default, `join!`'s generated future rotates which contained
future is polled first whenever it is woken.

This behavior can be overridden by adding `biased;` to the beginning of the
macro usage. See the examples for details. This will cause `join` to poll
the futures in the order they appear from top to bottom.

You may want this if your futures may interact in a way where known polling order is significant.

But there is an important caveat to this mode. It becomes your responsibility
to ensure that the polling order of your futures is fair. If for example you
are joining a stream and a shutdown future, and the stream has a
huge volume of messages that takes a long time to finish processing per poll, you should
place the shutdown future earlier in the `join!` list to ensure that it is
always polled, and will not be delayed due to the stream future taking a long time to return
`Poll::Pending`.

# Examples

Basic join with two branches

```
async fn do_stuff_async() {
    // async work
}

async fn more_async_work() {
    // more here
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (first, second) = tokio::join!(
    do_stuff_async(),
    more_async_work());

// do something with the values
# }
```

Using the `biased;` mode to control polling order.

```
# #[cfg(not(target_family = "wasm"))]
# {
async fn do_stuff_async() {
    // async work
}

async fn more_async_work() {
    // more here
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (first, second) = tokio::join!(
    biased;
    do_stuff_async(),
    more_async_work()
);

// do something with the values
# }
# }
```

```rust
macro_rules! join {
    ($(biased;)? $($future:expr),*) => { ... };
}
```



## Module: net

TCP/UDP/Unix bindings for `tokio`.

This module contains the TCP/UDP/Unix networking types, similar to the standard
library, which can be used to implement networking protocols.

# Organization

* [`TcpListener`] and [`TcpStream`] provide functionality for communication over TCP
* [`UdpSocket`] provides functionality for communication over UDP
* [`UnixListener`] and [`UnixStream`] provide functionality for communication over a
  Unix Domain Stream Socket **(available on Unix only)**
* [`UnixDatagram`] provides functionality for communication
  over Unix Domain Datagram Socket **(available on Unix only)**
* [`tokio::net::unix::pipe`] for FIFO pipes **(available on Unix only)**
* [`tokio::net::windows::named_pipe`] for Named Pipes **(available on Windows only)**

For IO resources not available in `tokio::net`, you can use [`AsyncFd`].

[`TcpListener`]: TcpListener
[`TcpStream`]: TcpStream
[`UdpSocket`]: UdpSocket
[`UnixListener`]: UnixListener
[`UnixStream`]: UnixStream
[`UnixDatagram`]: UnixDatagram
[`tokio::net::unix::pipe`]: unix::pipe
[`tokio::net::windows::named_pipe`]: windows::named_pipe
[`AsyncFd`]: crate::io::unix::AsyncFd



## tokio::pin

*Declarative Macro*

Pins a value on the stack.

Calls to `async fn` return anonymous [`Future`] values that are `!Unpin`.
These values must be pinned before they can be polled. Calling `.await` will
handle this, but consumes the future. If it is required to call `.await` on
a `&mut _` reference, the caller is responsible for pinning the future.

Pinning may be done by allocating with [`Box::pin`] or by using the stack
with the `pin!` macro.

The following will **fail to compile**:

```compile_fail
async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut future = my_async_fn();
    (&mut future).await;
}
```

To make this work requires pinning:

```
use tokio::pin;

async fn my_async_fn() {
    // async logic here
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let future = my_async_fn();
pin!(future);

(&mut future).await;
# }
```

Pinning is useful when using `select!` and stream operators that require `T:
Stream + Unpin`.

[`Future`]: trait@std::future::Future
[`Box::pin`]: std::boxed::Box::pin

# Usage

The `pin!` macro takes **identifiers** as arguments. It does **not** work
with expressions.

The following does not compile as an expression is passed to `pin!`.

```compile_fail
async fn my_async_fn() {
    // async logic here
}

#[tokio::main]
async fn main() {
    let mut future = pin!(my_async_fn());
    (&mut future).await;
}
```

# Examples

Using with select:

```
use tokio::{pin, select};
use tokio_stream::{self as stream, StreamExt};

async fn my_async_fn() {
    // async logic here
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut stream = stream::iter(vec![1, 2, 3, 4]);

let future = my_async_fn();
pin!(future);

loop {
    select! {
        _ = &mut future => {
            // Stop looping `future` will be polled after completion
            break;
        }
        Some(val) = stream.next() => {
            println!("got value = {}", val);
        }
    }
}
# }
```

Because assigning to a variable followed by pinning is common, there is also
a variant of the macro that supports doing both in one go.

```
use tokio::{pin, select};

async fn my_async_fn() {
    // async logic here
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
pin! {
    let future1 = my_async_fn();
    let future2 = my_async_fn();
}

select! {
    _ = &mut future1 => {}
    _ = &mut future2 => {}
}
# }
```

```rust
macro_rules! pin {
    ($($x:ident),*) => { ... };
    ($(
            let $x:ident = $init:expr;
    )*) => { ... };
}
```



## Module: process

An implementation of asynchronous process management for Tokio.

This module provides a [`Command`] struct that imitates the interface of the
[`std::process::Command`] type in the standard library, but provides asynchronous versions of
functions that create processes. These functions (`spawn`, `status`, `output` and their
variants) return "future aware" types that interoperate with Tokio. The asynchronous process
support is provided through signal handling on Unix and system APIs on Windows.

[`std::process::Command`]: std::process::Command

# Examples

Here's an example program which will spawn `echo hello world` and then wait
for it complete.

```no_run
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The usage is similar as with the standard library's `Command` type
    let mut child = Command::new("echo")
        .arg("hello")
        .arg("world")
        .spawn()
        .expect("failed to spawn");

    // Await until the command completes
    let status = child.wait().await?;
    println!("the command exited with: {}", status);
    Ok(())
}
```

Next, let's take a look at an example where we not only spawn `echo hello
world` but we also capture its output.

```no_run
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Like above, but use `output` which returns a future instead of
    // immediately returning the `Child`.
    let output = Command::new("echo").arg("hello").arg("world")
                        .output();

    let output = output.await?;

    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello world\n");
    Ok(())
}
```

We can also read input line by line.

```no_run
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("cat");

    // Specify that we want the command's standard output piped back to us.
    // By default, standard input/output/error will be inherited from the
    // current process (for example, this means that standard input will
    // come from the keyboard and standard output/error will go directly to
    // the terminal if this process is invoked from the command line).
    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn()
        .expect("failed to spawn command");

    let stdout = child.stdout.take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    // Ensure the child process is spawned in the runtime so it can
    // make progress on its own while we await for any output.
    tokio::spawn(async move {
        let status = child.wait().await
            .expect("child process encountered an error");

        println!("child status was: {}", status);
    });

    while let Some(line) = reader.next_line().await? {
        println!("Line: {}", line);
    }

    Ok(())
}
```

Here is another example using `sort` writing into the child process
standard input, capturing the output of the sorted text.

```no_run
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("sort");

    // Specifying that we want pipe both the output and the input.
    // Similarly to capturing the output, by configuring the pipe
    // to stdin it can now be used as an asynchronous writer.
    cmd.stdout(Stdio::piped());
    cmd.stdin(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn command");

    // These are the animals we want to sort
    let animals: &[&str] = &["dog", "bird", "frog", "cat", "fish"];

    let mut stdin = child
        .stdin
        .take()
        .expect("child did not have a handle to stdin");

    // Write our animals to the child process
    // Note that the behavior of `sort` is to buffer _all input_ before writing any output.
    // In the general sense, it is recommended to write to the child in a separate task as
    // awaiting its exit (or output) to avoid deadlocks (for example, the child tries to write
    // some output but gets stuck waiting on the parent to read from it, meanwhile the parent
    // is stuck waiting to write its input completely before reading the output).
    stdin
        .write(animals.join("\n").as_bytes())
        .await
        .expect("could not write to stdin");

    // We drop the handle here which signals EOF to the child process.
    // This tells the child process that it there is no more data on the pipe.
    drop(stdin);

    let op = child.wait_with_output().await?;

    // Results should come back in sorted order
    assert_eq!(op.stdout, "bird\ncat\ndog\nfish\nfrog\n".as_bytes());

    Ok(())
}
```

With some coordination, we can also pipe the output of one command into
another.

```no_run
use tokio::join;
use tokio::process::Command;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut echo = Command::new("echo")
        .arg("hello world!")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn echo");

    let tr_stdin: Stdio = echo
        .stdout
        .take()
        .unwrap()
        .try_into()
        .expect("failed to convert to Stdio");

    let tr = Command::new("tr")
        .arg("a-z")
        .arg("A-Z")
        .stdin(tr_stdin)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn tr");

    let (echo_result, tr_output) = join!(echo.wait(), tr.wait_with_output());

    assert!(echo_result.unwrap().success());

    let tr_output = tr_output.expect("failed to await tr");
    assert!(tr_output.status.success());

    assert_eq!(tr_output.stdout, b"HELLO WORLD!\n");

    Ok(())
}
```

# Caveats

## Dropping/Cancellation

Similar to the behavior to the standard library, and unlike the futures
paradigm of dropping-implies-cancellation, a spawned process will, by
default, continue to execute even after the `Child` handle has been dropped.

The [`Command::kill_on_drop`] method can be used to modify this behavior
and kill the child process if the `Child` wrapper is dropped before it
has exited.

## Unix Processes

On Unix platforms processes must be "reaped" by their parent process after
they have exited in order to release all OS resources. A child process which
has exited, but has not yet been reaped by its parent is considered a "zombie"
process. Such processes continue to count against limits imposed by the system,
and having too many zombie processes present can prevent additional processes
from being spawned.

The tokio runtime will, on a best-effort basis, attempt to reap and clean up
any process which it has spawned. No additional guarantees are made with regard to
how quickly or how often this procedure will take place.

It is recommended to avoid dropping a [`Child`] process handle before it has been
fully `await`ed if stricter cleanup guarantees are required.

[`Command`]: crate::process::Command
[`Command::kill_on_drop`]: crate::process::Command::kill_on_drop
[`Child`]: crate::process::Child



## Module: runtime

The Tokio runtime.

Unlike other Rust programs, asynchronous applications require runtime
support. In particular, the following runtime services are necessary:

* An **I/O event loop**, called the driver, which drives I/O resources and
  dispatches I/O events to tasks that depend on them.
* A **scheduler** to execute [tasks] that use these I/O resources.
* A **timer** for scheduling work to run after a set period of time.

Tokio's [`Runtime`] bundles all of these services as a single type, allowing
them to be started, shut down, and configured together. However, often it is
not required to configure a [`Runtime`] manually, and a user may just use the
[`tokio::main`] attribute macro, which creates a [`Runtime`] under the hood.

# Choose your runtime

Here is the rules of thumb to choose the right runtime for your application.

```plaintext
   +------------------------------------------------------+
   | Do you want work-stealing or multi-thread scheduler? |
   +------------------------------------------------------+
                   | Yes              | No
                   |                  |
                   |                  |
                   v                  |
     +------------------------+       |
     | Multi-threaded Runtime |       |
     +------------------------+       |
                                      |
                                      V
                     +--------------------------------+
                     | Do you execute `!Send` Future? |
                     +--------------------------------+
                           | Yes                 | No
                           |                     |
                           V                     |
                   +---------------+             |
                   | Local Runtime |             |
                   +---------------+             |
                                                 |
                                                 v
                                     +------------------------+
                                     | Current-thread Runtime |
                                     +------------------------+
```

The above decision tree is not exhaustive. there are other factors that
may influence your decision.

## Bridging with sync code

See <https://tokio.rs/tokio/topics/bridging> for details.

## NUMA awareness

The tokio runtime is not NUMA (Non-Uniform Memory Access) aware.
You may want to start multiple runtimes instead of a single runtime
for better performance on NUMA systems.

# Usage

When no fine tuning is required, the [`tokio::main`] attribute macro can be
used.

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
                        println!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    println!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
# }
```

From within the context of the runtime, additional tasks are spawned using
the [`tokio::spawn`] function. Futures spawned using this function will be
executed on the same thread pool used by the [`Runtime`].

A [`Runtime`] instance can also be used directly.

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the runtime
    let rt  = Runtime::new()?;

    // Spawn the root task
    rt.block_on(async {
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
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    })
}
# }
```

## Runtime Configurations

Tokio provides multiple task scheduling strategies, suitable for different
applications. The [runtime builder] or `#[tokio::main]` attribute may be
used to select which scheduler to use.

#### Multi-Thread Scheduler

The multi-thread scheduler executes futures on a _thread pool_, using a
work-stealing strategy. By default, it will start a worker thread for each
CPU core available on the system. This tends to be the ideal configuration
for most applications, but see the docs of
[`std::thread::available_parallelism`] for caveats. The multi-thread
scheduler requires the `rt-multi-thread` feature flag, and is selected by
default:
```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let threaded_rt = runtime::Runtime::new()?;
# Ok(()) }
# }
```

Most applications should use the multi-thread scheduler, except in some
niche use-cases, such as when running only a single thread is required.

#### Current-Thread Scheduler

The current-thread scheduler provides a _single-threaded_ future executor.
All tasks will be created and executed on the current thread. This requires
the `rt` feature flag.
```
use tokio::runtime;

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let rt = runtime::Builder::new_current_thread()
    .build()?;
# Ok(()) }
```

#### Resource drivers

When configuring a runtime by hand, no resource drivers are enabled by
default. In this case, attempting to use networking types or time types will
fail. In order to enable these types, the resource drivers must be enabled.
This is done with [`Builder::enable_io`] and [`Builder::enable_time`]. As a
shorthand, [`Builder::enable_all`] enables both resource drivers.

## Driving the runtime

A Tokio runtime can only execute tasks if the runtime is running. Normally
this is not an issue as the default configuration of a runtime is always running,
but alternate configurations such as the current-thread runtime require that
[`Runtime::block_on`] is called.

- A multi-threaded runtime is always running because it spawns its own worker
  threads.
- A current-thread runtime does not spawn any worker threads, so it can only
  execute tasks when you provide a thread by calling [`Runtime::block_on`].
- A [`LocalSet`](crate::task::LocalSet) only executes local tasks spawned on
  it when the `LocalSet` is `.awaited` or otherwise driven using one of its
  methods for this purpose.

Please be aware that [`Handle::block_on`] does not drive the runtime.
There must be at least one call to [`Runtime::block_on`] when using the current
thread runtime. [`Handle::block_on`] is not enough.

## Lifetime of spawned threads

The runtime may spawn threads depending on its configuration and usage. The
multi-thread scheduler spawns threads to schedule tasks and for `spawn_blocking`
calls.

While the `Runtime` is active, threads may shut down after periods of being
idle. Once `Runtime` is dropped, all runtime threads have usually been
terminated, but in the presence of unstoppable spawned work are not
guaranteed to have been terminated. See the
[struct level documentation](Runtime#shutdown) for more details.

## Unix `fork`

User code that calls `fork(2)` without immediately calling `exec` must not
reuse Tokio in the child process. Tokio supports this kind of fork only in
two cases:

- The fork happens before the parent process has used Tokio in any way.
- The child process does not use Tokio after the fork.

Creating or using a Tokio runtime in a child process after the parent has
used Tokio is not supported, even if the runtime in the child is newly
created. Some Tokio modules, including process and signal handling, use
process-global state that cannot currently be reset after `fork`.

[tasks]: crate::task
[`Runtime`]: Runtime
[`tokio::spawn`]: crate::spawn
[`tokio::main`]: ../attr.main.html
[runtime builder]: crate::runtime::Builder
[`Runtime::new`]: crate::runtime::Runtime::new
[`Builder::enable_io`]: crate::runtime::Builder::enable_io
[`Builder::enable_time`]: crate::runtime::Builder::enable_time
[`Builder::enable_all`]: crate::runtime::Builder::enable_all

# Detailed runtime behavior

This section gives more details into how the Tokio runtime will schedule
tasks for execution.

At its most basic level, a runtime has a collection of tasks that need to be
scheduled. It will repeatedly remove a task from that collection and
schedule it (by calling [`poll`]). When the collection is empty, the thread
will go to sleep until a task is added to the collection.

However, the above is not sufficient to guarantee a well-behaved runtime.
For example, the runtime might have a single task that is always ready to be
scheduled, and schedule that task every time. This is a problem because it
starves other tasks by not scheduling them. To solve this, Tokio provides
the following fairness guarantee:

> If the total number of tasks does not grow without bound, and no task is
> [blocking the thread], then it is guaranteed that tasks are scheduled
> fairly.

Or, more formally:

> Under the following two assumptions:
>
> * There is some number `MAX_TASKS` such that the total number of tasks on
>   the runtime at any specific point in time never exceeds `MAX_TASKS`.
> * There is some number `MAX_SCHEDULE` such that calling [`poll`] on any
>   task spawned on the runtime returns within `MAX_SCHEDULE` time units.
>
> Then, there is some number `MAX_DELAY` such that when a task is woken, it
> will be scheduled by the runtime within `MAX_DELAY` time units.

(Here, `MAX_TASKS` and `MAX_SCHEDULE` can be any number and the user of
the runtime may choose them. The `MAX_DELAY` number is controlled by the
runtime, and depends on the value of `MAX_TASKS` and `MAX_SCHEDULE`.)

Other than the above fairness guarantee, there is no guarantee about the
order in which tasks are scheduled. There is also no guarantee that the
runtime is equally fair to all tasks. For example, if the runtime has two
tasks A and B that are both ready, then the runtime may schedule A five
times before it schedules B. This is the case even if A yields using
[`yield_now`]. All that is guaranteed is that it will schedule B eventually.

Normally, tasks are scheduled only if they have been woken by calling
[`wake`] on their waker. However, this is not guaranteed, and Tokio may
schedule tasks that have not been woken under some circumstances. This is
called a spurious wakeup.

## IO and timers

Beyond just scheduling tasks, the runtime must also manage IO resources and
timers. It does this by periodically checking whether there are any IO
resources or timers that are ready, and waking the relevant task so that
it will be scheduled.

These checks are performed periodically between scheduling tasks. Under the
same assumptions as the previous fairness guarantee, Tokio guarantees that
it will wake tasks with an IO or timer event within some maximum number of
time units.

## Current thread runtime (behavior at the time of writing)

This section describes how the [current thread runtime] behaves today. This
behavior may change in future versions of Tokio.

The current thread runtime maintains two FIFO queues of tasks that are ready
to be scheduled: the global queue and the local queue. The runtime will prefer
to choose the next task to schedule from the local queue, and will only pick a
task from the global queue if the local queue is empty, or if it has picked
a task from the local queue 31 times in a row. The number 31 can be
changed using the [`global_queue_interval`] setting.

The runtime will check for new IO or timer events whenever there are no
tasks ready to be scheduled, or when it has scheduled 61 tasks in a row. The
number 61 may be changed using the [`event_interval`] setting.

When a task is woken from within a task running on the runtime, then the
woken task is added directly to the local queue. Otherwise, the task is
added to the global queue. The current thread runtime does not use [the lifo
slot optimization].

## Multi threaded runtime (behavior at the time of writing)

This section describes how the [multi thread runtime] behaves today. This
behavior may change in future versions of Tokio.

A multi thread runtime has a fixed number of worker threads, which are all
created on startup. The multi thread runtime maintains one global queue, and
a local queue for each worker thread. The local queue of a worker thread can
fit at most 256 tasks. If more than 256 tasks are added to the local queue,
then half of them are moved to the global queue to make space.

The runtime will prefer to choose the next task to schedule from the local
queue, and will only pick a task from the global queue if the local queue is
empty, or if it has picked a task from the local queue
[`global_queue_interval`] times in a row. If the value of
[`global_queue_interval`] is not explicitly set using the runtime builder,
then the runtime will dynamically compute it using a heuristic that targets
10ms intervals between each check of the global queue (based on the
[`worker_mean_poll_time`] metric).

If both the local queue and global queue is empty, then the worker thread
will attempt to steal tasks from the local queue of another worker thread.
Stealing is done by moving half of the tasks in one local queue to another
local queue.

The runtime will check for new IO or timer events whenever there are no
tasks ready to be scheduled, or when it has scheduled 61 tasks in a row. The
number 61 may be changed using the [`event_interval`] setting.

The multi thread runtime uses [the lifo slot optimization]: Whenever a task
wakes up another task, the other task is added to the worker thread's lifo
slot instead of being added to a queue. If there was already a task in the
lifo slot when this happened, then the lifo slot is replaced, and the task
that used to be in the lifo slot is placed in the thread's local queue.
When the runtime finishes scheduling a task, it will schedule the task in
the lifo slot immediately, if any. When the lifo slot is used, the [coop
budget] is not reset. Furthermore, if a worker thread uses the lifo slot
three times in a row, it is temporarily disabled until the worker thread has
scheduled a task that didn't come from the lifo slot. The lifo slot can be
disabled using the [`disable_lifo_slot`] setting. The lifo slot is separate
from the local queue, so other worker threads cannot steal the task in the
lifo slot.

When a task is woken from a thread that is not a worker thread, then the
task is placed in the global queue.

# Performance tuning

## File descriptor table pre-warming

On Linux, file descriptor table growth can stall worker threads. See the
[`prewarm-fd-table`] example.

[`poll`]: std::future::Future::poll
[`wake`]: std::task::Waker::wake
[`yield_now`]: crate::task::yield_now
[blocking the thread]: https://ryhl.io/blog/async-what-is-blocking/
[current thread runtime]: crate::runtime::Builder::new_current_thread
[multi thread runtime]: crate::runtime::Builder::new_multi_thread
[`global_queue_interval`]: crate::runtime::Builder::global_queue_interval
[`event_interval`]: crate::runtime::Builder::event_interval
[`disable_lifo_slot`]: crate::runtime::Builder::disable_lifo_slot
[the lifo slot optimization]: crate::runtime::Builder::disable_lifo_slot
[coop budget]: crate::task::coop#cooperative-scheduling
[`worker_mean_poll_time`]: crate::runtime::RuntimeMetrics::worker_mean_poll_time
[`prewarm-fd-table`]: https://github.com/tokio-rs/tokio/blob/master/examples/prewarm-fd-table.rs



## tokio::select

*Declarative Macro*

Waits on multiple concurrent branches, returning when the **first** branch
completes, cancelling the remaining branches.

The `select!` macro must be used inside of async functions, closures, and
blocks.

The `select!` macro accepts one or more branches with the following pattern:

```text
<pattern> = <async expression> (, if <precondition>)? => <handler>,
```

Additionally, the `select!` macro may include a single, optional `else`
branch, which evaluates if none of the other branches match their patterns:

```text
else => <expression>
```

The macro aggregates all `<async expression>` expressions and runs them
concurrently on the **current** task. Once the **first** expression
completes with a value that matches its `<pattern>`, the `select!` macro
returns the result of evaluating the completed branch's `<handler>`
expression.

Additionally, each branch may include an optional `if` precondition. If the
precondition returns `false`, then the branch is disabled. The provided
`<async expression>` is still evaluated but the resulting future is never
polled. This capability is useful when using `select!` within a loop.

The complete lifecycle of a `select!` expression is as follows:

1. Evaluate all provided `<precondition>` expressions. If the precondition
   returns `false`, disable the branch for the remainder of the current call
   to `select!`. Re-entering `select!` due to a loop clears the "disabled"
   state.
2. Aggregate the `<async expression>`s from each branch, including the
   disabled ones. If the branch is disabled, `<async expression>` is still
   evaluated, but the resulting future is not polled.
3. If **all** branches are disabled: go to step 6.
4. Concurrently await on the results for all remaining `<async expression>`s.
5. Once an `<async expression>` returns a value, attempt to apply the value to the
   provided `<pattern>`. If the pattern matches, evaluate the `<handler>` and return.
   If the pattern **does not** match, disable the current branch for the remainder of
   the current call to `select!`. Continue from step 3.
6. Evaluate the `else` expression. If no else expression is provided, panic.

# Runtime characteristics

By running all async expressions on the current task, the expressions are
able to run **concurrently** but not in **parallel**. This means all
expressions are run on the same thread and if one branch blocks the thread,
all other expressions will be unable to continue. If parallelism is
required, spawn each async expression using [`tokio::spawn`] and pass the
join handle to `select!`.

[`tokio::spawn`]: crate::spawn

# Fairness

By default, `select!` randomly picks a branch to check first. This provides
some level of fairness when calling `select!` in a loop with branches that
are always ready.

This behavior can be overridden by adding `biased;` to the beginning of the
macro usage. See the examples for details. This will cause `select` to poll
the futures in the order they appear from top to bottom. There are a few
reasons you may want this:

- The random number generation of `tokio::select!` has a non-zero CPU cost
- Your futures may interact in a way where known polling order is significant

But there is an important caveat to this mode. It becomes your responsibility
to ensure that the polling order of your futures is fair. If for example you
are selecting between a stream and a shutdown future, and the stream has a
huge volume of messages and zero or nearly zero time between them, you should
place the shutdown future earlier in the `select!` list to ensure that it is
always polled, and will not be ignored due to the stream being constantly
ready.

# Panics

The `select!` macro panics if all branches are disabled **and** there is no
provided `else` branch. A branch is disabled when the provided `if`
precondition returns `false` **or** when the pattern does not match the
result of `<async expression>`.

# Cancellation safety

When using `select!` in a loop to receive messages from multiple sources,
you should make sure that the receive call is cancellation safe to avoid
losing messages. This section goes through various common methods and
describes whether they are cancel safe.  The lists in this section are not
exhaustive.

Cancellation safety describes what happens when a future is dropped
before it completes. Whether something is cancellation safe depends on
the behavior of the future passed to `select!`, which may come from an
async method, an async expression, or another future-producing operation.

The following methods are cancellation safe:

 * [`tokio::sync::mpsc::Receiver::recv`](crate::sync::mpsc::Receiver::recv)
 * [`tokio::sync::mpsc::UnboundedReceiver::recv`](crate::sync::mpsc::UnboundedReceiver::recv)
 * [`tokio::sync::broadcast::Receiver::recv`](crate::sync::broadcast::Receiver::recv)
 * [`tokio::sync::watch::Receiver::changed`](crate::sync::watch::Receiver::changed)
 * [`tokio::net::TcpListener::accept`](crate::net::TcpListener::accept)
 * [`tokio::net::UnixListener::accept`](crate::net::UnixListener::accept)
 * [`tokio::signal::unix::Signal::recv`](crate::signal::unix::Signal::recv)
 * [`tokio::io::AsyncReadExt::read`](crate::io::AsyncReadExt::read) on any `AsyncRead`
 * [`tokio::io::AsyncReadExt::read_buf`](crate::io::AsyncReadExt::read_buf) on any `AsyncRead`
 * [`tokio::io::AsyncWriteExt::write`](crate::io::AsyncWriteExt::write) on any `AsyncWrite`
 * [`tokio::io::AsyncWriteExt::write_buf`](crate::io::AsyncWriteExt::write_buf) on any `AsyncWrite`
 * [`tokio_stream::StreamExt::next`](https://docs.rs/tokio-stream/0.1/tokio_stream/trait.StreamExt.html#method.next) on any `Stream`
 * [`futures::stream::StreamExt::next`](https://docs.rs/futures/0.3/futures/stream/trait.StreamExt.html#method.next) on any `Stream`

The following methods are not cancellation safe and can lead to loss of data:

 * [`tokio::io::AsyncReadExt::read_exact`](crate::io::AsyncReadExt::read_exact)
 * [`tokio::io::AsyncReadExt::read_to_end`](crate::io::AsyncReadExt::read_to_end)
 * [`tokio::io::AsyncReadExt::read_to_string`](crate::io::AsyncReadExt::read_to_string)
 * [`tokio::io::AsyncWriteExt::write_all`](crate::io::AsyncWriteExt::write_all)

The following methods are not cancellation safe because they use a queue for
fairness and cancellation makes you lose your place in the queue:

 * [`tokio::sync::Mutex::lock`](crate::sync::Mutex::lock)
 * [`tokio::sync::RwLock::read`](crate::sync::RwLock::read)
 * [`tokio::sync::RwLock::write`](crate::sync::RwLock::write)
 * [`tokio::sync::Semaphore::acquire`](crate::sync::Semaphore::acquire)
 * [`tokio::sync::Notify::notified`](crate::sync::Notify::notified)

To determine whether your own methods are cancellation safe, look for the
location of uses of `.await`. This is because when an asynchronous method is
cancelled, that always happens at an `.await`. If your function behaves
correctly even if it is restarted while waiting at an `.await`, then it is
cancellation safe.

Cancellation safety can be defined in the following way: If you have a
future that has not yet completed, then it must be a no-op to drop that
future and recreate it. This definition is motivated by the situation where
a `select!` is used in a loop. Without this guarantee, you would lose your
progress when another branch completes and you restart the `select!` by
going around the loop.

Be aware that cancelling something that is not cancellation safe is not
necessarily wrong. For example, if you are cancelling a task because the
application is shutting down, then you probably don't care that partially
read data is lost.

# Examples

Basic select with two branches.

```
async fn do_stuff_async() {
    // async work
}

async fn more_async_work() {
    // more here
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
tokio::select! {
    _ = do_stuff_async() => {
        println!("do_stuff_async() completed first")
    }
    _ = more_async_work() => {
        println!("more_async_work() completed first")
    }
};
# }
```

Basic stream selecting.

```
use tokio_stream::{self as stream, StreamExt};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut stream1 = stream::iter(vec![1, 2, 3]);
let mut stream2 = stream::iter(vec![4, 5, 6]);

let next = tokio::select! {
    v = stream1.next() => v.unwrap(),
    v = stream2.next() => v.unwrap(),
};

assert!(next == 1 || next == 4);
# }
```

Collect the contents of two streams. In this example, we rely on pattern
matching and the fact that `stream::iter` is "fused", i.e. once the stream
is complete, all calls to `next()` return `None`.

```
use tokio_stream::{self as stream, StreamExt};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut stream1 = stream::iter(vec![1, 2, 3]);
let mut stream2 = stream::iter(vec![4, 5, 6]);

let mut values = vec![];

loop {
    tokio::select! {
        Some(v) = stream1.next() => values.push(v),
        Some(v) = stream2.next() => values.push(v),
        else => break,
    }
}

values.sort();
assert_eq!(&[1, 2, 3, 4, 5, 6], &values[..]);
# }
```

Using the same future in multiple `select!` expressions can be done by passing
a reference to the future. Doing so requires the future to be [`Unpin`]. A
future can be made [`Unpin`] by either using [`Box::pin`] or stack pinning.

[`Unpin`]: std::marker::Unpin
[`Box::pin`]: std::boxed::Box::pin

Here, a stream is consumed for at most 1 second.

```
use tokio_stream::{self as stream, StreamExt};
use tokio::time::{self, Duration};

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut stream = stream::iter(vec![1, 2, 3]);
let sleep = time::sleep(Duration::from_secs(1));
tokio::pin!(sleep);

loop {
    tokio::select! {
        maybe_v = stream.next() => {
            if let Some(v) = maybe_v {
                println!("got = {}", v);
            } else {
                break;
            }
        }
        _ = &mut sleep => {
            println!("timeout");
            break;
        }
    }
}
# }
```

Joining two values using `select!`.

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx1, mut rx1) = oneshot::channel();
let (tx2, mut rx2) = oneshot::channel();

tokio::spawn(async move {
    tx1.send("first").unwrap();
});

tokio::spawn(async move {
    tx2.send("second").unwrap();
});

let mut a = None;
let mut b = None;

while a.is_none() || b.is_none() {
    tokio::select! {
        v1 = (&mut rx1), if a.is_none() => a = Some(v1.unwrap()),
        v2 = (&mut rx2), if b.is_none() => b = Some(v2.unwrap()),
    }
}

let res = (a.unwrap(), b.unwrap());

assert_eq!(res.0, "first");
assert_eq!(res.1, "second");
# }
```

Using the `biased;` mode to control polling order.

```
# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut count = 0u8;

loop {
    tokio::select! {
        // If you run this example without `biased;`, the polling order is
        // pseudo-random, and the assertions on the value of count will
        // (probably) fail.
        biased;

        _ = async {}, if count < 1 => {
            count += 1;
            assert_eq!(count, 1);
        }
        _ = async {}, if count < 2 => {
            count += 1;
            assert_eq!(count, 2);
        }
        _ = async {}, if count < 3 => {
            count += 1;
            assert_eq!(count, 3);
        }
        _ = async {}, if count < 4 => {
            count += 1;
            assert_eq!(count, 4);
        }

        else => {
            break;
        }
    };
}
# }
```

## Avoid racy `if` preconditions

Given that `if` preconditions are used to disable `select!` branches, some
caution must be used to avoid missing values.

For example, here is **incorrect** usage of `sleep` with `if`. The objective
is to repeatedly run an asynchronous task for up to 50 milliseconds.
However, there is a potential for the `sleep` completion to be missed.

```no_run,should_panic
use tokio::time::{self, Duration};

async fn some_async_work() {
    // do work
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let sleep = time::sleep(Duration::from_millis(50));
tokio::pin!(sleep);

while !sleep.is_elapsed() {
    tokio::select! {
        _ = &mut sleep, if !sleep.is_elapsed() => {
            println!("operation timed out");
        }
        _ = some_async_work() => {
            println!("operation completed");
        }
    }
}

panic!("This example shows how not to do it!");
# }
```

In the above example, `sleep.is_elapsed()` may return `true` even if
`sleep.poll()` never returned `Ready`. This opens up a potential race
condition where `sleep` expires between the `while !sleep.is_elapsed()`
check and the call to `select!` resulting in the `some_async_work()` call to
run uninterrupted despite the sleep having elapsed.

One way to write the above example without the race would be:

```
use tokio::time::{self, Duration};

async fn some_async_work() {
# time::sleep(Duration::from_millis(10)).await;
    // do work
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let sleep = time::sleep(Duration::from_millis(50));
tokio::pin!(sleep);

loop {
    tokio::select! {
        _ = &mut sleep => {
            println!("operation timed out");
            break;
        }
        _ = some_async_work() => {
            println!("operation completed");
        }
    }
}
# }
```
# Alternatives from the Ecosystem

The `select!` macro is a powerful tool for managing multiple asynchronous
branches, enabling tasks to run concurrently within the same thread. However,
its use can introduce challenges, particularly around cancellation safety, which
can lead to subtle and hard-to-debug errors. For many use cases, ecosystem
alternatives may be preferable as they mitigate these concerns by offering
clearer syntax, more predictable control flow, and reducing the need to manually
handle issues like fuse semantics or cancellation safety.

## Merging Streams

For cases where `loop { select! { ... } }` is used to poll multiple tasks,
stream merging offers a concise alternative, inherently handle cancellation-safe
processing, removing the risk of data loss. Libraries such as [`tokio_stream`],
[`futures::stream`] and [`futures_concurrency`] provide tools for merging
streams and handling their outputs sequentially.

[`tokio_stream`]: https://docs.rs/tokio-stream/latest/tokio_stream/
[`futures::stream`]: https://docs.rs/futures/latest/futures/stream/
[`futures_concurrency`]: https://docs.rs/futures-concurrency/latest/futures_concurrency/

### Example with `select!`

```
struct File;
struct Channel;
struct Socket;

impl Socket {
    async fn read_packet(&mut self) -> Vec<u8> {
        vec![]
    }
}

async fn read_send(_file: &mut File, _channel: &mut Channel) {
    // do work that is not cancel safe
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
// open our IO types
let mut file = File;
let mut channel = Channel;
let mut socket = Socket;

loop {
    tokio::select! {
        _ = read_send(&mut file, &mut channel) => { /* ... */ },
        _data = socket.read_packet() => { /* ... */ }
        _ = futures::future::ready(()) => break
    }
}
# }
```

### Moving to `merge`

By using merge, you can unify multiple asynchronous tasks into a single stream,
eliminating the need to manage tasks manually and reducing the risk of
unintended behavior like data loss.

```
use std::pin::pin;

use futures::stream::unfold;
use tokio_stream::StreamExt;

struct File;
struct Channel;
struct Socket;

impl Socket {
    async fn read_packet(&mut self) -> Vec<u8> {
        vec![]
    }
}

async fn read_send(_file: &mut File, _channel: &mut Channel) {
    // do work that is not cancel safe
}

enum Message {
    Stop,
    Sent,
    Data(Vec<u8>),
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
// open our IO types
let file = File;
let channel = Channel;
let socket = Socket;

let a = unfold((file, channel), |(mut file, mut channel)| async {
    read_send(&mut file, &mut channel).await;
    Some((Message::Sent, (file, channel)))
});
let b = unfold(socket, |mut socket| async {
    let data = socket.read_packet().await;
    Some((Message::Data(data), socket))
});
let c = tokio_stream::iter([Message::Stop]);

let mut s = pin!(a.merge(b).merge(c));
while let Some(msg) = s.next().await {
    match msg {
        Message::Data(_data) => { /* ... */ }
        Message::Sent => continue,
        Message::Stop => break,
    }
}
# }
```

## Racing Futures

If you need to wait for the first completion among several asynchronous tasks,
ecosystem utilities such as
[`futures`](https://docs.rs/futures/latest/futures/),
[`futures-lite`](https://docs.rs/futures-lite/latest/futures_lite/) or
[`futures-concurrency`](https://docs.rs/futures-concurrency/latest/futures_concurrency/)
provide streamlined syntax for racing futures:

- [`futures_concurrency::future::Race`](https://docs.rs/futures-concurrency/latest/futures_concurrency/future/trait.Race.html)
- [`futures::select`](https://docs.rs/futures/latest/futures/macro.select.html)
- [`futures::stream::select_all`](https://docs.rs/futures/latest/futures/stream/select_all/index.html) (for streams)
- [`futures_lite::future::or`](https://docs.rs/futures-lite/latest/futures_lite/future/fn.or.html)
- [`futures_lite::future::race`](https://docs.rs/futures-lite/latest/futures_lite/future/fn.race.html)

```
use futures_concurrency::future::Race;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let task_a = async { Ok("ok") };
let task_b = async { Err("error") };
let result = (task_a, task_b).race().await;

match result {
    Ok(output) => println!("First task completed with: {output}"),
    Err(err) => eprintln!("Error occurred: {err}"),
}
# }
```

```rust
macro_rules! select {
    {
        $(
            biased;
        )?
        $(
            $bind:pat = $fut:expr $(, if $cond:expr)? => $handler:expr,
        )*
        $(
            else => $els:expr $(,)?
        )?
    } => { ... };
}
```



## Module: signal

Asynchronous signal handling for Tokio.

Note that signal handling is in general a very tricky topic and should be
used with great care. This crate attempts to implement 'best practice' for
signal handling, but it should be evaluated for your own applications' needs
to see if it's suitable.

There are some fundamental limitations of this crate documented on the OS
specific structures, as well.

# Examples

Print on "ctrl-c" notification.

```rust,no_run
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    signal::ctrl_c().await?;
    println!("ctrl-c received!");
    Ok(())
}
```

Wait for `SIGHUP` on Unix

```rust,no_run
# #[cfg(unix)] {
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An infinite stream of hangup signals.
    let mut stream = signal(SignalKind::hangup())?;

    // Print whenever a HUP signal is received
    loop {
        stream.recv().await;
        println!("got signal HUP");
    }
}
# }
```



## Module: stream

Due to the `Stream` trait's inclusion in `std` landing later than Tokio's 1.0
release, most of the Tokio stream utilities have been moved into the [`tokio-stream`]
crate.

# Why was `Stream` not included in Tokio 1.0?

Originally, we had planned to ship Tokio 1.0 with a stable `Stream` type
but unfortunately the [RFC] had not been merged in time for `Stream` to
reach `std` on a stable compiler in time for the 1.0 release of Tokio. For
this reason, the team has decided to move all `Stream` based utilities to
the [`tokio-stream`] crate. While this is not ideal, once `Stream` has made
it into the standard library and the `MSRV` period has passed, we will implement
stream for our different types.

While this may seem unfortunate, not all is lost as you can get much of the
`Stream` support with `async/await` and `while let` loops. It is also possible
to create a `impl Stream` from `async fn` using the [`async-stream`] crate.

[`tokio-stream`]: https://docs.rs/tokio-stream
[`async-stream`]: https://docs.rs/async-stream
[RFC]: https://github.com/rust-lang/rfcs/pull/2996

# Example

Convert a [`sync::mpsc::Receiver`] to an `impl Stream`.

```rust,no_run
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<usize>(16);

let stream = async_stream::stream! {
    while let Some(item) = rx.recv().await {
        yield item;
    }
};
```



## Module: sync

Synchronization primitives for use in asynchronous contexts.

Tokio programs tend to be organized as a set of [tasks] where each task
operates independently and may be executed on separate physical threads. The
synchronization primitives provided in this module permit these independent
tasks to communicate together.

[tasks]: crate::task

# Message passing

The most common form of synchronization in a Tokio program is message
passing. Two tasks operate independently and send messages to each other to
synchronize. Doing so has the advantage of avoiding shared state.

Message passing is implemented using channels. A channel supports sending a
message from one producer task to one or more consumer tasks. There are a
few flavors of channels provided by Tokio. Each channel flavor supports
different message passing patterns. When a channel supports multiple
producers, many separate tasks may **send** messages. When a channel
supports multiple consumers, many different separate tasks may **receive**
messages.

Tokio provides many different channel flavors as different message passing
patterns are best handled with different implementations.

## `oneshot` channel

The [`oneshot` channel][oneshot] supports sending a **single** value from a
single producer to a single consumer. This channel is usually used to send
the result of a computation to a waiter.

**Example:** using a [`oneshot` channel][oneshot] to receive the result of a
computation.

```
use tokio::sync::oneshot;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    let res = some_computation().await;
    tx.send(res).unwrap();
});

// Do other work while the computation is happening in the background

// Wait for the computation result
let res = rx.await.unwrap();
# }
```

Note, if the task produces a computation result as its final
action before terminating, the [`JoinHandle`] can be used to
receive that value instead of allocating resources for the
`oneshot` channel. Awaiting on [`JoinHandle`] returns `Result`. If
the task panics, the `Joinhandle` yields `Err` with the panic
cause.

**Example:**

```
async fn some_computation() -> String {
    "the result of the computation".to_string()
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let join_handle = tokio::spawn(async move {
    some_computation().await
});

// Do other work while the computation is happening in the background

// Wait for the computation result
let res = join_handle.await.unwrap();
# }
```

[`JoinHandle`]: crate::task::JoinHandle

## `mpsc` channel

The [`mpsc` channel][mpsc] supports sending **many** values from **many**
producers to a single consumer. This channel is often used to send work to a
task or to receive the result of many computations.

This is also the channel you should use if you want to send many messages
from a single producer to a single consumer. There is no dedicated spsc
channel.

**Example:** using an mpsc to incrementally stream the results of a series
of computations.

```
use tokio::sync::mpsc;

async fn some_computation(input: u32) -> String {
    format!("the result of computation {}", input)
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = mpsc::channel(100);

tokio::spawn(async move {
    for i in 0..10 {
        let res = some_computation(i).await;
        tx.send(res).await.unwrap();
    }
});

while let Some(res) = rx.recv().await {
    println!("got = {}", res);
}
# }
```

The argument to `mpsc::channel` is the channel capacity. This is the maximum
number of values that can be stored in the channel pending receipt at any
given time. Properly setting this value is key in implementing robust
programs as the channel capacity plays a critical part in handling back
pressure.

A common concurrency pattern for resource management is to spawn a task
dedicated to managing that resource and using message passing between other
tasks to interact with the resource. The resource may be anything that may
not be concurrently used. Some examples include a socket and program state.
For example, if multiple tasks need to send data over a single socket, spawn
a task to manage the socket and use a channel to synchronize.

**Example:** sending data from many tasks over a single socket using message
passing.

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut socket = TcpStream::connect("www.example.com:1234").await?;
    let (tx, mut rx) = mpsc::channel(100);

    for _ in 0..10 {
        // Each task needs its own `tx` handle. This is done by cloning the
        // original handle.
        let tx = tx.clone();

        tokio::spawn(async move {
            tx.send(&b"data to write"[..]).await.unwrap();
        });
    }

    // The `rx` half of the channel returns `None` once **all** `tx` clones
    // drop. To ensure `None` is returned, drop the handle owned by the
    // current task. If this `tx` handle is not dropped, there will always
    // be a single outstanding `tx` handle.
    drop(tx);

    while let Some(res) = rx.recv().await {
        socket.write_all(res).await?;
    }

    Ok(())
}
# }
```

The [`mpsc`] and [`oneshot`] channels can be combined to provide a request /
response type synchronization pattern with a shared resource. A task is
spawned to synchronize a resource and waits on commands received on a
[`mpsc`] channel. Each command includes a [`oneshot`] `Sender` on which the
result of the command is sent.

**Example:** use a task to synchronize a `u64` counter. Each task sends an
"fetch and increment" command. The counter value **before** the increment is
sent over the provided `oneshot` channel.

```
use tokio::sync::{oneshot, mpsc};
use Command::Increment;

enum Command {
    Increment,
    // Other commands can be added here
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<u64>)>(100);

// Spawn a task to manage the counter
tokio::spawn(async move {
    let mut counter: u64 = 0;

    while let Some((cmd, response)) = cmd_rx.recv().await {
        match cmd {
            Increment => {
                let prev = counter;
                counter += 1;
                response.send(prev).unwrap();
            }
        }
    }
});

let mut join_handles = vec![];

// Spawn tasks that will send the increment command.
for _ in 0..10 {
    let cmd_tx = cmd_tx.clone();

    join_handles.push(tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        cmd_tx.send((Increment, resp_tx)).await.ok().unwrap();
        let res = resp_rx.await.unwrap();

        println!("previous value = {}", res);
    }));
}

// Wait for all tasks to complete
for join_handle in join_handles.drain(..) {
    join_handle.await.unwrap();
}
# }
```

## `broadcast` channel

The [`broadcast` channel] supports sending **many** values from
**many** producers to **many** consumers. Each consumer will receive
**each** value. This channel can be used to implement "fan out" style
patterns common with pub / sub or "chat" systems.

This channel tends to be used less often than `oneshot` and `mpsc` but still
has its use cases.

This is also the channel you should use if you want to broadcast values from
a single producer to many consumers. There is no dedicated spmc broadcast
channel.

Basic usage

```
use tokio::sync::broadcast;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    assert_eq!(rx1.recv().await.unwrap(), 10);
    assert_eq!(rx1.recv().await.unwrap(), 20);
});

tokio::spawn(async move {
    assert_eq!(rx2.recv().await.unwrap(), 10);
    assert_eq!(rx2.recv().await.unwrap(), 20);
});

tx.send(10).unwrap();
tx.send(20).unwrap();
# }
```

[`broadcast` channel]: crate::sync::broadcast

## `watch` channel

The [`watch` channel] supports sending **many** values from **many**
producers to **many** consumers. However, only the **most recent** value is
stored in the channel. Consumers are notified when a new value is sent, but
there is no guarantee that consumers will see **all** values.

The [`watch` channel] is similar to a [`broadcast` channel] with capacity 1.

Use cases for the [`watch` channel] include broadcasting configuration
changes or signalling program state changes, such as transitioning to
shutdown.

**Example:** use a [`watch` channel] to notify tasks of configuration
changes. In this example, a configuration file is checked periodically. When
the file changes, the configuration changes are signalled to consumers.

```
use tokio::sync::watch;
use tokio::time::{self, Duration, Instant};

use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Config {
    timeout: Duration,
}

impl Config {
    async fn load_from_file() -> io::Result<Config> {
        // file loading and deserialization logic here
# Ok(Config { timeout: Duration::from_secs(1) })
    }
}

async fn my_async_operation() {
    // Do something here
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
// Load initial configuration value
let mut config = Config::load_from_file().await.unwrap();

// Create the watch channel, initialized with the loaded configuration
let (tx, rx) = watch::channel(config.clone());

// Spawn a task to monitor the file.
tokio::spawn(async move {
    loop {
        // Wait 10 seconds between checks
        time::sleep(Duration::from_secs(10)).await;

        // Load the configuration file
        let new_config = Config::load_from_file().await.unwrap();

        // If the configuration changed, send the new config value
        // on the watch channel.
        if new_config != config {
            tx.send(new_config.clone()).unwrap();
            config = new_config;
        }
    }
});

let mut handles = vec![];

// Spawn tasks that runs the async operation for at most `timeout`. If
// the timeout elapses, restart the operation.
//
// The task simultaneously watches the `Config` for changes. When the
// timeout duration changes, the timeout is updated without restarting
// the in-flight operation.
for _ in 0..5 {
    // Clone a config watch handle for use in this task
    let mut rx = rx.clone();

    let handle = tokio::spawn(async move {
        // Start the initial operation and pin the future to the stack.
        // Pinning to the stack is required to resume the operation
        // across multiple calls to `select!`
        let op = my_async_operation();
        tokio::pin!(op);

        // Get the initial config value
        let mut conf = rx.borrow().clone();

        let mut op_start = Instant::now();
        let sleep = time::sleep_until(op_start + conf.timeout);
        tokio::pin!(sleep);

        loop {
            tokio::select! {
                _ = &mut sleep => {
                    // The operation elapsed. Restart it
                    op.set(my_async_operation());

                    // Track the new start time
                    op_start = Instant::now();

                    // Restart the timeout
                    sleep.set(time::sleep_until(op_start + conf.timeout));
                }
                _ = rx.changed() => {
                    conf = rx.borrow_and_update().clone();

                    // The configuration has been updated. Update the
                    // `sleep` using the new `timeout` value.
                    sleep.as_mut().reset(op_start + conf.timeout);
                }
                _ = &mut op => {
                    // The operation completed!
                    return
                }
            }
        }
    });

    handles.push(handle);
}

for handle in handles.drain(..) {
    handle.await.unwrap();
}
# }
```

[`watch` channel]: mod@crate::sync::watch
[`broadcast` channel]: mod@crate::sync::broadcast

# State synchronization

The remaining synchronization primitives focus on synchronizing state.
These are asynchronous equivalents to versions provided by `std`. They
operate in a similar way as their `std` counterparts but will wait
asynchronously instead of blocking the thread.

* [`Barrier`] Ensures multiple tasks will wait for each other to reach a
  point in the program, before continuing execution all together.

* [`Mutex`] Mutual Exclusion mechanism, which ensures that at most one
  thread at a time is able to access some data.

* [`Notify`] Basic task notification. `Notify` supports notifying a
  receiving task without sending data. In this case, the task wakes up and
  resumes processing.

* [`RwLock`] Provides a mutual exclusion mechanism which allows multiple
  readers at the same time, while allowing only one writer at a time. In
  some cases, this can be more efficient than a mutex.

* [`Semaphore`] Limits the amount of concurrency. A semaphore holds a
  number of permits, which tasks may request in order to enter a critical
  section. Semaphores are useful for implementing limiting or bounding of
  any kind.

# Runtime compatibility

All synchronization primitives provided in this module are runtime agnostic.
You can freely move them between different instances of the Tokio runtime
or even use them from non-Tokio runtimes.

When used in a Tokio runtime, the synchronization primitives participate in
[cooperative scheduling](crate::task::coop#cooperative-scheduling) to avoid
starvation. This feature does not apply when used from non-Tokio runtimes.

As an exception, methods ending in `_timeout` are not runtime agnostic
because they require access to the Tokio timer. See the documentation of
each `*_timeout` method for more information on its use.



## Module: task

Asynchronous green-threads.

## What are Tasks?

A _task_ is a light weight, non-blocking unit of execution. A task is similar
to an OS thread, but rather than being managed by the OS scheduler, they are
managed by the [Tokio runtime][rt]. Another name for this general pattern is
[green threads]. If you are familiar with [Go's goroutines], [Kotlin's
coroutines], or [Erlang's processes], you can think of Tokio's tasks as
something similar.

Key points about tasks include:

* Tasks are **light weight**. Because tasks are scheduled by the Tokio
  runtime rather than the operating system, creating new tasks or switching
  between tasks does not require a context switch and has fairly low
  overhead. Creating, running, and destroying large numbers of tasks is
  quite cheap, especially compared to OS threads.

* Tasks are scheduled **cooperatively**. Most operating systems implement
  _preemptive multitasking_. This is a scheduling technique where the
  operating system allows each thread to run for a period of time, and then
  _preempts_ it, temporarily pausing that thread and switching to another.
  Tasks, on the other hand, implement _cooperative multitasking_. In
  cooperative multitasking, a task is allowed to run until it _yields_,
  indicating to the Tokio runtime's scheduler that it cannot currently
  continue executing. When a task yields, the Tokio runtime switches to
  executing the next task.

* Tasks are **non-blocking**. Typically, when an OS thread performs I/O or
  must synchronize with another thread, it _blocks_, allowing the OS to
  schedule another thread. When a task cannot continue executing, it must
  yield instead, allowing the Tokio runtime to schedule another task. Tasks
  should generally not perform system calls or other operations that could
  block a thread, as this would prevent other tasks running on the same
  thread from executing as well. Instead, this module provides APIs for
  running blocking operations in an asynchronous context.

[rt]: crate::runtime
[green threads]: https://en.wikipedia.org/wiki/Green_threads
[Go's goroutines]: https://tour.golang.org/concurrency/1
[Kotlin's coroutines]: https://kotlinlang.org/docs/reference/coroutines-overview.html
[Erlang's processes]: http://erlang.org/doc/getting_started/conc_prog.html#processes

## Working with Tasks

This module provides the following APIs for working with tasks:

### Spawning

Perhaps the most important function in this module is [`task::spawn`]. This
function can be thought of as an async equivalent to the standard library's
[`thread::spawn`][`std::thread::spawn`]. It takes an `async` block or other
[future], and creates a new task to run that work concurrently:

```
use tokio::task;

# async fn doc() {
task::spawn(async {
    // perform some work here...
});
# }
```

Like [`std::thread::spawn`], `task::spawn` returns a [`JoinHandle`] struct.
A `JoinHandle` is itself a future which may be used to await the output of
the spawned task. For example:

```
use tokio::task;

# #[tokio::main(flavor = "current_thread")]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let join = task::spawn(async {
    // ...
    "hello world!"
});

// ...

// Await the result of the spawned task.
let result = join.await?;
assert_eq!(result, "hello world!");
# Ok(())
# }
```

Again, like `std::thread`'s [`JoinHandle` type][thread_join], if the spawned
task panics, awaiting its `JoinHandle` will return a [`JoinError`]. For
example:

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::task;

# #[tokio::main] async fn main() {
let join = task::spawn(async {
    panic!("something bad happened!")
});

// The returned result indicates that the task failed.
assert!(join.await.is_err());
# }
# }
```

`spawn`, `JoinHandle`, and `JoinError` are present when the "rt"
feature flag is enabled.

[`task::spawn`]: crate::task::spawn()
[future]: std::future::Future
[`std::thread::spawn`]: std::thread::spawn
[`JoinHandle`]: crate::task::JoinHandle
[thread_join]: std::thread::JoinHandle
[`JoinError`]: crate::task::JoinError

#### Cancellation

Spawned tasks may be cancelled using the [`JoinHandle::abort`] or
[`AbortHandle::abort`] methods. When one of these methods are called, the
task is signalled to shut down next time it yields at an `.await` point. If
the task is already idle, then it will be shut down as soon as possible
without running again before being shut down. Additionally, shutting down a
Tokio runtime (e.g. by returning from `#[tokio::main]`) immediately cancels
all tasks on it.

When tasks are shut down, it will stop running at whichever `.await` it has
yielded at. All local variables are destroyed by running their destructor.
Once shutdown has completed, awaiting the [`JoinHandle`] will fail with a
[cancelled error](crate::task::JoinError::is_cancelled).

Note that aborting a task does not guarantee that it fails with a cancelled
error, since it may complete normally first. For example, if the task does
not yield to the runtime at any point between the call to `abort` and the
end of the task, then the [`JoinHandle`] will instead report that the task
exited normally.

Be aware that tasks spawned using [`spawn_blocking`] cannot be aborted
because they are not async. If you call `abort` on a `spawn_blocking`
task, then this *will not have any effect*, and the task will continue
running normally. The exception is if the task has not started running
yet; in that case, calling `abort` may prevent the task from starting.

Be aware that calls to [`JoinHandle::abort`] just schedule the task for
cancellation, and will return before the cancellation has completed. To wait
for cancellation to complete, wait for the task to finish by awaiting the
[`JoinHandle`]. Similarly, the [`JoinHandle::is_finished`] method does not
return `true` until the cancellation has finished.

Calling [`JoinHandle::abort`] multiple times has the same effect as calling
it once.

Tokio also provides an [`AbortHandle`], which is like the [`JoinHandle`],
except that it does not provide a mechanism to wait for the task to finish.
Each task can only have one [`JoinHandle`], but it can have more than one
[`AbortHandle`].

[`JoinHandle::abort`]: crate::task::JoinHandle::abort
[`AbortHandle::abort`]: crate::task::AbortHandle::abort
[`AbortHandle`]: crate::task::AbortHandle
[`JoinHandle::is_finished`]: crate::task::JoinHandle::is_finished

### Blocking and Yielding

As we discussed above, code running in asynchronous tasks should not perform
operations that can block. A blocking operation performed in a task running
on a thread that is also running other tasks would block the entire thread,
preventing other tasks from running.

Instead, Tokio provides two APIs for running blocking operations in an
asynchronous context: [`task::spawn_blocking`] and [`task::block_in_place`].

Be aware that if you call a non-async method from async code, that non-async
method is still inside the asynchronous context, so you should also avoid
blocking operations there. This includes destructors of objects destroyed in
async code.

#### `spawn_blocking`

The `task::spawn_blocking` function is similar to the `task::spawn` function
discussed in the previous section, but rather than spawning a
_non-blocking_ future on the Tokio runtime, it instead spawns a
_blocking_ function on a dedicated thread pool for blocking tasks. For
example:

```
use tokio::task;

# async fn docs() {
task::spawn_blocking(|| {
    // do some compute-heavy work or call synchronous code
});
# }
```

Just like `task::spawn`, `task::spawn_blocking` returns a `JoinHandle`
which we can use to await the result of the blocking operation:

```rust
# use tokio::task;
# async fn docs() -> Result<(), Box<dyn std::error::Error>>{
let join = task::spawn_blocking(|| {
    // do some compute-heavy work or call synchronous code
    "blocking completed"
});

let result = join.await?;
assert_eq!(result, "blocking completed");
# Ok(())
# }
```

#### `block_in_place`

When using the [multi-threaded runtime][rt-multi-thread], the [`task::block_in_place`]
function is also available. Like `task::spawn_blocking`, this function
allows running a blocking operation from an asynchronous context. Unlike
`spawn_blocking`, however, `block_in_place` works by transitioning the
_current_ worker thread to a blocking thread, moving other tasks running on
that thread to another worker thread. This can improve performance by avoiding
context switches.

For example:

```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::task;

# async fn docs() {
let result = task::block_in_place(|| {
    // do some compute-heavy work or call synchronous code
    "blocking completed"
});

assert_eq!(result, "blocking completed");
# }
# }
```

#### `yield_now`

In addition, this module provides a [`task::yield_now`] async function
that is analogous to the standard library's [`thread::yield_now`]. Calling
and `await`ing this function will cause the current task to yield to the
Tokio runtime's scheduler, allowing other tasks to be
scheduled. Eventually, the yielding task will be polled again, allowing it
to execute. For example:

```rust
use tokio::task;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
async {
    task::spawn(async {
        // ...
        println!("spawned task done!")
    });

    // Yield, allowing the newly-spawned task to execute first.
    task::yield_now().await;
    println!("main task done!");
}
# .await;
# }
```

[`task::spawn_blocking`]: crate::task::spawn_blocking
[`task::block_in_place`]: crate::task::block_in_place
[rt-multi-thread]: ../runtime/index.html#threaded-scheduler
[`task::yield_now`]: crate::task::yield_now()
[`thread::yield_now`]: std::thread::yield_now



## tokio::task_local

*Declarative Macro*

Declares a new task-local key of type [`tokio::task::LocalKey`].

# Syntax

The macro wraps any number of static declarations and makes them local to the current task.
Publicity and attributes for each static is preserved. For example:

# Examples

```
# use tokio::task_local;
task_local! {
    pub static ONE: u32;

    #[allow(unused)]
    static TWO: f32;
}
# fn main() {}
```

See [`LocalKey` documentation][`tokio::task::LocalKey`] for more
information.

[`tokio::task::LocalKey`]: struct@crate::task::LocalKey

```rust
macro_rules! task_local {
    () => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty; $($rest:tt)*) => { ... };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $t:ty) => { ... };
}
```



## Module: time

Utilities for tracking time.

This module provides a number of types for executing code after a set period
of time.

* [`Sleep`] is a future that does no work and completes at a specific [`Instant`]
  in time.

* [`Interval`] is a stream yielding a value at a fixed period. It is
  initialized with a [`Duration`] and repeatedly yields each time the duration
  elapses.

* [`Timeout`]: Wraps a future or stream, setting an upper bound to the amount
  of time it is allowed to execute. If the future or stream does not
  complete in time, then it is canceled and an error is returned.

These types are sufficient for handling a large number of scenarios
involving time.

These types must be used from within the context of the [`Runtime`](crate::runtime::Runtime).

# Examples

Wait 100ms and print "100 ms have elapsed"

```
use std::time::Duration;
use tokio::time::sleep;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
sleep(Duration::from_millis(100)).await;
println!("100 ms have elapsed");
# }
```

Require that an operation takes no more than 1s.

```
use tokio::time::{timeout, Duration};

async fn long_future() {
    // do work here
}

# async fn dox() {
let res = timeout(Duration::from_secs(1), long_future()).await;

if res.is_err() {
    println!("operation timed out");
}
# }
```

A simple example using [`interval`] to execute a task every two seconds.

The difference between [`interval`] and [`sleep`] is that an [`interval`]
measures the time since the last tick, which means that `.tick().await` may
wait for a shorter time than the duration specified for the interval
if some time has passed between calls to `.tick().await`.

If the tick in the example below was replaced with [`sleep`], the task
would only be executed once every three seconds, and not every two
seconds.

```
use tokio::time;

async fn task_that_takes_a_second() {
    println!("hello");
    time::sleep(time::Duration::from_secs(1)).await
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let mut interval = time::interval(time::Duration::from_secs(2));
for _i in 0..5 {
    interval.tick().await;
    task_that_takes_a_second().await;
}
# }
```

[`interval`]: crate::time::interval()
[`sleep`]: sleep()



## tokio::try_join

*Declarative Macro*

Waits on multiple concurrent branches, returning when **all** branches
complete with `Ok(_)` or on the first `Err(_)`.

The `try_join!` macro must be used inside of async functions, closures, and
blocks.

Similar to [`join!`], the `try_join!` macro takes a list of async
expressions and evaluates them concurrently on the same task. Each async
expression evaluates to a future and the futures from each expression are
multiplexed on the current task. The `try_join!` macro returns when **all**
branches return with `Ok` or when the **first** branch returns with `Err`.

[`join!`]: macro@join

# Notes

The supplied futures are stored inline and do not require allocating a
`Vec`.

## Runtime characteristics

By running all async expressions on the current task, the expressions are
able to run **concurrently** but not in **parallel**. This means all
expressions are run on the same thread and if one branch blocks the thread,
all other expressions will be unable to continue. If parallelism is
required, spawn each async expression using [`tokio::spawn`] and pass the
join handle to `try_join!`.

[`tokio::spawn`]: crate::spawn

## Fairness

By default, `try_join!`'s generated future rotates which
contained future is polled first whenever it is woken.

This behavior can be overridden by adding `biased;` to the beginning of the
macro usage. See the examples for details. This will cause `try_join` to poll
the futures in the order they appear from top to bottom.

You may want this if your futures may interact in a way where known polling order is significant.

But there is an important caveat to this mode. It becomes your responsibility
to ensure that the polling order of your futures is fair. If for example you
are joining a stream and a shutdown future, and the stream has a
huge volume of messages that takes a long time to finish processing per poll, you should
place the shutdown future earlier in the `try_join!` list to ensure that it is
always polled, and will not be delayed due to the stream future taking a long time to return
`Poll::Pending`.

# Examples

Basic `try_join` with two branches.

```
async fn do_stuff_async() -> Result<(), &'static str> {
    // async work
# Ok(())
}

async fn more_async_work() -> Result<(), &'static str> {
    // more here
# Ok(())
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let res = tokio::try_join!(
    do_stuff_async(),
    more_async_work());

match res {
    Ok((first, second)) => {
        // do something with the values
    }
    Err(err) => {
        println!("processing failed; error = {}", err);
    }
}
# }
```

Using `try_join!` with spawned tasks.

```
use tokio::task::JoinHandle;

async fn do_stuff_async() -> Result<(), &'static str> {
    // async work
# Err("failed")
}

async fn more_async_work() -> Result<(), &'static str> {
    // more here
# Ok(())
}

async fn flatten<T>(handle: JoinHandle<Result<T, &'static str>>) -> Result<T, &'static str> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err),
        Err(err) => Err("handling failed"),
    }
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let handle1 = tokio::spawn(do_stuff_async());
let handle2 = tokio::spawn(more_async_work());
match tokio::try_join!(flatten(handle1), flatten(handle2)) {
    Ok(val) => {
        // do something with the values
    }
    Err(err) => {
        println!("Failed with {}.", err);
        # assert_eq!(err, "failed");
    }
}
# }
```
Using the `biased;` mode to control polling order.

```
async fn do_stuff_async() -> Result<(), &'static str> {
    // async work
# Ok(())
}

async fn more_async_work() -> Result<(), &'static str> {
    // more here
# Ok(())
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let res = tokio::try_join!(
    biased;
    do_stuff_async(),
    more_async_work()
);

match res {
    Ok((first, second)) => {
        // do something with the values
    }
    Err(err) => {
        println!("processing failed; error = {}", err);
    }
}
# }
```

```rust
macro_rules! try_join {
    ($(biased;)? $($future:expr),*) => { ... };
}
```



