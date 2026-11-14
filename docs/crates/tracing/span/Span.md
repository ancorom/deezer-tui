**tracing > span > Span**

# Module: span::Span

## Contents

**Functions**

- [`current`](#current) - Returns a handle to the span [considered by the `Subscriber`] to be the
- [`enter`](#enter) - Enters this span, returning a guard that will exit the span when dropped.
- [`entered`](#entered) - Enters this span, consuming it and returning a [guard][`EnteredSpan`]
- [`follows_from`](#follows_from) - Indicates that the span with the given ID has an indirect causal
- [`in_scope`](#in_scope) - Executes the given function in the context of this span.
- [`is_disabled`](#is_disabled) - Returns `true` if this span was disabled by the subscriber and does not
- [`is_none`](#is_none) - Returns `true` if this span was constructed by [`Span::none`] and is
- [`none`](#none) - Constructs a new span that is *completely disabled*.
- [`or_current`](#or_current) - Returns this span, if it was [enabled] by the current [`Subscriber`], or

---

## tracing::span::Span::current

*Function*

Returns a handle to the span [considered by the `Subscriber`] to be the
current span.

If the subscriber indicates that it does not track the current span, or
that the thread from which this function is called is not currently
inside a span, the returned span will be disabled.

[considered by the `Subscriber`]:
    super::subscriber::Subscriber::current_span

```rust
fn current() -> Span
```



## tracing::span::Span::enter

*Function*

Enters this span, returning a guard that will exit the span when dropped.

If this span is enabled by the current subscriber, then this function will
call [`Subscriber::enter`] with the span's [`Id`], and dropping the guard
will call [`Subscriber::exit`]. If the span is disabled, this does
nothing.

# In Asynchronous Code

**Warning**: in asynchronous code that uses [async/await syntax][syntax],
`Span::enter` should be used very carefully or avoided entirely. Holding
the drop guard returned by `Span::enter` across `.await` points will
result in incorrect traces. For example,

```
# use tracing::info_span;
# async fn some_other_async_function() {}
async fn my_async_function() {
    let span = info_span!("my_async_function");

    // WARNING: This span will remain entered until this
    // guard is dropped...
    let _enter = span.enter();
    // ...but the `await` keyword may yield, causing the
    // runtime to switch to another task, while remaining in
    // this span!
    some_other_async_function().await

    // ...
}
```

The drop guard returned by `Span::enter` exits the span when it is
dropped. When an async function or async block yields at an `.await`
point, the current scope is _exited_, but values in that scope are
**not** dropped (because the async block will eventually resume
execution from that await point). This means that _another_ task will
begin executing while _remaining_ in the entered span. This results in
an incorrect trace.

Instead of using `Span::enter` in asynchronous code, prefer the
following:

* To enter a span for a synchronous section of code within an async
  block or function, prefer [`Span::in_scope`]. Since `in_scope` takes a
  synchronous closure and exits the span when the closure returns, the
  span will always be exited before the next await point. For example:
  ```
  # use tracing::info_span;
  # async fn some_other_async_function(_: ()) {}
  async fn my_async_function() {
      let span = info_span!("my_async_function");

      let some_value = span.in_scope(|| {
          // run some synchronous code inside the span...
      });

      // This is okay! The span has already been exited before we reach
      // the await point.
      some_other_async_function(some_value).await;

      // ...
  }
  ```
* For instrumenting asynchronous code, `tracing` provides the
  [`Future::instrument` combinator][instrument] for
  attaching a span to a future (async function or block). This will
  enter the span _every_ time the future is polled, and exit it whenever
  the future yields.

  `Instrument` can be used with an async block inside an async function:
  ```ignore
  # use tracing::info_span;
  use tracing::Instrument;

  # async fn some_other_async_function() {}
  async fn my_async_function() {
      let span = info_span!("my_async_function");
      async move {
         // This is correct! If we yield here, the span will be exited,
         // and re-entered when we resume.
         some_other_async_function().await;

         //more asynchronous code inside the span...

      }
        // instrument the async block with the span...
        .instrument(span)
        // ...and await it.
        .await
  }
  ```

  It can also be used to instrument calls to async functions at the
  callsite:
  ```ignore
  # use tracing::debug_span;
  use tracing::Instrument;

  # async fn some_other_async_function() {}
  async fn my_async_function() {
      let some_value = some_other_async_function()
         .instrument(debug_span!("some_other_async_function"))
         .await;

      // ...
  }
  ```

* The [`#[instrument]` attribute macro][attr] can automatically generate
  correct code when used on an async function:

  ```ignore
  # async fn some_other_async_function() {}
  #[tracing::instrument(level = "info")]
  async fn my_async_function() {

      // This is correct! If we yield here, the span will be exited,
      // and re-entered when we resume.
      some_other_async_function().await;

      // ...

  }
  ```

[syntax]: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
[`Span::in_scope`]: Span::in_scope()
[instrument]: crate::Instrument
[attr]: macro@crate::instrument

# Examples

```
# use tracing::{span, Level};
let span = span!(Level::INFO, "my_span");
let guard = span.enter();

// code here is within the span

drop(guard);

// code here is no longer within the span

```

Guards need not be explicitly dropped:

```
# use tracing::trace_span;
fn my_function() -> String {
    // enter a span for the duration of this function.
    let span = trace_span!("my_function");
    let _enter = span.enter();

    // anything happening in functions we call is still inside the span...
    my_other_function();

    // returning from the function drops the guard, exiting the span.
    return "Hello world".to_owned();
}

fn my_other_function() {
    // ...
}
```

Sub-scopes may be created to limit the duration for which the span is
entered:

```
# use tracing::{info, info_span};
let span = info_span!("my_great_span");

{
    let _enter = span.enter();

    // this event occurs inside the span.
    info!("i'm in the span!");

    // exiting the scope drops the guard, exiting the span.
}

// this event is not inside the span.
info!("i'm outside the span!")
```

[`Subscriber::enter`]: super::subscriber::Subscriber::enter()
[`Subscriber::exit`]: super::subscriber::Subscriber::exit()
[`Id`]: super::Id

```rust
fn enter(self: &Self) -> Entered
```



## tracing::span::Span::entered

*Function*

Enters this span, consuming it and returning a [guard][`EnteredSpan`]
that will exit the span when dropped.

<pre class="compile_fail" style="white-space:normal;font:inherit;">
    <strong>Warning</strong>: In asynchronous code that uses async/await syntax,
    <code>Span::entered</code> may produce incorrect traces if the returned drop
    guard is held across an await point. See <a href="#in-asynchronous-code">the
    <code>Span::enter</code> documentation</a> for details.
</pre>


If this span is enabled by the current subscriber, then this function will
call [`Subscriber::enter`] with the span's [`Id`], and dropping the guard
will call [`Subscriber::exit`]. If the span is disabled, this does
nothing.

This is similar to the [`Span::enter`] method, except that it moves the
span by value into the returned guard, rather than borrowing it.
Therefore, this method can be used to create and enter a span in a
single expression, without requiring a `let`-binding. For example:

```
# use tracing::info_span;
let _span = info_span!("something_interesting").entered();
```
rather than:
```
# use tracing::info_span;
let span = info_span!("something_interesting");
let _e = span.enter();
```

Furthermore, `entered` may be used when the span must be stored in some
other struct or be passed to a function while remaining entered.

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: The returned <a href="../struct.EnteredSpan.html">
    <code>EnteredSpan</code></a> guard does not implement <code>Send</code>.
    Dropping the guard will exit <em>this</em> span, and if the guard is sent
    to another thread and dropped there, that thread may never have entered
    this span. Thus, <code>EnteredSpan</code>s should not be sent between threads.
</pre>

[syntax]: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html

# Examples

The returned guard can be [explicitly exited][EnteredSpan::exit],
returning the un-entered span:

```
# use tracing::{Level, span};
let span = span!(Level::INFO, "doing_something").entered();

// code here is within the span

// explicitly exit the span, returning it
let span = span.exit();

// code here is no longer within the span

// enter the span again
let span = span.entered();

// now we are inside the span once again
```

Guards need not be explicitly dropped:

```
# use tracing::trace_span;
fn my_function() -> String {
    // enter a span for the duration of this function.
    let span = trace_span!("my_function").entered();

    // anything happening in functions we call is still inside the span...
    my_other_function();

    // returning from the function drops the guard, exiting the span.
    return "Hello world".to_owned();
}

fn my_other_function() {
    // ...
}
```

Since the [`EnteredSpan`] guard can dereference to the [`Span`] itself,
the span may still be accessed while entered. For example:

```rust
# use tracing::info_span;
use tracing::field;

// create the span with an empty field, and enter it.
let span = info_span!("my_span", some_field = field::Empty).entered();

// we can still record a value for the field while the span is entered.
span.record("some_field", &"hello world!");
```

[`Subscriber::enter`]: super::subscriber::Subscriber::enter()
[`Subscriber::exit`]: super::subscriber::Subscriber::exit()
[`Id`]: super::Id

```rust
fn entered(self: Self) -> EnteredSpan
```



## tracing::span::Span::follows_from

*Function*

Indicates that the span with the given ID has an indirect causal
relationship with this span.

This relationship differs somewhat from the parent-child relationship: a
span may have any number of prior spans, rather than a single one; and
spans are not considered to be executing _inside_ of the spans they
follow from. This means that a span may close even if subsequent spans
that follow from it are still open, and time spent inside of a
subsequent span should not be included in the time its precedents were
executing. This is used to model causal relationships such as when a
single future spawns several related background tasks, et cetera.

If this span is disabled, or the resulting follows-from relationship
would be invalid, this function will do nothing.

# Examples

Setting a `follows_from` relationship with a `Span`:
```
# use tracing::{span, Id, Level, Span};
let span1 = span!(Level::INFO, "span_1");
let span2 = span!(Level::DEBUG, "span_2");
span2.follows_from(span1);
```

Setting a `follows_from` relationship with the current span:
```
# use tracing::{span, Id, Level, Span};
let span = span!(Level::INFO, "hello!");
span.follows_from(Span::current());
```

Setting a `follows_from` relationship with a `Span` reference:
```
# use tracing::{span, Id, Level, Span};
let span = span!(Level::INFO, "hello!");
let curr = Span::current();
span.follows_from(&curr);
```

Setting a `follows_from` relationship with an `Id`:
```
# use tracing::{span, Id, Level, Span};
let span = span!(Level::INFO, "hello!");
let id = span.id();
span.follows_from(id);
```

```rust
fn follows_from<impl Into<Option<Id>>>(self: &Self, from: impl Trait) -> &Self
```



## tracing::span::Span::in_scope

*Function*

Executes the given function in the context of this span.

If this span is enabled, then this function enters the span, invokes `f`
and then exits the span. If the span is disabled, `f` will still be
invoked, but in the context of the currently-executing span (if there is
one).

Returns the result of evaluating `f`.

# Examples

```
# use tracing::{trace, span, Level};
let my_span = span!(Level::TRACE, "my_span");

my_span.in_scope(|| {
    // this event occurs within the span.
    trace!("i'm in the span!");
});

// this event occurs outside the span.
trace!("i'm not in the span!");
```

Calling a function and returning the result:
```
# use tracing::{info_span, Level};
fn hello_world() -> String {
    "Hello world!".to_owned()
}

let span = info_span!("hello_world");
// the span will be entered for the duration of the call to
// `hello_world`.
let a_string = span.in_scope(hello_world);


```rust
fn in_scope<F, T>(self: &Self, f: F) -> T
```



## tracing::span::Span::is_disabled

*Function*

Returns `true` if this span was disabled by the subscriber and does not
exist.

See also [`is_none`].

[`is_none`]: Span::is_none()

```rust
fn is_disabled(self: &Self) -> bool
```



## tracing::span::Span::is_none

*Function*

Returns `true` if this span was constructed by [`Span::none`] and is
empty.

If `is_none` returns `true` for a given span, then [`is_disabled`] will
also return `true`. However, when a span is disabled by the subscriber
rather than constructed by `Span::none`, this method will return
`false`, while `is_disabled` will return `true`.

[`Span::none`]: Span::none()
[`is_disabled`]: Span::is_disabled()

```rust
fn is_none(self: &Self) -> bool
```



## tracing::span::Span::none

*Function*

Constructs a new span that is *completely disabled*.

This can be used rather than `Option<Span>` to represent cases where a
span is not present.

Entering, exiting, and recording values on this span will do nothing.

```rust
fn none() -> Span
```



## tracing::span::Span::or_current

*Function*

Returns this span, if it was [enabled] by the current [`Subscriber`], or
the [current span] (whose lexical distance may be further than expected),
 if this span [is disabled].

This method can be useful when propagating spans to spawned threads or
[async tasks]. Consider the following:

```
let _parent_span = tracing::info_span!("parent").entered();

// ...

let child_span = tracing::debug_span!("child");

std::thread::spawn(move || {
    let _entered = child_span.entered();

    tracing::info!("spawned a thread!");

    // ...
});
```

If the current [`Subscriber`] enables the [`DEBUG`] level, then both
the "parent" and "child" spans will be enabled. Thus, when the "spawned
a thread!" event occurs, it will be inside of the "child" span. Because
"parent" is the parent of "child", the event will _also_ be inside of
"parent".

However, if the [`Subscriber`] only enables the [`INFO`] level, the "child"
span will be disabled. When the thread is spawned, the
`child_span.entered()` call will do nothing, since "child" is not
enabled. In this case, the "spawned a thread!" event occurs outside of
*any* span, since the "child" span was responsible for propagating its
parent to the spawned thread.

If this is not the desired behavior, `Span::or_current` can be used to
ensure that the "parent" span is propagated in both cases, either as a
parent of "child" _or_ directly. For example:

```
let _parent_span = tracing::info_span!("parent").entered();

// ...

// If DEBUG is enabled, then "child" will be enabled, and `or_current`
// returns "child". Otherwise, if DEBUG is not enabled, "child" will be
// disabled, and `or_current` returns "parent".
let child_span = tracing::debug_span!("child").or_current();

std::thread::spawn(move || {
    let _entered = child_span.entered();

    tracing::info!("spawned a thread!");

    // ...
});
```

When spawning [asynchronous tasks][async tasks], `Span::or_current` can
be used similarly, in combination with [`instrument`]:

```
use tracing::Instrument;
# // lol
# mod tokio {
#     pub(super) fn spawn(_: impl std::future::Future) {}
# }

let _parent_span = tracing::info_span!("parent").entered();

// ...

let child_span = tracing::debug_span!("child");

tokio::spawn(
    async {
        tracing::info!("spawned a task!");

        // ...

    }.instrument(child_span.or_current())
);
```

In general, `or_current` should be preferred over nesting an
[`instrument`]  call inside of an [`in_current_span`] call, as using
`or_current` will be more efficient.

```
use tracing::Instrument;
# // lol
# mod tokio {
#     pub(super) fn spawn(_: impl std::future::Future) {}
# }
async fn my_async_fn() {
    // ...
}

let _parent_span = tracing::info_span!("parent").entered();

// Do this:
tokio::spawn(
    my_async_fn().instrument(tracing::debug_span!("child").or_current())
);

// ...rather than this:
tokio::spawn(
    my_async_fn()
        .instrument(tracing::debug_span!("child"))
        .in_current_span()
);
```

[enabled]: crate::Subscriber::enabled
[`Subscriber`]: crate::Subscriber
[current span]: Span::current
[is disabled]: Span::is_disabled
[`INFO`]: crate::Level::INFO
[`DEBUG`]: crate::Level::DEBUG
[async tasks]: std::task
[`instrument`]: crate::instrument::Instrument::instrument
[`in_current_span`]: crate::instrument::Instrument::in_current_span

```rust
fn or_current(self: Self) -> Self
```



