**tokio > process**

# Module: process

## Contents

**Structs**

- [`Child`](#child) - Representation of a child process spawned onto an event loop.
- [`ChildStderr`](#childstderr) - The standard error stream for spawned children.
- [`ChildStdin`](#childstdin) - The standard input stream for spawned children.
- [`ChildStdout`](#childstdout) - The standard output stream for spawned children.
- [`Command`](#command) - This structure mimics the API of [`std::process::Command`] found in the standard library, but

---

## tokio::process::Child

*Struct*

Representation of a child process spawned onto an event loop.

# Caveats
Similar to the behavior to the standard library, and unlike the futures
paradigm of dropping-implies-cancellation, a spawned process will, by
default, continue to execute even after the `Child` handle has been dropped.

The `Command::kill_on_drop` method can be used to modify this behavior
and kill the child process if the `Child` wrapper is dropped before it
has exited.

**Fields:**
- `stdin: Option<ChildStdin>` - The handle for writing to the child's standard input (stdin), if it has
- `stdout: Option<ChildStdout>` - The handle for reading from the child's standard output (stdout), if it
- `stderr: Option<ChildStderr>` - The handle for reading from the child's standard error (stderr), if it

**Methods:**

- `fn id(self: &Self) -> Option<u32>` - Returns the OS-assigned process identifier associated with this child
- `fn start_kill(self: & mut Self) -> io::Result<()>` - Attempts to force the child to exit, but does not wait for the request
- `fn kill(self: & mut Self) -> io::Result<()>` - Forces the child to exit.
- `fn wait(self: & mut Self) -> io::Result<ExitStatus>` - Waits for the child to exit completely, returning the status that it
- `fn try_wait(self: & mut Self) -> io::Result<Option<ExitStatus>>` - Attempts to collect the exit status of the child if it has already
- `fn wait_with_output(self: Self) -> io::Result<Output>` - Returns a future that will resolve to an `Output`, containing the exit

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## tokio::process::ChildStderr

*Struct*

The standard error stream for spawned children.

This type implements the `AsyncRead` trait to read data from the stderr
handle of a child process asynchronously.

**Methods:**

- `fn into_owned_fd(self: Self) -> io::Result<OwnedFd>` - Convert into [`OwnedFd`].
- `fn from_std(inner: std::process::ChildStderr) -> io::Result<Self>` - Creates an asynchronous `ChildStderr` from a synchronous one.

**Trait Implementations:**

- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryInto**
  - `fn try_into(self: Self) -> Result<Stdio, <Self as >::Error>`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`



## tokio::process::ChildStdin

*Struct*

The standard input stream for spawned children.

This type implements the `AsyncWrite` trait to pass data to the stdin
handle of a child process asynchronously.

**Methods:**

- `fn into_owned_fd(self: Self) -> io::Result<OwnedFd>` - Convert into [`OwnedFd`].
- `fn from_std(inner: std::process::ChildStdin) -> io::Result<Self>` - Creates an asynchronous `ChildStdin` from a synchronous one.

**Trait Implementations:**

- **AsyncWrite**
  - `fn poll_write(self: Pin<& mut Self>, cx: & mut Context, buf: &[u8]) -> Poll<io::Result<usize>>`
  - `fn poll_flush(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_shutdown(self: Pin<& mut Self>, cx: & mut Context) -> Poll<io::Result<()>>`
  - `fn poll_write_vectored(self: Pin<& mut Self>, cx: & mut Context, bufs: &[io::IoSlice]) -> Poll<Result<usize, io::Error>>`
  - `fn is_write_vectored(self: &Self) -> bool`
- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryInto**
  - `fn try_into(self: Self) -> Result<Stdio, <Self as >::Error>`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`



## tokio::process::ChildStdout

*Struct*

The standard output stream for spawned children.

This type implements the `AsyncRead` trait to read data from the stdout
handle of a child process asynchronously.

**Methods:**

- `fn into_owned_fd(self: Self) -> io::Result<OwnedFd>` - Convert into [`OwnedFd`].
- `fn from_std(inner: std::process::ChildStdout) -> io::Result<Self>` - Creates an asynchronous `ChildStdout` from a synchronous one.

**Trait Implementations:**

- **AsFd**
  - `fn as_fd(self: &Self) -> BorrowedFd`
- **AsyncRead**
  - `fn poll_read(self: Pin<& mut Self>, cx: & mut Context, buf: & mut ReadBuf) -> Poll<io::Result<()>>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **TryInto**
  - `fn try_into(self: Self) -> Result<Stdio, <Self as >::Error>`
- **AsRawFd**
  - `fn as_raw_fd(self: &Self) -> RawFd`



## tokio::process::Command

*Struct*

This structure mimics the API of [`std::process::Command`] found in the standard library, but
replaces functions that create a process with an asynchronous variant. The main provided
asynchronous functions are [spawn](Command::spawn), [status](Command::status), and
[output](Command::output).

`Command` uses asynchronous versions of some `std` types (for example [`Child`]).

[`std::process::Command`]: std::process::Command
[`Child`]: struct@Child

**Methods:**

- `fn new<S>(program: S) -> Command` - Constructs a new `Command` for launching the program at
- `fn as_std(self: &Self) -> &StdCommand` - Cheaply convert to a `&std::process::Command` for places where the type from the standard
- `fn as_std_mut(self: & mut Self) -> & mut StdCommand` - Cheaply convert to a `&mut std::process::Command` for places where the type from the
- `fn into_std(self: Self) -> StdCommand` - Cheaply convert into a `std::process::Command`.
- `fn arg<S>(self: & mut Self, arg: S) -> & mut Command` - Adds an argument to pass to the program.
- `fn args<I, S>(self: & mut Self, args: I) -> & mut Command` - Adds multiple arguments to pass to the program.
- `fn env<K, V>(self: & mut Self, key: K, val: V) -> & mut Command` - Inserts or updates an environment variable mapping.
- `fn envs<I, K, V>(self: & mut Self, vars: I) -> & mut Command` - Adds or updates multiple environment variable mappings.
- `fn env_remove<K>(self: & mut Self, key: K) -> & mut Command` - Removes an environment variable mapping.
- `fn env_clear(self: & mut Self) -> & mut Command` - Clears the entire environment map for the child process.
- `fn current_dir<P>(self: & mut Self, dir: P) -> & mut Command` - Sets the working directory for the child process.
- `fn stdin<T>(self: & mut Self, cfg: T) -> & mut Command` - Sets configuration for the child process's standard input (stdin) handle.
- `fn stdout<T>(self: & mut Self, cfg: T) -> & mut Command` - Sets configuration for the child process's standard output (stdout) handle.
- `fn stderr<T>(self: & mut Self, cfg: T) -> & mut Command` - Sets configuration for the child process's standard error (stderr) handle.
- `fn kill_on_drop(self: & mut Self, kill_on_drop: bool) -> & mut Command` - Controls whether a `kill` operation should be invoked on a spawned child
- `fn uid(self: & mut Self, id: u32) -> & mut Command` - Sets the child process's user ID. This translates to a
- `fn gid(self: & mut Self, id: u32) -> & mut Command` - Similar to `uid` but sets the group ID of the child process. This has
- `fn arg0<S>(self: & mut Self, arg: S) -> & mut Command` - Sets executable argument.
- `fn pre_exec<F>(self: & mut Self, f: F) -> & mut Command` - Schedules a closure to be run just before the `exec` function is
- `fn process_group(self: & mut Self, pgroup: i32) -> & mut Command` - Sets the process group ID (PGID) of the child process. Equivalent to a
- `fn spawn(self: & mut Self) -> io::Result<Child>` - Executes the command as a child process, returning a handle to it.
- `fn status(self: & mut Self) -> impl Trait` - Executes the command as a child process, waiting for it to finish and
- `fn output(self: & mut Self) -> impl Trait` - Executes the command as a child process, waiting for it to finish and
- `fn get_kill_on_drop(self: &Self) -> bool` - Returns the boolean value that was previously set by [`Command::kill_on_drop`].

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(std: StdCommand) -> Command`



