**ratatui > widgets > stateful_widget_ref**

# Module: widgets::stateful_widget_ref

## Contents

**Traits**

- [`StatefulWidgetRef`](#statefulwidgetref) -  A `StatefulWidgetRef` is a trait that allows rendering a stateful widget by reference.

---

## ratatui::widgets::stateful_widget_ref::StatefulWidgetRef

*Trait*

 A `StatefulWidgetRef` is a trait that allows rendering a stateful widget by reference.

 This is the stateful equivalent of `WidgetRef`. It is useful when you need to store a reference
 to a stateful widget and render it later. It also allows you to render boxed stateful widgets.

 This trait was introduced in Ratatui 0.26.0. It is currently marked as unstable as we are still
 evaluating the API and may make changes in the future. See
 <https://github.com/ratatui/ratatui/issues/1287> for more information.

 A blanket implementation of `StatefulWidgetRef` for `&W` where `W` implements `StatefulWidget`
 is provided. Most of the time you will want to implement `StatefulWidget` against a reference to
 the widget instead of implementing `StatefulWidgetRef` directly.

 See the documentation for [`WidgetRef`] for more information on boxed widgets. See the
 documentation for [`StatefulWidget`] for more information on stateful widgets.

 For comprehensive information about widget implementation patterns, rendering, and usage,
 see the [`widgets`] module documentation.

 [`widgets`]: crate::widgets

 # Examples

 ```rust
 # #[cfg(feature = "unstable-widget-ref")] {
 use ratatui::widgets::StatefulWidgetRef;
 use ratatui_core::buffer::Buffer;
 use ratatui_core::layout::Rect;
 use ratatui_core::style::Stylize;
 use ratatui_core::text::Line;
 use ratatui_core::widgets::{StatefulWidget, Widget};

 struct PersonalGreeting;

 impl StatefulWidgetRef for PersonalGreeting {
     type State = String;
     fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
         Line::raw(format!("Hello {}", state)).render(area, buf);
     }
 }

 impl StatefulWidget for PersonalGreeting {
     type State = String;
     fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
         (&self).render_ref(area, buf, state);
     }
 }

 fn render(area: Rect, buf: &mut Buffer) {
     let widget = PersonalGreeting;
     let mut state = "world".to_string();
     widget.render(area, buf, &mut state);
 }
 # }
 ```
# Stability

**This API is marked as unstable** and is only available when the `unstable-widget-ref`
crate feature is enabled. This comes with no stability guarantees, and could be changed
or removed at any time.

**Methods:**

- `State`: State associated with the stateful widget.
- `render_ref`: Draws the current state of the widget in the given buffer. That is the only method required



