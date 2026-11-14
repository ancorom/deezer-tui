**ratatui > widgets**

# Module: widgets

## Contents

**Traits**

- [`FrameExt`](#frameext) -  Extension trait for [`Frame`] that provides methods to render [`WidgetRef`] and

---

## ratatui::widgets::FrameExt

*Trait*

 Extension trait for [`Frame`] that provides methods to render [`WidgetRef`] and
 [`StatefulWidgetRef`] to the current buffer.
# Stability

**This API is marked as unstable** and is only available when the `unstable-widget-ref`
crate feature is enabled. This comes with no stability guarantees, and could be changed
or removed at any time.

**Methods:**

- `render_widget_ref`: Render a [`WidgetRef`] to the current buffer using [`WidgetRef::render_ref`].
- `render_stateful_widget_ref`: Render a [`StatefulWidgetRef`] to the current buffer using



