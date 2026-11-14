**tracing > level_filters**

# Module: level_filters

## Contents

**Constants**

- [`STATIC_MAX_LEVEL`](#static_max_level) - The statically configured maximum trace level.

---

## tracing::level_filters::STATIC_MAX_LEVEL

*Constant*: `LevelFilter`

The statically configured maximum trace level.

See the [module-level documentation] for information on how to configure
this.

This value is checked by the `event!` and `span!` macros. Code that
manually constructs events or spans via the `Event::record` function or
`Span` constructors should compare the level against this value to
determine if those spans or events are enabled.

[module-level documentation]: self#compile-time-filters



