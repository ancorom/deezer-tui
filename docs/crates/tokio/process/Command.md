**tokio > process > Command**

# Module: process::Command

## Contents

**Functions**

- [`arg`](#arg) - Adds an argument to pass to the program.
- [`args`](#args) - Adds multiple arguments to pass to the program.
- [`kill_on_drop`](#kill_on_drop) - Controls whether a `kill` operation should be invoked on a spawned child
- [`output`](#output) - Executes the command as a child process, waiting for it to finish and
- [`spawn`](#spawn) - Executes the command as a child process, returning a handle to it.
- [`status`](#status) - Executes the command as a child process, waiting for it to finish and

---

## tokio::process::Command::arg

*Function*

Adds an argument to pass to the program.

Only one argument can be passed per use. So instead of:

```no_run
let mut command = tokio::process::Command::new("sh");
command.arg("-C /path/to/repo");

# let _ = command.output(); // assert borrow checker
```

usage would be:

```no_run
let mut command = tokio::process::Command::new("sh");
command.arg("-C");
command.arg("/path/to/repo");

# let _ = command.output(); // assert borrow checker
```

To pass multiple arguments see [`args`].

[`args`]: method@Self::args

# Examples

Basic usage:

```no_run
# async fn test() { // allow using await
use tokio::process::Command;

let output = Command::new("ls")
        .arg("-l")
        .arg("-a")
        .output().await.unwrap();
# }

```

```rust
fn arg<S>(self: & mut Self, arg: S) -> & mut Command
```



## tokio::process::Command::args

*Function*

Adds multiple arguments to pass to the program.

To pass a single argument see [`arg`].

[`arg`]: method@Self::arg

# Examples

Basic usage:

```no_run
# async fn test() { // allow using await
use tokio::process::Command;

let output = Command::new("ls")
        .args(&["-l", "-a"])
        .output().await.unwrap();
# }
```

```rust
fn args<I, S>(self: & mut Self, args: I) -> & mut Command
```



## tokio::process::Command::kill_on_drop

*Function*

Controls whether a `kill` operation should be invoked on a spawned child
process when its corresponding `Child` handle is dropped.

By default, this value is assumed to be `false`, meaning the next spawned
process will not be killed on drop, similar to the behavior of the standard
library.

# Caveats

On Unix platforms processes must be "reaped" by their parent process after
they have exited in order to release all OS resources. A child process which
has exited, but has not yet been reaped by its parent is considered a "zombie"
process. Such processes continue to count against limits imposed by the system,
and having too many zombie processes present can prevent additional processes
from being spawned.

Although issuing a `kill` signal to the child process is a synchronous
operation, the resulting zombie process cannot be `.await`ed inside of the
destructor to avoid blocking other tasks. The tokio runtime will, on a
best-effort basis, attempt to reap and clean up such processes in the
background, but no additional guarantees are made with regard to
how quickly or how often this procedure will take place.

If stronger guarantees are required, it is recommended to avoid dropping
a [`Child`] handle where possible, and instead utilize `child.wait().await`
or `child.kill().await` where possible.

```rust
fn kill_on_drop(self: & mut Self, kill_on_drop: bool) -> & mut Command
```



## tokio::process::Command::output

*Function*

Executes the command as a child process, waiting for it to finish and
collecting all of its output.

> **Note**: this method, unlike the standard library, will
> unconditionally configure the stdout/stderr handles to be pipes, even
> if they have been previously configured. If this is not desired then
> the `spawn` method should be used in combination with the
> `wait_with_output` method on child.

This method will return a future representing the collection of the
child process's stdout/stderr. It will resolve to
the `Output` type in the standard library, containing `stdout` and
`stderr` as `Vec<u8>` along with an `ExitStatus` representing how the
process exited.

All I/O this child does will be associated with the current default
event loop.

The destructor of the future returned by this function will kill
the child if [`kill_on_drop`] is set to true.

[`kill_on_drop`]: fn@Self::kill_on_drop

# Errors

This future will return an error if the child process cannot be spawned
or if there is an error while awaiting its status.

On Unix platforms this method will fail with `std::io::ErrorKind::WouldBlock`
if the system process limit is reached (which includes other applications
running on the system).
# Examples

Basic usage:

```no_run
use tokio::process::Command;

async fn run_ls() {
    let output: std::process::Output = Command::new("ls")
        .output()
        .await
        .expect("ls command failed to run");
    println!("stderr of ls: {:?}", output.stderr);
}
```

```rust
fn output(self: & mut Self) -> impl Trait
```



## tokio::process::Command::spawn

*Function*

Executes the command as a child process, returning a handle to it.

By default, stdin, stdout and stderr are inherited from the parent.

This method will spawn the child process synchronously and return a
handle to a future-aware child process. The `Child` returned implements
`Future` itself to acquire the `ExitStatus` of the child, and otherwise
the `Child` has methods to acquire handles to the stdin, stdout, and
stderr streams.

All I/O this child does will be associated with the current default
event loop.

# Examples

Basic usage:

```no_run
use tokio::process::Command;

async fn run_ls() -> std::process::ExitStatus {
    Command::new("ls")
        .spawn()
        .expect("ls command failed to start")
        .wait()
        .await
        .expect("ls command failed to run")
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

# Errors

On Unix platforms this method will fail with `std::io::ErrorKind::WouldBlock`
if the system process limit is reached (which includes other applications
running on the system).

```rust
fn spawn(self: & mut Self) -> io::Result<Child>
```



## tokio::process::Command::status

*Function*

Executes the command as a child process, waiting for it to finish and
collecting its exit status.

By default, stdin, stdout and stderr are inherited from the parent.
If any input/output handles are set to a pipe then they will be immediately
closed after the child is spawned.

All I/O this child does will be associated with the current default
event loop.

The destructor of the future returned by this function will kill
the child if [`kill_on_drop`] is set to true.

[`kill_on_drop`]: fn@Self::kill_on_drop

# Errors

This future will return an error if the child process cannot be spawned
or if there is an error while awaiting its status.

On Unix platforms this method will fail with `std::io::ErrorKind::WouldBlock`
if the system process limit is reached (which includes other applications
running on the system).

# Examples

Basic usage:

```no_run
use tokio::process::Command;

async fn run_ls() -> std::process::ExitStatus {
    Command::new("ls")
        .status()
        .await
        .expect("ls command failed to run")
}
```

```rust
fn status(self: & mut Self) -> impl Trait
```



