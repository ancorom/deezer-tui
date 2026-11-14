**tokio > task > local**

# Module: task::local

## Contents

**Structs**

- [`LocalEnterGuard`](#localenterguard) - Context guard for `LocalSet`
- [`LocalSet`](#localset) - A set of tasks which are executed on the same thread.

**Functions**

- [`spawn_local`](#spawn_local) - Spawns a `!Send` future on the current [`LocalSet`] or [`LocalRuntime`].

---

## tokio::task::local::LocalEnterGuard

*Struct*

Context guard for `LocalSet`

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## tokio::task::local::LocalSet

*Struct*

A set of tasks which are executed on the same thread.

In some cases, it is necessary to run one or more futures that do not
implement [`Send`] and thus are unsafe to send between threads. In these
cases, a [local task set] may be used to schedule one or more `!Send`
futures to run together on the same thread.

For example, the following code will not compile:

```rust,compile_fail
use std::rc::Rc;

#[tokio::main]
async fn main() {
    // `Rc` does not implement `Send`, and thus may not be sent between
    // threads safely.
    let nonsend_data = Rc::new("my nonsend data...");

    let nonsend_data = nonsend_data.clone();
    // Because the `async` block here moves `nonsend_data`, the future is `!Send`.
    // Since `tokio::spawn` requires the spawned future to implement `Send`, this
    // will not compile.
    tokio::spawn(async move {
        println!("{}", nonsend_data);
        // ...
    }).await.unwrap();
}
```

# Use with `run_until`

To spawn `!Send` futures, we can use a local task set to schedule them
on the thread calling [`Runtime::block_on`]. When running inside of the
local task set, we can use [`task::spawn_local`], which can spawn
`!Send` futures. For example:

```rust
use std::rc::Rc;
use tokio::task;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let nonsend_data = Rc::new("my nonsend data...");

// Construct a local task set that can run `!Send` futures.
let local = task::LocalSet::new();

// Run the local task set.
local.run_until(async move {
    let nonsend_data = nonsend_data.clone();
    // `spawn_local` ensures that the future is spawned on the local
    // task set.
    task::spawn_local(async move {
        println!("{}", nonsend_data);
        // ...
    }).await.unwrap();
}).await;
# }
```
**Note:** The `run_until` method can only be used in `#[tokio::main]`,
`#[tokio::test]` or directly inside a call to [`Runtime::block_on`]. It
cannot be used inside a task spawned with `tokio::spawn`.

## Awaiting a `LocalSet`

Additionally, a `LocalSet` itself implements `Future`, completing when
*all* tasks spawned on the `LocalSet` complete. This can be used to run
several futures on a `LocalSet` and drive the whole set until they
complete. For example,

```rust
use tokio::{task, time};
use std::rc::Rc;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let nonsend_data = Rc::new("world");
let local = task::LocalSet::new();

let nonsend_data2 = nonsend_data.clone();
local.spawn_local(async move {
    // ...
    println!("hello {}", nonsend_data2)
});

local.spawn_local(async move {
    time::sleep(time::Duration::from_millis(100)).await;
    println!("goodbye {}", nonsend_data)
});

// ...

local.await;
# }
```
**Note:** Awaiting a `LocalSet` can only be done inside
`#[tokio::main]`, `#[tokio::test]` or directly inside a call to
[`Runtime::block_on`]. It cannot be used inside a task spawned with
`tokio::spawn`.

## Use inside `tokio::spawn`

The two methods mentioned above cannot be used inside `tokio::spawn`, so
to spawn `!Send` futures from inside `tokio::spawn`, we need to do
something else. The solution is to create the `LocalSet` somewhere else,
and communicate with it using an [`mpsc`] channel.

The following example puts the `LocalSet` inside a new thread.
```
# #[cfg(not(target_family = "wasm"))]
# {
use tokio::runtime::Builder;
use tokio::sync::{mpsc, oneshot};
use tokio::task::LocalSet;

// This struct describes the task you want to spawn. Here we include
// some simple examples. The oneshot channel allows sending a response
// to the spawner.
#[derive(Debug)]
enum Task {
    PrintNumber(u32),
    AddOne(u32, oneshot::Sender<u32>),
}

#[derive(Clone)]
struct LocalSpawner {
   send: mpsc::UnboundedSender<Task>,
}

impl LocalSpawner {
    pub fn new() -> Self {
        let (send, mut recv) = mpsc::unbounded_channel();

        let rt = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        std::thread::spawn(move || {
            let local = LocalSet::new();

            local.spawn_local(async move {
                while let Some(new_task) = recv.recv().await {
                    tokio::task::spawn_local(run_task(new_task));
                }
                // If the while loop returns, then all the LocalSpawner
                // objects have been dropped.
            });

            // This will return once all senders are dropped and all
            // spawned tasks have returned.
            rt.block_on(local);
        });

        Self {
            send,
        }
    }

    pub fn spawn(&self, task: Task) {
        self.send.send(task).expect("Thread with LocalSet has shut down.");
    }
}

// This task may do !Send stuff. We use printing a number as an example,
// but it could be anything.
//
// The Task struct is an enum to support spawning many different kinds
// of operations.
async fn run_task(task: Task) {
    match task {
        Task::PrintNumber(n) => {
            println!("{}", n);
        },
        Task::AddOne(n, response) => {
            // We ignore failures to send the response.
            let _ = response.send(n + 1);
        },
    }
}

#[tokio::main]
async fn main() {
    let spawner = LocalSpawner::new();

    let (send, response) = oneshot::channel();
    spawner.spawn(Task::AddOne(10, send));
    let eleven = response.await.unwrap();
    assert_eq!(eleven, 11);
}
# }
```

[`Send`]: trait@std::marker::Send
[local task set]: struct@LocalSet
[`Runtime::block_on`]: method@crate::runtime::Runtime::block_on
[`task::spawn_local`]: fn@spawn_local
[`mpsc`]: mod@crate::sync::mpsc

**Methods:**

- `fn new() -> LocalSet` - Returns a new local task set.
- `fn enter(self: &Self) -> LocalEnterGuard` - Enters the context of this `LocalSet`.
- `fn spawn_local<F>(self: &Self, future: F) -> JoinHandle<<F as >::Output>` - Spawns a `!Send` task onto the local task set.
- `fn block_on<F>(self: &Self, rt: &crate::runtime::Runtime, future: F) -> <F as >::Output` - Runs a future to completion on the provided runtime, driving any local
- `fn run_until<F>(self: &Self, future: F) -> <F as >::Output` - Runs a future to completion on the local set, returning its output.
- `fn id(self: &Self) -> runtime::Id` - Returns the [`Id`] of the current [`LocalSet`] runtime.

**Trait Implementations:**

- **Default**
  - `fn default() -> LocalSet`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, fmt: & mut fmt::Formatter) -> fmt::Result`
- **Future**
  - `fn poll(self: Pin<& mut Self>, cx: & mut std::task::Context) -> Poll<<Self as >::Output>`



## tokio::task::local::spawn_local

*Function*

Spawns a `!Send` future on the current [`LocalSet`] or [`LocalRuntime`].

This is possible when either using one of these types explicitly, or by
opting to use the `"local"` runtime flavor in `tokio::main`:

```ignore
#[tokio::main(flavor = "local")]
```

The spawned future will run on the same thread that called `spawn_local`.

The provided future will start running in the background immediately
when `spawn_local` is called, even if you don't await the returned
`JoinHandle`.

# Panics

This function panics if called outside of a [`LocalSet`] or [`LocalRuntime`].

Note that if [`tokio::spawn`] is used from within a `LocalSet`, the
resulting new task will _not_ be inside the `LocalSet`, so you must use
`spawn_local` if you want to stay within the `LocalSet`.

# Examples

With `LocalSet`:

```rust
use std::rc::Rc;
use tokio::task;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let nonsend_data = Rc::new("my nonsend data...");

let local = task::LocalSet::new();

// Run the local task set.
local.run_until(async move {
    let nonsend_data = nonsend_data.clone();
    task::spawn_local(async move {
        println!("{}", nonsend_data);
        // ...
    }).await.unwrap();
}).await;
# }
```
With local runtime flavor.

```rust
#[tokio::main(flavor = "local")]
async fn main() {
    let join = tokio::task::spawn_local(async {
        println!("my nonsend data...")
    });

   join.await.unwrap()
 }

```

[`LocalSet`]: struct@crate::task::LocalSet
[`LocalRuntime`]: struct@crate::runtime::LocalRuntime
[`tokio::spawn`]: fn@crate::task::spawn

```rust
fn spawn_local<F>(future: F) -> crate::runtime::task::JoinHandle<<F as >::Output>
```



