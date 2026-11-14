**tracing_subscriber > layer > context > Context**

# Module: layer::context::Context

## Contents

**Functions**

- [`current_span`](#current_span) - Returns the wrapped subscriber's view of the current span.
- [`enabled`](#enabled) - Returns whether the wrapped subscriber would enable the current span.
- [`event_scope`](#event_scope) - Returns an iterator over the [stored data] for all the spans in the
- [`lookup_current`](#lookup_current) - Returns [stored data] for the span that the wrapped subscriber considers
- [`span`](#span) - Returns [stored data] for the span with the given `id`, if it exists.
- [`span_scope`](#span_scope) - Returns an iterator over the [stored data] for all the spans in the

---

## tracing_subscriber::layer::context::Context::current_span

*Function*

Returns the wrapped subscriber's view of the current span.

```rust
fn current_span(self: &Self) -> span::Current
```



## tracing_subscriber::layer::context::Context::enabled

*Function*

Returns whether the wrapped subscriber would enable the current span.

```rust
fn enabled(self: &Self, metadata: &Metadata) -> bool
```



## tracing_subscriber::layer::context::Context::event_scope

*Function*

Returns an iterator over the [stored data] for all the spans in the
current context, starting with the parent span of the specified event,
and ending with the root of the trace tree and ending with the current span.

<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: Compared to <a href="#method.scope"><code>scope</code></a> this
returns the spans in reverse order (from leaf to root). Use
<a href="../registry/struct.Scope.html#method.from_root"><code>Scope::from_root</code></a>
in case root-to-leaf ordering is desired.
</pre>

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This requires the wrapped subscriber to
    implement the <a href="../registry/trait.LookupSpan.html"><code>
    LookupSpan</code></a> trait. See the documentation on
    <a href="./struct.Context.html"><code>Context</code>'s
    declaration</a> for details.
</pre>

[stored data]: crate::registry::SpanRef

```rust
fn event_scope(self: &Self, event: &Event) -> Option<registry::Scope<S>>
```



## tracing_subscriber::layer::context::Context::lookup_current

*Function*

Returns [stored data] for the span that the wrapped subscriber considers
to be the current.

If this returns `None`, then we are not currently within a span.

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This requires the wrapped subscriber to
    implement the <a href="../registry/trait.LookupSpan.html"><code>
    LookupSpan</code></a> trait. See the documentation on
    <a href="./struct.Context.html"><code>Context</code>'s
    declaration</a> for details.
</pre>

[stored data]: crate::registry::SpanRef

```rust
fn lookup_current(self: &Self) -> Option<registry::SpanRef<S>>
```



## tracing_subscriber::layer::context::Context::span

*Function*

Returns [stored data] for the span with the given `id`, if it exists.

If this returns `None`, then no span exists for that ID (either it has
closed or the ID is invalid).

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This requires the wrapped subscriber to
    implement the <a href="../registry/trait.LookupSpan.html"><code>
    LookupSpan</code></a> trait. See the documentation on
    <a href="./struct.Context.html"><code>Context</code>'s
    declaration</a> for details.
</pre>

[stored data]: crate::registry::SpanRef

```rust
fn span(self: &Self, id: &span::Id) -> Option<registry::SpanRef<S>>
```



## tracing_subscriber::layer::context::Context::span_scope

*Function*

Returns an iterator over the [stored data] for all the spans in the
current context, starting with the specified span and ending with the
root of the trace tree.

<pre class="ignore" style="white-space:normal;font:inherit;">
<strong>Note</strong>: This returns the spans in reverse order (from leaf to root). Use
<a href="../registry/struct.Scope.html#method.from_root"><code>Scope::from_root</code></a>
in case root-to-leaf ordering is desired.
</pre>

<pre class="ignore" style="white-space:normal;font:inherit;">
    <strong>Note</strong>: This requires the wrapped subscriber to
    implement the <a href="../registry/trait.LookupSpan.html"><code>
    LookupSpan</code></a> trait. See the documentation on
    <a href="./struct.Context.html"><code>Context</code>'s
    declaration</a> for details.
</pre>

[stored data]: crate::registry::SpanRef

```rust
fn span_scope(self: &Self, id: &span::Id) -> Option<registry::Scope<S>>
```



