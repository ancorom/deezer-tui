**tokio > process > Child**

# Module: process::Child

## Contents

**Functions**

- [`start_kill`](#start_kill) - Attempts to force the child to exit, but does not wait for the request
- [`wait`](#wait) - Waits for the child to exit completely, returning the status that it

---

## tokio::process::Child::start_kill

*Function*

Attempts to force the child to exit, but does not wait for the request
to take effect.

On Unix platforms, this is the equivalent to sending a `SIGKILL`. Note
that on Unix platforms it is possible for a zombie process to remain
after a kill is sent; to avoid this, the caller should ensure that either
`child.wait().await` or `child.try_wait()` is invoked successfully.

```rust
fn start_kill(self: & mut Self) -> io::Result<()>
```



## tokio::process::Child::wait

*Function*

Waits for the child to exit completely, returning the status that it
exited with. This function will continue to have the same return value
after it has been called at least once.

The stdin handle to the child process, if any, will be closed
before waiting. This helps avoid deadlock: it ensures that the
child does not block waiting for input from the parent, while
the parent waits for the child to exit.

If the caller wishes to explicitly control when the child's stdin
handle is closed, they may `.take()` it before calling `.wait()`:

# Cancel safety

This function is cancel safe.

```
# #[cfg(not(unix))]fn main(){}
# #[cfg(unix)]
use tokio::io::AsyncWriteExt;
# #[cfg(unix)]
use tokio::process::Command;
# #[cfg(unix)]
use std::process::Stdio;

# #[cfg(unix)]
#[tokio::main]
async fn main() {
#   if cfg!(miri) { return; } // No `pidfd_spawnp` in miri.
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    tokio::spawn(async move {
        // do something with stdin here...
        stdin.write_all(b"hello world\n").await.unwrap();

        // then drop when finished
        drop(stdin);
    });

    // wait for the process to complete
    let _ = child.wait().await;
}
```

```rust
fn wait(self: & mut Self) -> io::Result<ExitStatus>
```



