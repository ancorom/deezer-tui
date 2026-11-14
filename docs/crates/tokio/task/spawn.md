**tokio > task > spawn**

# Module: task::spawn

## Contents

**Functions**

- [`spawn`](#spawn) - Spawns a new asynchronous task, returning a

---

## tokio::task::spawn::spawn

*Function*

Spawns a new asynchronous task, returning a
[`JoinHandle`] for it.

The provided future will start running in the background immediately
when `spawn` is called, even if you don't await the returned
[`JoinHandle`].

Spawning a task enables the task to execute concurrently to other tasks. The
spawned task may execute on the current thread, or it may be sent to a
different thread to be executed. The specifics depend on the current
[`Runtime`](crate::runtime::Runtime) configuration. In a
[running runtime][running-runtime], the task will start immediately in the
background. On a blocked runtime, the user must drive the runtime forward (for
example, by calling [`Runtime::block_on`](crate::runtime::Runtime::block_on)).

It is guaranteed that spawn will not synchronously poll the task being spawned.
This means that calling spawn while holding a lock does not pose a risk of
deadlocking with the spawned task.

There is no guarantee that a spawned task will execute to completion.
When a runtime is shutdown, all outstanding tasks are dropped,
regardless of the lifecycle of that task.

This function must be called from the context of a Tokio runtime. Tasks running on
the Tokio runtime are always inside its context, but you can also enter the context
using the [`Runtime::enter`](crate::runtime::Runtime::enter()) method.

[running-runtime]: ../runtime/index.html#driving-the-runtime

# Examples

In this example, a server is started and `spawn` is used to start a new task
that processes each received connection.

```no_run
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::net::{TcpListener, TcpStream};

use std::io;

async fn process(socket: TcpStream) {
    // ...
# drop(socket);
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Process each socket concurrently.
            process(socket).await
        });
    }
}
# }
```

To run multiple tasks in parallel and receive their results, join
handles can be stored in a vector.
```
# #[tokio::main(flavor = "current_thread")] async fn main() {
async fn my_background_op(id: i32) -> String {
    let s = format!("Starting background task {}.", id);
    println!("{}", s);
    s
}

let ops = vec![1, 2, 3];
let mut tasks = Vec::with_capacity(ops.len());
for op in ops {
    // This call will make them start running in the background
    // immediately.
    tasks.push(tokio::spawn(my_background_op(op)));
}

let mut outputs = Vec::with_capacity(tasks.len());
for task in tasks {
    outputs.push(task.await.unwrap());
}
println!("{:?}", outputs);
# }
```
This example pushes the tasks to `outputs` in the order they were
started in. If you do not care about the ordering of the outputs, then
you can also use a [`JoinSet`].

[`JoinSet`]: struct@crate::task::JoinSet

# Panics

Panics if called from **outside** of the Tokio runtime.

# Using `!Send` values from a task

The task supplied to `spawn` must implement `Send`. However, it is
possible to **use** `!Send` values from the task as long as they only
exist between calls to `.await`.

For example, this will work:

```
use tokio::task;

use std::rc::Rc;

fn use_rc(rc: Rc<()>) {
    // Do stuff w/ rc
# drop(rc);
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
tokio::spawn(async {
    // Force the `Rc` to stay in a scope with no `.await`
    {
        let rc = Rc::new(());
        use_rc(rc.clone());
    }

    task::yield_now().await;
}).await.unwrap();
# }
```

This will **not** work:

```compile_fail
use tokio::task;

use std::rc::Rc;

fn use_rc(rc: Rc<()>) {
    // Do stuff w/ rc
# drop(rc);
}

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let rc = Rc::new(());

        task::yield_now().await;

        use_rc(rc.clone());
    }).await.unwrap();
}
```

Holding on to a `!Send` value across calls to `.await` will result in
an unfriendly compile error message similar to:

```text
`[... some type ...]` cannot be sent between threads safely
```

or:

```text
error[E0391]: cycle detected when processing `main`
```

```rust
fn spawn<F>(future: F) -> crate::task::JoinHandle<<F as >::Output>
```



