**tokio > sync**

# Module: sync

## Contents

**Modules**

- [`broadcast`](#broadcast) - A multi-producer, multi-consumer broadcast queue. Each sent value is seen by
- [`futures`](#futures) - Named future types.
- [`mpsc`](#mpsc) - A multi-producer, single-consumer queue for sending values between
- [`oneshot`](#oneshot) - A one-shot channel is used for sending a single message between
- [`watch`](#watch) - A multi-producer, multi-consumer channel that only retains the *last* sent

---

## Module: broadcast

A multi-producer, multi-consumer broadcast queue. Each sent value is seen by
all consumers.

A [`Sender`] is used to broadcast values to **all** connected [`Receiver`]
values. [`Sender`] handles are clone-able, allowing concurrent send and
receive actions. [`Sender`] and [`Receiver`] are both `Send` and `Sync` as
long as `T` is `Send`.

When a value is sent, **all** [`Receiver`] handles are notified and will
receive the value. The value is stored once inside the channel and cloned on
demand for each receiver. Once all receivers have received a clone of the
value, the value is released from the channel.

A channel is created by calling [`channel`], specifying the maximum number
of messages the channel can retain at any given time.

New [`Receiver`] handles are created by calling [`Sender::subscribe`]. The
returned [`Receiver`] will receive values sent **after** the call to
`subscribe`.

This channel is also suitable for the single-producer multi-consumer
use-case, where a single sender broadcasts values to many receivers.

## Lagging

As sent messages must be retained until **all** [`Receiver`] handles receive
a clone, broadcast channels are susceptible to the "slow receiver" problem.
In this case, all but one receiver are able to receive values at the rate
they are sent. Because one receiver is stalled, the channel starts to fill
up.

This broadcast channel implementation handles this case by setting a hard
upper bound on the number of values the channel may retain at any given
time. This upper bound is passed to the [`channel`] function as an argument.
The provided capacity is rounded **up** to the next power of two; that
rounded size is the number of messages the ring buffer can hold, and is what
lag detection is based on. For example, `channel(3)` allocates a buffer of
length 4, so a receiver only lags once it falls more than 4 messages behind
the sender.

If a value is sent when the channel is at capacity, the oldest value
currently held by the channel is overwritten. This frees up space for the
new value. Any receiver that has not yet seen the overwritten value will
return [`RecvError::Lagged`] the next time [`recv`] (or
[`try_recv`](Receiver::try_recv)) is called. The error carries the number of
messages that were dropped before the receiver's cursor and are therefore
no longer available.

Returning [`RecvError::Lagged`] does **not** close or disconnect the
receiver. The lagging receiver's internal cursor is advanced to the oldest
value still retained by the channel. The **next** successful call to
[`recv`] / [`try_recv`](Receiver::try_recv) returns that oldest retained
value (unless further sends overwrite it again before the receiver reads
it). Subsequent receives then continue in send order from there.

This behavior enables a receiver to detect when it has lagged so far behind
that data has been dropped. The caller may decide how to respond to this:
either by aborting its task or by tolerating lost messages and resuming
consumption of the channel.

## Closing

When **all** [`Sender`] handles have been dropped, no new values may be
sent. At this point, the channel is "closed". Once a receiver has received
all values retained by the channel, the next call to [`recv`] will return
with [`RecvError::Closed`].

When a [`Receiver`] handle is dropped, any messages not read by the receiver
will be marked as read. If this receiver was the only one not to have read
that message, the message will be dropped at this point.

[`Sender`]: crate::sync::broadcast::Sender
[`Sender::subscribe`]: crate::sync::broadcast::Sender::subscribe
[`Receiver`]: crate::sync::broadcast::Receiver
[`channel`]: crate::sync::broadcast::channel
[`RecvError::Lagged`]: crate::sync::broadcast::error::RecvError::Lagged
[`RecvError::Closed`]: crate::sync::broadcast::error::RecvError::Closed
[`recv`]: crate::sync::broadcast::Receiver::recv

# Examples

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

Handling lag

```
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
// Capacity 2 → ring buffer of length 2.
let (tx, mut rx) = broadcast::channel(2);

tx.send(10).unwrap();
tx.send(20).unwrap();
// Overwrites 10; receiver has not read it yet.
tx.send(30).unwrap();

// One message (10) was dropped; cursor moves to the oldest retained value (20).
assert!(matches!(rx.recv().await, Err(RecvError::Lagged(1))));

// At this point, we can abort or continue with lost messages.
// Continuing resumes from the oldest retained message.
assert_eq!(20, rx.recv().await.unwrap());
assert_eq!(30, rx.recv().await.unwrap());
# }
```



## Module: futures

Named future types.



## Module: mpsc

A multi-producer, single-consumer queue for sending values between
asynchronous tasks.

This module provides two variants of the channel: bounded and unbounded. The
bounded variant has a limit on the number of messages that the channel can
store, and if this limit is reached, trying to send another message will
wait until a message is received from the channel. An unbounded channel has
an infinite capacity, so the `send` method will always complete immediately.
This makes the [`UnboundedSender`] usable from both synchronous and
asynchronous code.

Similar to the `mpsc` channels provided by `std`, the channel constructor
functions provide separate send and receive handles, [`Sender`] and
[`Receiver`] for the bounded channel, [`UnboundedSender`] and
[`UnboundedReceiver`] for the unbounded channel. If there is no message to read,
the current task will be notified when a new value is sent. [`Sender`] and
[`UnboundedSender`] allow sending values into the channel. If the bounded
channel is at capacity, the send is rejected and the task will be notified
when additional capacity is available. In other words, the channel provides
backpressure.

This channel is also suitable for the single-producer single-consumer
use-case. (Unless you only need to send one message, in which case you
should use the [oneshot] channel.)

# Disconnection

When all [`Sender`] handles have been dropped, it is no longer
possible to send values into the channel. This is considered the termination
event of the stream. Once all senders have been dropped and any remaining
buffered values have been received, `Receiver::recv` returns `None`
(and `Receiver::poll_recv` returns `Poll::Ready(None)`).

If the [`Receiver`] handle is dropped, then messages can no longer
be read out of the channel. In this case, all further attempts to send will
result in an error. Additionally, all unread messages will be drained from the
channel and dropped.

# Clean Shutdown

When the [`Receiver`] is dropped, it is possible for unprocessed messages to
remain in the channel. Instead, it is usually desirable to perform a "clean"
shutdown. To do this, the receiver first calls `close`, which will prevent
any further messages to be sent into the channel. Then, the receiver
consumes the channel to completion, at which point the receiver can be
dropped.

# Communicating between sync and async code

When you want to communicate between synchronous and asynchronous code, there
are two situations to consider:

**Bounded channel**: If you need a bounded channel, you should use a bounded
Tokio `mpsc` channel for both directions of communication. Instead of calling
the async [`send`][bounded-send] or [`recv`][bounded-recv] methods, in
synchronous code you will need to use the [`blocking_send`][blocking-send] or
[`blocking_recv`][blocking-recv] methods.

**Unbounded channel**: You should use the kind of channel that matches where
the receiver is. So for sending a message _from async to sync_, you should
use [the standard library unbounded channel][std-unbounded] or
[crossbeam][crossbeam-unbounded].  Similarly, for sending a message _from sync
to async_, you should use an unbounded Tokio `mpsc` channel.

Please be aware that the above remarks were written with the `mpsc` channel
in mind, but they can also be generalized to other kinds of channels. In
general, any channel method that isn't marked async can be called anywhere,
including outside of the runtime. For example, sending a message on a
[oneshot] channel from outside the runtime is perfectly fine.

# Multiple runtimes

The `mpsc` channel is runtime agnostic. You can freely move it between
different instances of the Tokio runtime or even use it from non-Tokio
runtimes.

When used in a Tokio runtime, it participates in
[cooperative scheduling](crate::task::coop#cooperative-scheduling) to avoid
starvation. This feature does not apply when used from non-Tokio runtimes.

As an exception, methods ending in `_timeout` are not runtime agnostic
because they require access to the Tokio timer. See the documentation of
each `*_timeout` method for more information on its use.

# Allocation behavior

<div class="warning">The implementation details described in this section may change in future
Tokio releases.</div>

The mpsc channel stores elements in blocks. Blocks are organized in a linked list. Sending
pushes new elements onto the block at the front of the list, and receiving pops them off the
one at the back. A block can hold 32 messages on a 64-bit target and 16 messages on a 32-bit
target. This number is independent of channel and message size. Each block also stores 4
pointer-sized values for bookkeeping (so on a 64-bit machine, each message has 1 byte of
overhead).

When all values in a block have been received, it becomes empty. It will then be freed, unless
the channel's first block (where newly-sent elements are being stored) has no next block. In
that case, the empty block is reused as the next block.

[`Sender`]: crate::sync::mpsc::Sender
[`Receiver`]: crate::sync::mpsc::Receiver
[bounded-send]: crate::sync::mpsc::Sender::send()
[bounded-recv]: crate::sync::mpsc::Receiver::recv()
[blocking-send]: crate::sync::mpsc::Sender::blocking_send()
[blocking-recv]: crate::sync::mpsc::Receiver::blocking_recv()
[`UnboundedSender`]: crate::sync::mpsc::UnboundedSender
[`UnboundedReceiver`]: crate::sync::mpsc::UnboundedReceiver
[oneshot]: crate::sync::oneshot
[`Handle::block_on`]: crate::runtime::Handle::block_on()
[std-unbounded]: std::sync::mpsc::channel
[crossbeam-unbounded]: https://docs.rs/crossbeam/*/crossbeam/channel/fn.unbounded.html
[`send_timeout`]: crate::sync::mpsc::Sender::send_timeout



## Module: oneshot

A one-shot channel is used for sending a single message between
asynchronous tasks. The [`channel`] function is used to create a
[`Sender`] and [`Receiver`] handle pair that form the channel.

The `Sender` handle is used by the producer to send the value.
The `Receiver` handle is used by the consumer to receive the value.

Each handle can be used on separate tasks.

Since the `send` method is not async, it can be used anywhere. This includes
sending between two runtimes, and using it from non-async code.

If the [`Receiver`] is closed before receiving a message which has already
been sent, the message will remain in the channel until the receiver is
dropped, at which point the message will be dropped immediately.

# Examples

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    if let Err(_) = tx.send(3) {
        println!("the receiver dropped");
    }
});

match rx.await {
    Ok(v) => println!("got = {:?}", v),
    Err(_) => println!("the sender dropped"),
}
# }
```

If the sender is dropped without sending, the receiver will fail with
[`error::RecvError`]:

```
use tokio::sync::oneshot;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, rx) = oneshot::channel::<u32>();

tokio::spawn(async move {
    drop(tx);
});

match rx.await {
    Ok(_) => panic!("This doesn't happen"),
    Err(_) => println!("the sender dropped"),
}
# }
```

To use a `oneshot` channel in a `tokio::select!` loop, add `&mut` in front of
the channel.

```
use tokio::sync::oneshot;
use tokio::time::{interval, sleep, Duration};

# #[tokio::main(flavor = "current_thread")]
# async fn _doc() {}
# #[tokio::main(flavor = "current_thread", start_paused = true)]
# async fn main() {
let (send, mut recv) = oneshot::channel();
let mut interval = interval(Duration::from_millis(100));

# let handle =
tokio::spawn(async move {
    sleep(Duration::from_secs(1)).await;
    send.send("shut down").unwrap();
});

loop {
    tokio::select! {
        _ = interval.tick() => println!("Another 100ms"),
        msg = &mut recv => {
            println!("Got message: {}", msg.unwrap());
            break;
        }
    }
}
# handle.await.unwrap();
# }
```

To use a `Sender` from a destructor, put it in an [`Option`] and call
[`Option::take`].

```
use tokio::sync::oneshot;

struct SendOnDrop {
    sender: Option<oneshot::Sender<&'static str>>,
}
impl Drop for SendOnDrop {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            // Using `let _ =` to ignore send errors.
            let _ = sender.send("I got dropped!");
        }
    }
}

# #[tokio::main(flavor = "current_thread")]
# async fn _doc() {}
# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (send, recv) = oneshot::channel();

let send_on_drop = SendOnDrop { sender: Some(send) };
drop(send_on_drop);

assert_eq!(recv.await, Ok("I got dropped!"));
# }
```



## Module: watch

A multi-producer, multi-consumer channel that only retains the *last* sent
value.

This channel is useful for watching for changes to a value from multiple
points in the code base, for example, changes to configuration values.

# Usage

[`channel`] returns a [`Sender`] / [`Receiver`] pair. These are the producer
and consumer halves of the channel. The channel is created with an initial
value.

Each [`Receiver`] independently tracks the last value *seen* by its caller.

To access the **current** value stored in the channel and mark it as *seen*
by a given [`Receiver`], use [`Receiver::borrow_and_update()`].

To access the current value **without** marking it as *seen*, use
[`Receiver::borrow()`]. (If the value has already been marked *seen*,
[`Receiver::borrow()`] is equivalent to [`Receiver::borrow_and_update()`].)

For more information on when to use these methods, see
[here](#borrow_and_update-versus-borrow).

## Change notifications

The [`Receiver`] half provides an asynchronous [`changed`] method. This
method is ready when a new, *unseen* value is sent via the [`Sender`] half.

* [`Receiver::changed()`] returns:
    * `Ok(())` on receiving a new value.
    * `Err(`[`RecvError`](error::RecvError)`)` if the
      channel has been closed __AND__ the current value is *seen*.
* If the current value is *unseen* when calling [`changed`], then
  [`changed`] will return immediately. If the current value is *seen*, then
  it will sleep until either a new message is sent via the [`Sender`] half,
  or the [`Sender`] is dropped.
* On completion, the [`changed`] method marks the new value as *seen*.
* At creation, the initial value is considered *seen*. In other words,
  [`Receiver::changed()`] will not return until a subsequent value is sent.
* New [`Receiver`] instances can be created with [`Sender::subscribe()`].
  The current value at the time the [`Receiver`] is created is considered
  *seen*.

## [`changed`] versus [`has_changed`]

The [`Receiver`] half provides two methods for checking for changes
in the channel, [`has_changed`] and [`changed`].

* [`has_changed`] is a *synchronous* method that checks whether the current
  value is seen or not and returns a boolean. This method does __not__ mark the
  value as seen.

* [`changed`] is an *asynchronous* method that will return once an unseen
  value is in the channel. This method does mark the value as seen.

Note there are two behavioral differences on when these two methods return
an error.

- [`has_changed`] errors if and only if the channel is closed.
- [`changed`] errors if the channel has been closed __AND__
  the current value is seen.

See the example below that shows how these methods have different fallibility.

## [`borrow_and_update`] versus [`borrow`]

If the receiver intends to await notifications from [`changed`] in a loop,
[`Receiver::borrow_and_update()`] should be preferred over
[`Receiver::borrow()`].  This avoids a potential race where a new value is
sent between [`changed`] being ready and the value being read. (If
[`Receiver::borrow()`] is used, the loop may run twice with the same value.)

If the receiver is only interested in the current value, and does not intend
to wait for changes, then [`Receiver::borrow()`] can be used. It may be more
convenient to use [`borrow`](Receiver::borrow) since it's an `&self`
method---[`borrow_and_update`](Receiver::borrow_and_update) requires `&mut
self`.

# Examples

The following example prints `hello! world! `.

```
use tokio::sync::watch;
use tokio::time::{Duration, sleep};

# async fn dox() -> Result<(), Box<dyn std::error::Error>> {
let (tx, mut rx) = watch::channel("hello");

tokio::spawn(async move {
    // Use the equivalent of a "do-while" loop so the initial value is
    // processed before awaiting the `changed()` future.
    loop {
        println!("{}! ", *rx.borrow_and_update());
        if rx.changed().await.is_err() {
            break;
        }
    }
});

sleep(Duration::from_millis(100)).await;
tx.send("world")?;
# Ok(())
# }
```

Difference on fallibility of [`changed`] versus [`has_changed`].
```
use tokio::sync::watch;

# #[tokio::main(flavor = "current_thread")]
# async fn main() {
let (tx, mut rx) = watch::channel("hello");
tx.send("goodbye").unwrap();
drop(tx);

// `has_changed` does not mark the value as seen and errors
// since the channel is closed.
assert!(rx.has_changed().is_err());

// `changed` returns Ok since the value is not already marked as seen
// even if the channel is closed.
assert!(rx.changed().await.is_ok());

// The `changed` call above marks the value as seen.
// The next `changed` call now returns an error as the channel is closed
// AND the current value is seen.
assert!(rx.changed().await.is_err());
# }
```

# Closing

[`Sender::is_closed`] and [`Sender::closed`] allow the producer to detect
when all [`Receiver`] handles have been dropped. This indicates that there
is no further interest in the values being produced and work can be stopped.

The value in the channel will not be dropped until all senders and all
receivers have been dropped.

# Thread safety

Both [`Sender`] and [`Receiver`] are thread safe. They can be moved to other
threads and can be used in a concurrent environment. Clones of [`Receiver`]
handles may be moved to separate threads and also used concurrently.

[`Sender`]: crate::sync::watch::Sender
[`Receiver`]: crate::sync::watch::Receiver
[`changed`]: crate::sync::watch::Receiver::changed
[`has_changed`]: crate::sync::watch::Receiver::has_changed
[`borrow`]: crate::sync::watch::Receiver::borrow
[`borrow_and_update`]: crate::sync::watch::Receiver::borrow_and_update
[`Receiver::changed()`]: crate::sync::watch::Receiver::changed
[`Receiver::borrow()`]: crate::sync::watch::Receiver::borrow
[`Receiver::borrow_and_update()`]:
    crate::sync::watch::Receiver::borrow_and_update
[`channel`]: crate::sync::watch::channel
[`Sender::is_closed`]: crate::sync::watch::Sender::is_closed
[`Sender::closed`]: crate::sync::watch::Sender::closed
[`Sender::subscribe()`]: crate::sync::watch::Sender::subscribe



