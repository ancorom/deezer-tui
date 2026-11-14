**tracing**

# Module: tracing

## Contents

**Modules**

- [`dispatcher`](#dispatcher) - Dispatches trace events to [`Subscriber`]s.
- [`field`](#field) - `Span` and `Event` key-value data.
- [`instrument`](#instrument) - Attach a span to a `std::future::Future`.
- [`level_filters`](#level_filters) - Trace verbosity level filtering.
- [`span`](#span) -  Spans represent periods of time in which a program was executing in a
- [`subscriber`](#subscriber) - Collects and records trace data.

**Macros**

- [`debug`](#debug) - Constructs an event at the debug level.
- [`debug_span`](#debug_span) - Constructs a span at the debug level.
- [`enabled`](#enabled) - Checks whether a span or event is [enabled] based on the provided [metadata].
- [`error`](#error) - Constructs an event at the error level.
- [`error_span`](#error_span) - Constructs a span at the error level.
- [`event`](#event) - Constructs a new `Event`.
- [`event_enabled`](#event_enabled) - Tests whether an event with the specified level and target would be enabled.
- [`info`](#info) - Constructs an event at the info level.
- [`info_span`](#info_span) - Constructs a span at the info level.
- [`record_all`](#record_all) - Records multiple values on a span in a single call. As with recording
- [`span`](#span) - Constructs a new span.
- [`span_enabled`](#span_enabled) - Tests whether a span with the specified level and target would be enabled.
- [`trace`](#trace) - Constructs an event at the trace level.
- [`trace_span`](#trace_span) - Constructs a span at the trace level.
- [`warn`](#warn) - Constructs an event at the warn level.
- [`warn_span`](#warn_span) - Constructs a span at the warn level.

---

## tracing::debug

*Declarative Macro*

Constructs an event at the debug level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::debug;
# fn main() {
# #[derive(Debug)] struct Position { x: f32, y: f32 }

let pos = Position { x: 3.234, y: -1.223 };

debug!(?pos.x, ?pos.y);
debug!(target: "app_events", position = ?pos, "New position");
debug!(name: "completed", position = ?pos);
# }
```

```rust
macro_rules! debug {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



## tracing::debug_span

*Declarative Macro*

Constructs a span at the debug level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{debug_span, span, Level};
# fn main() {
debug_span!("my_span");
// is equivalent to:
span!(Level::DEBUG, "my_span");
# }
```

```rust
# use tracing::debug_span;
# fn main() {
let span = debug_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
macro_rules! debug_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
}
```



## Module: dispatcher

Dispatches trace events to [`Subscriber`]s.

The _dispatcher_ is the component of the tracing system which is responsible
for forwarding trace data from the instrumentation points that generate it
to the subscriber that collects it.

# Using the Trace Dispatcher

Every thread in a program using `tracing` has a _default subscriber_. When
events occur, or spans are created, they are dispatched to the thread's
current subscriber.

## Setting the Default Subscriber

By default, the current subscriber is an empty implementation that does
nothing. To use a subscriber implementation, it must be set as the default.
There are two methods for doing so: [`with_default`] and
[`set_global_default`]. `with_default` sets the default subscriber for the
duration of a scope, while `set_global_default` sets a default subscriber
for the entire process.

To use either of these functions, we must first wrap our subscriber in a
[`Dispatch`], a cloneable, type-erased reference to a subscriber. For
example:
```rust
# pub struct FooSubscriber;
# use tracing_core::{
#   dispatcher, Event, Metadata,
#   span::{Attributes, Id, Record}
# };
# impl tracing_core::Subscriber for FooSubscriber {
#   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
#   fn record(&self, _: &Id, _: &Record) {}
#   fn event(&self, _: &Event) {}
#   fn record_follows_from(&self, _: &Id, _: &Id) {}
#   fn enabled(&self, _: &Metadata) -> bool { false }
#   fn enter(&self, _: &Id) {}
#   fn exit(&self, _: &Id) {}
# }
# impl FooSubscriber { fn new() -> Self { FooSubscriber } }
use dispatcher::Dispatch;

let my_subscriber = FooSubscriber::new();
let my_dispatch = Dispatch::new(my_subscriber);
```
Then, we can use [`with_default`] to set our `Dispatch` as the default for
the duration of a block:
```rust
# pub struct FooSubscriber;
# use tracing_core::{
#   dispatcher, Event, Metadata,
#   span::{Attributes, Id, Record}
# };
# impl tracing_core::Subscriber for FooSubscriber {
#   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
#   fn record(&self, _: &Id, _: &Record) {}
#   fn event(&self, _: &Event) {}
#   fn record_follows_from(&self, _: &Id, _: &Id) {}
#   fn enabled(&self, _: &Metadata) -> bool { false }
#   fn enter(&self, _: &Id) {}
#   fn exit(&self, _: &Id) {}
# }
# impl FooSubscriber { fn new() -> Self { FooSubscriber } }
# let my_subscriber = FooSubscriber::new();
# let my_dispatch = dispatcher::Dispatch::new(my_subscriber);
// no default subscriber

# #[cfg(feature = "std")]
dispatcher::with_default(&my_dispatch, || {
    // my_subscriber is the default
});

// no default subscriber again
```
It's important to note that `with_default` will not propagate the current
thread's default subscriber to any threads spawned within the `with_default`
block. To propagate the default subscriber to new threads, either use
`with_default` from the new thread, or use `set_global_default`.

As an alternative to `with_default`, we can use [`set_global_default`] to
set a `Dispatch` as the default for all threads, for the lifetime of the
program. For example:
```rust
# pub struct FooSubscriber;
# use tracing_core::{
#   dispatcher, Event, Metadata,
#   span::{Attributes, Id, Record}
# };
# impl tracing_core::Subscriber for FooSubscriber {
#   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
#   fn record(&self, _: &Id, _: &Record) {}
#   fn event(&self, _: &Event) {}
#   fn record_follows_from(&self, _: &Id, _: &Id) {}
#   fn enabled(&self, _: &Metadata) -> bool { false }
#   fn enter(&self, _: &Id) {}
#   fn exit(&self, _: &Id) {}
# }
# impl FooSubscriber { fn new() -> Self { FooSubscriber } }
# let my_subscriber = FooSubscriber::new();
# let my_dispatch = dispatcher::Dispatch::new(my_subscriber);
// no default subscriber

dispatcher::set_global_default(my_dispatch)
    // `set_global_default` will return an error if the global default
    // subscriber has already been set.
    .expect("global default was already set!");

// `my_subscriber` is now the default
```

<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: The thread-local scoped dispatcher (<code>with_default</code>)
requires the Rust standard library. <code>no_std</code> users should
use <a href="fn.set_global_default.html"><code>set_global_default</code></a>
instead.
</pre>

## Accessing the Default Subscriber

A thread's current default subscriber can be accessed using the
[`get_default`] function, which executes a closure with a reference to the
currently default `Dispatch`. This is used primarily by `tracing`
instrumentation.

[`Subscriber`]: crate::Subscriber



## tracing::enabled

*Declarative Macro*

Checks whether a span or event is [enabled] based on the provided [metadata].

[enabled]: crate::Subscriber::enabled
[metadata]: crate::Metadata

This macro is a specialized tool: it is intended to be used prior
to an expensive computation required *just* for that event, but
*cannot* be done as part of an argument to that event, such as
when multiple events are emitted (e.g., iterating over a collection
and emitting an event for each item).

# Usage

[Subscribers] can make filtering decisions based all the data included in a
span or event's [`Metadata`]. This means that it is possible for `enabled!`
to return a _false positive_ (indicating that something would be enabled
when it actually would not be) or a _false negative_ (indicating that
something would be disabled when it would actually be enabled).

[Subscribers]: crate::subscriber::Subscriber
[`Metadata`]: crate::metadata::Metadata

This occurs when a subscriber is using a _more specific_ filter than the
metadata provided to the `enabled!` macro. Some situations that can result
in false positives or false negatives include:

- If a subscriber is using a filter which may enable a span or event based
  on field names, but `enabled!` is invoked without listing field names,
  `enabled!` may return a false negative if a specific field name would
  cause the subscriber to enable something that would otherwise be disabled.
- If a subscriber is using a filter which enables or disables specific events by
  file path and line number,  a particular event may be enabled/disabled
  even if an `enabled!` invocation with the same level, target, and fields
  indicated otherwise.
- The subscriber can choose to enable _only_ spans or _only_ events, which `enabled`
  will not reflect.

`enabled!()` requires a [level](crate::Level) argument, an optional `target:`
argument, and an optional set of field names. If the fields are not provided,
they are considered to be unknown. `enabled!` attempts to match the
syntax of `event!()` as closely as possible, which can be seen in the
examples below.

# Examples

If the current subscriber is interested in recording `DEBUG`-level spans and
events in the current file and module path, this will evaluate to true:
```rust
use tracing::{enabled, Level};

if enabled!(Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events
in the current file and module path, with the target "my_crate", and at the
level  `DEBUG`, this will evaluate to true:
```rust
# use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
```

If the current subscriber is interested in recording spans and events
in the current file and module path, with the target "my_crate", at
the level `DEBUG`, and with a field named "hello", this will evaluate
to true:

```rust
# use tracing::{enabled, Level};
if enabled!(target: "my_crate", Level::DEBUG, hello) {
    // some expensive work...
}
```

# Alternatives

`enabled!` queries subscribers with [`Metadata`] where
[`is_event`] and [`is_span`] both return `false`. Alternatively,
use [`event_enabled!`] or [`span_enabled!`] to ensure one of these
returns true.


[`Metadata`]: crate::Metadata
[`is_event`]: crate::Metadata::is_event
[`is_span`]: crate::Metadata::is_span
[`enabled!`]: crate::enabled
[`span_enabled!`]: crate::span_enabled

```rust
macro_rules! enabled {
    (kind: $kind:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (kind: $kind:expr, target: $target:expr, $lvl:expr ) => { ... };
    (target: $target:expr, $lvl:expr ) => { ... };
    (kind: $kind:expr, target: $target:expr, $lvl:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $lvl:expr, $($field:tt)*) => { ... };
    (kind: $kind:expr, $lvl:expr, $($field:tt)*) => { ... };
    (kind: $kind:expr, $lvl:expr) => { ... };
    ($lvl:expr) => { ... };
    ($lvl:expr, $($field:tt)*) => { ... };
}
```



## tracing::error

*Declarative Macro*

Constructs an event at the error level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::error;
# fn main() {

let (err_info, port) = ("No connection", 22);

error!(port, error = %err_info);
error!(target: "app_events", "App Error: {}", err_info);
error!({ info = err_info }, "error on port: {}", port);
error!(name: "invalid_input", "Invalid input: {}", err_info);
# }
```

```rust
macro_rules! error {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



## tracing::error_span

*Declarative Macro*

Constructs a span at the error level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{span, error_span, Level};
# fn main() {
error_span!("my_span");
// is equivalent to:
span!(Level::ERROR, "my_span");
# }
```

```rust
# use tracing::error_span;
# fn main() {
let span = error_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
macro_rules! error_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
}
```



## tracing::event

*Declarative Macro*

Constructs a new `Event`.

The event macro is invoked with a `Level` and up to 32 key-value fields.
Optionally, a format string and arguments may follow the fields; this will
be used to construct an implicit field named "message".

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros

# Examples

```rust
use tracing::{event, Level};

# fn main() {
let data = (42, "forty-two");
let private_data = "private";
let error = "a bad error";

event!(Level::ERROR, %error, "Received error");
event!(
    target: "app_events",
    Level::WARN,
    private_data,
    ?data,
    "App warning: {}",
    error
);
event!(name: "answer", Level::INFO, the_answer = data.0);
event!(Level::INFO, the_answer = data.0);
# }
```


```rust
macro_rules! event {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $lvl:expr, $($arg:tt)+) => { ... };
    (name: $name:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (name: $name:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (name: $name:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (name: $name:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, $lvl:expr, { $($fields:tt)* } ) => { ... };
    (target: $target:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (target: $target:expr, $lvl:expr, $($k:ident).+ = $($fields:tt)* ) => { ... };
    (target: $target:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $lvl:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, $($arg:tt)+ ) => { ... };
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    ( $lvl:expr, { $($fields:tt)* }, $($arg:tt)+ ) => { ... };
    ($lvl:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    ($lvl:expr, $($k:ident).+, $($field:tt)*) => { ... };
    ($lvl:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    ($lvl:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    ($lvl:expr, ?$($k:ident).+) => { ... };
    ($lvl:expr, %$($k:ident).+) => { ... };
    ($lvl:expr, $($k:ident).+) => { ... };
    ( $lvl:expr, $($arg:tt)+ ) => { ... };
}
```



## tracing::event_enabled

*Declarative Macro*

Tests whether an event with the specified level and target would be enabled.

This is similar to [`enabled!`], but queries the current subscriber specifically for
an event, whereas [`enabled!`] queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also [`span_enabled!`].

# Examples

```rust
# use tracing::{event_enabled, Level};
if event_enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
// simpler
if event_enabled!(Level::DEBUG) {
    // some expensive work...
}
// with fields
if event_enabled!(Level::DEBUG, foo_field) {
    // some expensive work...
}
```

[`enabled!`]: crate::enabled
[`span_enabled!`]: crate::span_enabled

```rust
macro_rules! event_enabled {
    ($($rest:tt)*) => { ... };
}
```



## Module: field

`Span` and `Event` key-value data.

Spans and events may be annotated with key-value data, referred to as _fields_.
These fields consist of a mapping from a key (corresponding to
a `&str` but represented internally as an array index) to a [`Value`].

# `Value`s and `Subscriber`s

`Subscriber`s consume `Value`s as fields attached to [span]s or [`Event`]s.
The set of field keys on a given span or event is defined on its [`Metadata`].
When a span is created, it provides [`Attributes`] to the `Subscriber`'s
[`new_span`] method, containing any fields whose values were provided when
the span was created; and may call the `Subscriber`'s [`record`] method
with additional [`Record`]s if values are added for more of its fields.
Similarly, the [`Event`] type passed to the subscriber's [`event`] method
will contain any fields attached to each event.

`tracing` represents values as either one of a set of Rust primitives
(`i64`, `u64`, `f64`, `bool`, and `&str`) or using a `fmt::Display` or
`fmt::Debug` implementation. `Subscriber`s are provided these primitive
value types as `dyn Value` trait objects.

These trait objects can be formatted using `fmt::Debug`, but may also be
recorded as typed data by calling the [`Value::record`] method on these
trait objects with a _visitor_ implementing the [`Visit`] trait. This trait
represents the behavior used to record values of various types. For example,
an implementation of `Visit` might record integers by incrementing counters
for their field names rather than printing them.


# Using `valuable`

`tracing`'s [`Value`] trait is intentionally minimalist: it supports only a small
number of Rust primitives as typed values, and only permits recording
user-defined types with their [`fmt::Debug`] or [`fmt::Display`]
implementations. However, there are some cases where it may be useful to record
nested values (such as arrays, `Vec`s, or `HashMap`s containing values), or
user-defined `struct` and `enum` types without having to format them as
unstructured text.

To address `Value`'s limitations, `tracing` offers experimental support for
the [`valuable`] crate, which provides object-safe inspection of structured
values. User-defined types can implement the [`valuable::Valuable`] trait,
and be recorded as a `tracing` field by calling their [`as_value`] method.
If the [`Subscriber`] also supports the `valuable` crate, it can
then visit those types fields as structured values using `valuable`.

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: <code>valuable</code> support is an
    <a href = "../index.html#unstable-features">unstable feature</a>. See
    the documentation on unstable features for details on how to enable it.
</pre>

For example:
```ignore
// Derive `Valuable` for our types:
use valuable::Valuable;

#[derive(Clone, Debug, Valuable)]
struct User {
    name: String,
    age: u32,
    address: Address,
}

#[derive(Clone, Debug, Valuable)]
struct Address {
    country: String,
    city: String,
    street: String,
}

let user = User {
    name: "Arwen Undomiel".to_string(),
    age: 3000,
    address: Address {
        country: "Middle Earth".to_string(),
        city: "Rivendell".to_string(),
        street: "leafy lane".to_string(),
    },
};

// Recording `user` as a `valuable::Value` will allow the `tracing` subscriber
// to traverse its fields as a nested, typed structure:
tracing::info!(current_user = user.as_value());
```

Alternatively, the [`valuable()`] function may be used to convert a type
implementing [`Valuable`] into a `tracing` field value.

When the `valuable` feature is enabled, the [`Visit`] trait will include an
optional [`record_value`] method. `Visit` implementations that wish to
record `valuable` values can implement this method with custom behavior.
If a visitor does not implement `record_value`, the [`valuable::Value`] will
be forwarded to the visitor's [`record_debug`] method.

[`fmt::Debug`]: std::fmt::Debug
[`fmt::Display`]: std::fmt::Debug
[`valuable`]: https://crates.io/crates/valuable
[`valuable::Valuable`]: https://docs.rs/valuable/latest/valuable/trait.Valuable.html
[`as_value`]: https://docs.rs/valuable/latest/valuable/trait.Valuable.html#tymethod.as_value
[`valuable::Value`]: https://docs.rs/valuable/latest/valuable/enum.Value.html
[`Subscriber`]: crate::Subscriber
[`record_value`]: Visit::record_value
[`record_debug`]: Visit::record_debug
[span]: mod@crate::span
[`Event`]: crate::event::Event
[`Metadata`]: crate::Metadata
[`Attributes`]: crate::span::Attributes
[`Record`]: crate::span::Record
[`new_span`]: crate::Subscriber::new_span
[`record`]: crate::Subscriber::record
[`event`]: crate::Subscriber::event



## tracing::info

*Declarative Macro*

Constructs an event at the info level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::info;
# // this is so the test will still work in no-std mode
# #[derive(Debug)]
# pub struct Ipv4Addr;
# impl Ipv4Addr { fn new(o1: u8, o2: u8, o3: u8, o4: u8) -> Self { Self } }
# fn main() {
# struct Connection { port: u32, speed: f32 }
use tracing::field;

let addr = Ipv4Addr::new(127, 0, 0, 1);
let conn = Connection { port: 40, speed: 3.20 };

info!(conn.port, "connected to {:?}", addr);
info!(
    target: "connection_events",
    ip = ?addr,
    conn.port,
    ?conn.speed,
);
info!(name: "completed", "completed connection to {:?}", addr);
# }
```

```rust
macro_rules! info {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



## tracing::info_span

*Declarative Macro*

Constructs a span at the info level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{span, info_span, Level};
# fn main() {
info_span!("my_span");
// is equivalent to:
span!(Level::INFO, "my_span");
# }
```

```rust
# use tracing::info_span;
# fn main() {
let span = info_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
macro_rules! info_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
}
```



## Module: instrument

Attach a span to a `std::future::Future`.



## Module: level_filters

Trace verbosity level filtering.

# Compile time filters

Trace verbosity levels can be statically disabled at compile time via Cargo
features, similar to the [`log` crate]. Trace instrumentation at disabled
levels will be skipped and will not even be present in the resulting binary
unless the verbosity level is specified dynamically. This level is
configured separately for release and debug builds. The features are:

* `max_level_off`
* `max_level_error`
* `max_level_warn`
* `max_level_info`
* `max_level_debug`
* `max_level_trace`
* `release_max_level_off`
* `release_max_level_error`
* `release_max_level_warn`
* `release_max_level_info`
* `release_max_level_debug`
* `release_max_level_trace`

These features control the value of the `STATIC_MAX_LEVEL` constant. The
instrumentation macros macros check this value before recording an event or
constructing a span. By default, no levels are disabled.

For example, a crate can disable trace level instrumentation in debug builds
and trace, debug, and info level instrumentation in release builds with the
following configuration:

```toml
[dependencies]
tracing = { version = "0.1", features = ["max_level_debug", "release_max_level_warn"] }
```
## Notes

Please note that `tracing`'s static max level features do *not* control the
[`log`] records that may be emitted when [`tracing`'s "log" feature flag][f] is
enabled. This is to allow `tracing` to be disabled entirely at compile time
while still emitting `log` records --- such as when a library using
`tracing` is used by an application using `log` that doesn't want to
generate any `tracing`-related code, but does want to collect `log` records.

This means that if the "log" feature is in use, some code may be generated
for `log` records emitted by disabled `tracing` events. If this is not
desirable, `log` records may be disabled separately using [`log`'s static
max level features][`log` crate].

[`log`]: https://docs.rs/log/
[`log` crate]: https://docs.rs/log/latest/log/#compile-time-filters
[f]: https://docs.rs/tracing/latest/tracing/#emitting-log-records



## tracing::record_all

*Declarative Macro*

Records multiple values on a span in a single call. As with recording
individual values, all fields must be declared when the span is created.

This macro supports two optional sigils:
- `%` uses the Display implementation.
- `?` uses the Debug implementation.

For more details, see the [top-level documentation][lib].

[lib]: tracing/#recording-fields

# Examples

```
# use tracing::{field, info_span, record_all};
let span = info_span!("my span", field1 = field::Empty, field2 = field::Empty, field3 = field::Empty).entered();
record_all!(span, field1 = ?"1", field2 = %"2", field3 = 3);
```

```rust
macro_rules! record_all {
    ($span:expr, $($fields:tt)*) => { ... };
}
```



## Module: span

 Spans represent periods of time in which a program was executing in a
 particular context.

 A span consists of [fields], user-defined key-value pairs of arbitrary data
 that describe the context the span represents, and a set of fixed attributes
 that describe all `tracing` spans and events. Attributes describing spans
 include:

 - An [`Id`] assigned by the subscriber that uniquely identifies it in relation
   to other spans.
 - The span's [parent] in the trace tree.
 - [Metadata] that describes static characteristics of all spans
   originating from that callsite, such as its name, source code location,
   [verbosity level], and the names of its fields.

 # Creating Spans

 Spans are created using the [`span!`] macro. This macro is invoked with the
 following arguments, in order:

 - The [`target`] and/or [`parent`][parent] attributes, if the user wishes to
   override their default values.
 - The span's [verbosity level]
 - A string literal providing the span's name.
 - Finally, zero or more arbitrary key/value fields.

 [`target`]: super::Metadata::target

 For example:
 ```rust
 use tracing::{span, Level};

 /// Construct a new span at the `INFO` level named "my_span", with a single
 /// field named answer , with the value `42`.
 let my_span = span!(Level::INFO, "my_span", answer = 42);
 ```

 The documentation for the [`span!`] macro provides additional examples of
 the various options that exist when creating spans.

 The [`trace_span!`], [`debug_span!`], [`info_span!`], [`warn_span!`], and
 [`error_span!`] exist as shorthand for constructing spans at various
 verbosity levels.

 ## Recording Span Creation

 The [`Attributes`] type contains data associated with a span, and is
 provided to the [`Subscriber`] when a new span is created. It contains
 the span's metadata, the ID of [the span's parent][parent] if one was
 explicitly set, and any fields whose values were recorded when the span was
 constructed. The subscriber, which is responsible for recording `tracing`
 data, can then store or record these values.

 # The Span Lifecycle

 ## Entering a Span

 A thread of execution is said to _enter_ a span when it begins executing,
 and _exit_ the span when it switches to another context. Spans may be
 entered through the [`enter`], [`entered`], and [`in_scope`] methods.

 The [`enter`] method enters a span, returning a [guard] that exits the span
 when dropped
 ```
 # use tracing::{span, Level};
 let my_var: u64 = 5;
 let my_span = span!(Level::TRACE, "my_span", my_var);

 // `my_span` exists but has not been entered.

 // Enter `my_span`...
 let _enter = my_span.enter();

 // Perform some work inside of the context of `my_span`...
 // Dropping the `_enter` guard will exit the span.
```

 <div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">
     <strong>Warning</strong>: In asynchronous code that uses async/await syntax,
     <code>Span::enter</code> may produce incorrect traces if the returned drop
     guard is held across an await point. See
     <a href="struct.Span.html#in-asynchronous-code">the method documentation</a>
     for details.
 </pre></div>

 The [`entered`] method is analogous to [`enter`], but moves the span into
 the returned guard, rather than borrowing it. This allows creating and
 entering a span in a single expression:

 ```
 # use tracing::{span, Level};
 // Create a span and enter it, returning a guard:
 let span = span!(Level::INFO, "my_span").entered();

 // We are now inside the span! Like `enter()`, the guard returned by
 // `entered()` will exit the span when it is dropped...

 // ...but, it can also be exited explicitly, returning the `Span`
 // struct:
 let span = span.exit();
 ```

 Finally, [`in_scope`] takes a closure or function pointer and executes it
 inside the span:

 ```
 # use tracing::{span, Level};
 let my_var: u64 = 5;
 let my_span = span!(Level::TRACE, "my_span", my_var = &my_var);

 my_span.in_scope(|| {
     // perform some work in the context of `my_span`...
 });

 // Perform some work outside of the context of `my_span`...

 my_span.in_scope(|| {
     // Perform some more work in the context of `my_span`.
 });
 ```

 <pre class="ignore" style="white-space:normal;font:inherit;">
     <strong>Note</strong>: Since entering a span takes <code>&self</code>, and
     <code>Span</code>s are <code>Clone</code>, <code>Send</code>, and
     <code>Sync</code>, it is entirely valid for multiple threads to enter the
     same span concurrently.
 </pre>

 ## Span Relationships

 Spans form a tree structure — unless it is a root span, all spans have a
 _parent_, and may have one or more _children_. When a new span is created,
 the current span becomes the new span's parent. The total execution time of
 a span consists of the time spent in that span and in the entire subtree
 represented by its children. Thus, a parent span always lasts for at least
 as long as the longest-executing span in its subtree.

 ```
 # use tracing::{Level, span};
 // this span is considered the "root" of a new trace tree:
 span!(Level::INFO, "root").in_scope(|| {
     // since we are now inside "root", this span is considered a child
     // of "root":
     span!(Level::DEBUG, "outer_child").in_scope(|| {
         // this span is a child of "outer_child", which is in turn a
         // child of "root":
         span!(Level::TRACE, "inner_child").in_scope(|| {
             // and so on...
         });
     });
     // another span created here would also be a child of "root".
 });
```

 In addition, the parent of a span may be explicitly specified in
 the `span!` macro. For example:

 ```rust
 # use tracing::{Level, span};
 // Create, but do not enter, a span called "foo".
 let foo = span!(Level::INFO, "foo");

 // Create and enter a span called "bar".
 let bar = span!(Level::INFO, "bar");
 let _enter = bar.enter();

 // Although we have currently entered "bar", "baz"'s parent span
 // will be "foo".
 let baz = span!(parent: &foo, Level::INFO, "baz");
 ```

 A child span should typically be considered _part_ of its parent. For
 example, if a subscriber is recording the length of time spent in various
 spans, it should generally include the time spent in a span's children as
 part of that span's duration.

 In addition to having zero or one parent, a span may also _follow from_ any
 number of other spans. This indicates a causal relationship between the span
 and the spans that it follows from, but a follower is *not* typically
 considered part of the duration of the span it follows. Unlike the parent, a
 span may record that it follows from another span after it is created, using
 the [`follows_from`] method.

 As an example, consider a listener task in a server. As the listener accepts
 incoming connections, it spawns new tasks that handle those connections. We
 might want to have a span representing the listener, and instrument each
 spawned handler task with its own span. We would want our instrumentation to
 record that the handler tasks were spawned as a result of the listener task.
 However, we might not consider the handler tasks to be _part_ of the time
 spent in the listener task, so we would not consider those spans children of
 the listener span. Instead, we would record that the handler tasks follow
 from the listener, recording the causal relationship but treating the spans
 as separate durations.

 ## Closing Spans

 Execution may enter and exit a span multiple times before that span is
 _closed_. Consider, for example, a future which has an associated
 span and enters that span every time it is polled:
 ```rust
 # use std::future::Future;
 # use std::task::{Context, Poll};
 # use std::pin::Pin;
 struct MyFuture {
    // data
    span: tracing::Span,
 }

 impl Future for MyFuture {
     type Output = ();

     fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
         let _enter = self.span.enter();
         // Do actual future work...
 # Poll::Ready(())
     }
 }
 ```

 If this future was spawned on an executor, it might yield one or more times
 before `poll` returns [`Poll::Ready`]. If the future were to yield, then
 the executor would move on to poll the next future, which may _also_ enter
 an associated span or series of spans. Therefore, it is valid for a span to
 be entered repeatedly before it completes. Only the time when that span or
 one of its children was the current span is considered to be time spent in
 that span. A span which is not executing and has not yet been closed is said
 to be _idle_.

 Because spans may be entered and exited multiple times before they close,
 [`Subscriber`]s have separate trait methods which are called to notify them
 of span exits and when span handles are dropped. When execution exits a
 span, [`exit`] will always be called with that span's ID to notify the
 subscriber that the span has been exited. When span handles are dropped, the
 [`drop_span`] method is called with that span's ID. The subscriber may use
 this to determine whether or not the span will be entered again.

 If there is only a single handle with the capacity to exit a span, dropping
 that handle "closes" the span, since the capacity to enter it no longer
 exists. For example:
 ```
 # use tracing::{Level, span};
 {
     span!(Level::TRACE, "my_span").in_scope(|| {
         // perform some work in the context of `my_span`...
     }); // --> Subscriber::exit(my_span)

     // The handle to `my_span` only lives inside of this block; when it is
     // dropped, the subscriber will be informed via `drop_span`.

 } // --> Subscriber::drop_span(my_span)
 ```

 However, if multiple handles exist, the span can still be re-entered even if
 one or more is dropped. For determining when _all_ handles to a span have
 been dropped, `Subscriber`s have a [`clone_span`] method, which is called
 every time a span handle is cloned. Combined with `drop_span`, this may be
 used to track the number of handles to a given span — if `drop_span` has
 been called one more time than the number of calls to `clone_span` for a
 given ID, then no more handles to the span with that ID exist. The
 subscriber may then treat it as closed.

 # When to use spans

 As a rule of thumb, spans should be used to represent discrete units of work
 (e.g., a given request's lifetime in a server) or periods of time spent in a
 given context (e.g., time spent interacting with an instance of an external
 system, such as a database).

 Which scopes in a program correspond to new spans depend somewhat on user
 intent. For example, consider the case of a loop in a program. Should we
 construct one span and perform the entire loop inside of that span, like:

 ```rust
 # use tracing::{Level, span};
 # let n = 1;
 let span = span!(Level::TRACE, "my_loop");
 let _enter = span.enter();
 for i in 0..n {
     # let _ = i;
     // ...
 }
 ```
 Or, should we create a new span for each iteration of the loop, as in:
 ```rust
 # use tracing::{Level, span};
 # let n = 1u64;
 for i in 0..n {
     let span = span!(Level::TRACE, "my_loop", iteration = i);
     let _enter = span.enter();
     // ...
 }
 ```

 Depending on the circumstances, we might want to do either, or both. For
 example, if we want to know how long was spent in the loop overall, we would
 create a single span around the entire loop; whereas if we wanted to know how
 much time was spent in each individual iteration, we would enter a new span
 on every iteration.

 [fields]: super::field
 [Metadata]: super::Metadata
 [verbosity level]: super::Level
 [`Poll::Ready`]: std::task::Poll::Ready
 [`span!`]: super::span!
 [`trace_span!`]: super::trace_span!
 [`debug_span!`]: super::debug_span!
 [`info_span!`]: super::info_span!
 [`warn_span!`]: super::warn_span!
 [`error_span!`]: super::error_span!
 [`clone_span`]: super::subscriber::Subscriber::clone_span()
 [`drop_span`]: super::subscriber::Subscriber::drop_span()
 [`exit`]: super::subscriber::Subscriber::exit
 [`Subscriber`]: super::subscriber::Subscriber
 [`enter`]: Span::enter()
 [`entered`]: Span::entered()
 [`in_scope`]: Span::in_scope()
 [`follows_from`]: Span::follows_from()
 [guard]: Entered
 [parent]: #span-relationships



## tracing::span

*Declarative Macro*

Constructs a new span.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros

# Examples

Creating a new span:
```
# use tracing::{span, Level};
# fn main() {
let span = span!(Level::TRACE, "my span");
let _enter = span.enter();
// do work inside the span...
# }
```

```rust
macro_rules! span {
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $lvl:expr, $name:expr) => { ... };
    (parent: $parent:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (parent: $parent:expr, $lvl:expr, $name:expr) => { ... };
    (target: $target:expr, $lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    (target: $target:expr, $lvl:expr, $name:expr) => { ... };
    ($lvl:expr, $name:expr, $($fields:tt)*) => { ... };
    ($lvl:expr, $name:expr) => { ... };
}
```



## tracing::span_enabled

*Declarative Macro*

Tests whether a span with the specified level and target would be enabled.

This is similar to [`enabled!`], but queries the current subscriber specifically for
an event, whereas [`enabled!`] queries for an event _or_ span.

See the documentation for [`enabled!]` for more details on using this macro.
See also [`span_enabled!`].

# Examples

```rust
# use tracing::{span_enabled, Level};
if span_enabled!(target: "my_crate", Level::DEBUG) {
    // some expensive work...
}
// simpler
if span_enabled!(Level::DEBUG) {
    // some expensive work...
}
// with fields
if span_enabled!(Level::DEBUG, foo_field) {
    // some expensive work...
}
```

[`enabled!`]: crate::enabled
[`span_enabled!`]: crate::span_enabled

```rust
macro_rules! span_enabled {
    ($($rest:tt)*) => { ... };
}
```



## Module: subscriber

Collects and records trace data.



## tracing::trace

*Declarative Macro*

Constructs an event at the trace level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::trace;
# #[derive(Debug, Copy, Clone)] struct Position { x: f32, y: f32 }
# impl Position {
# const ORIGIN: Self = Self { x: 0.0, y: 0.0 };
# fn dist(&self, other: Position) -> f32 {
#    let x = (other.x - self.x).exp2(); let y = (self.y - other.y).exp2();
#    (x + y).sqrt()
# }
# }
# fn main() {
let pos = Position { x: 3.234, y: -1.223 };
let origin_dist = pos.dist(Position::ORIGIN);

trace!(position = ?pos, ?origin_dist);
trace!(
    target: "app_events",
    position = ?pos,
    "x is {} and y is {}",
    if pos.x >= 0.0 { "positive" } else { "negative" },
    if pos.y >= 0.0 { "positive" } else { "negative" }
);
trace!(name: "completed", position = ?pos);
# }
```

```rust
macro_rules! trace {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



## tracing::trace_span

*Declarative Macro*

Constructs a span at the trace level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{trace_span, span, Level};
# fn main() {
trace_span!("my_span");
// is equivalent to:
span!(Level::TRACE, "my_span");
# }
```

```rust
# use tracing::{trace_span, span, Level};
# fn main() {
let span = trace_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
macro_rules! trace_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
}
```



## tracing::warn

*Declarative Macro*

Constructs an event at the warn level.

This functions similarly to the [`event!`] macro. See [the top-level
documentation][lib] for details on the syntax accepted by
this macro.

[`event!`]: crate::event!
[lib]: crate#using-the-macros

# Examples

```rust
use tracing::warn;
# fn main() {

let warn_description = "Invalid Input";
let input = &[0x27, 0x45];

warn!(?input, warning = warn_description);
warn!(
    target: "input_events",
    warning = warn_description,
    "Received warning for input: {:?}", input,
);
warn!(name: "invalid", ?input);
# }
```

```rust
macro_rules! warn {
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => { ... };
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (name: $name:expr, $($arg:tt)+ ) => { ... };
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => { ... };
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => { ... };
    (target: $target:expr, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => { ... };
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => { ... };
    (parent: $parent:expr, $($arg:tt)+) => { ... };
    ({ $($field:tt)+ }, $($arg:tt)+ ) => { ... };
    ($($k:ident).+ = $($field:tt)*) => { ... };
    (?$($k:ident).+ = $($field:tt)*) => { ... };
    (%$($k:ident).+ = $($field:tt)*) => { ... };
    ($($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+, $($field:tt)*) => { ... };
    (%$($k:ident).+, $($field:tt)*) => { ... };
    (?$($k:ident).+) => { ... };
    (%$($k:ident).+) => { ... };
    ($($k:ident).+) => { ... };
    ($($arg:tt)+) => { ... };
}
```



## tracing::warn_span

*Declarative Macro*

Constructs a span at the warn level.

[Fields] and [attributes] are set using the same syntax as the [`span!`]
macro.

See [the top-level documentation][lib] for details on the syntax accepted by
this macro.

[lib]: crate#using-the-macros
[attributes]: crate#configuring-attributes
[Fields]: crate#recording-fields
[`span!`]: crate::span!

# Examples

```rust
# use tracing::{warn_span, span, Level};
# fn main() {
warn_span!("my_span");
// is equivalent to:
span!(Level::WARN, "my_span");
# }
```

```rust
use tracing::warn_span;
# fn main() {
let span = warn_span!("my span");
span.in_scope(|| {
    // do work inside the span...
});
# }
```

```rust
macro_rules! warn_span {
    (target: $target:expr, parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, parent: $parent:expr, $name:expr) => { ... };
    (parent: $parent:expr, $name:expr, $($field:tt)*) => { ... };
    (parent: $parent:expr, $name:expr) => { ... };
    (target: $target:expr, $name:expr, $($field:tt)*) => { ... };
    (target: $target:expr, $name:expr) => { ... };
    ($name:expr, $($field:tt)*) => { ... };
    ($name:expr) => { ... };
}
```



