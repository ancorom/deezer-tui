**tokio > signal > unix > Signal**

# Module: signal::unix::Signal

## Contents

**Functions**

- [`recv`](#recv) - Receives the next signal notification event.

---

## tokio::signal::unix::Signal::recv

*Function*

Receives the next signal notification event.

Although this returns `Option<()>`, it will never actually return `None`.
This was accidentally exposed and would be a breaking change to be removed.

# Cancel safety

This method is cancel safe. If you use it as a branch in
[`tokio::select!`](crate::select) and another branch
completes first, then it is guaranteed that no signal is lost.

# Examples

Wait for `SIGHUP`

```rust,no_run
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
```

```rust
fn recv(self: & mut Self) -> Option<()>
```



