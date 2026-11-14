**ratatui > widgets > widget_ref**

# Module: widgets::widget_ref

## Contents

**Traits**

- [`WidgetRef`](#widgetref) -  A `WidgetRef` is a trait that allows rendering a widget by reference.

---

## ratatui::widgets::widget_ref::WidgetRef

*Trait*

 A `WidgetRef` is a trait that allows rendering a widget by reference.

 This trait is useful when you want to store a reference to a widget and render it later. It also
 allows you to render boxed widgets.

 Boxed widgets allow you to store widgets with a type that is not known at compile time. This is
 useful when you want to store a collection of widgets with different types. You can then iterate
 over the collection and render each widget.

 This trait was introduced in Ratatui 0.26.0 and is implemented for all the internal widgets. It
 is currently marked as unstable as we are still evaluating the API and may make changes in the
 future. See <https://github.com/ratatui/ratatui/issues/1287> for more information.

 A blanket implementation of `Widget` for `&W` where `W` implements `WidgetRef` is provided.

 A blanket implementation of `WidgetRef` for `Option<W>` where `W` implements `WidgetRef` is
 provided. This is a convenience approach to make it easier to attach child widgets to parent
 widgets. It allows you to render an optional widget by reference.

 For comprehensive information about widget implementation patterns, rendering, and usage,
 see the [`widgets`] module documentation.

 [`widgets`]: crate::widgets

 # Examples

 ```rust
 # #[cfg(feature = "unstable-widget-ref")] {
 use ratatui::widgets::WidgetRef;
 use ratatui_core::buffer::Buffer;
 use ratatui_core::layout::Rect;
 use ratatui_core::text::Line;
 use ratatui_core::widgets::Widget;

 struct Greeting;

 struct Farewell;

 impl WidgetRef for Greeting {
     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
         Line::raw("Hello").render(area, buf);
     }
 }

 /// Only needed for backwards compatibility
 impl Widget for Greeting {
     fn render(self, area: Rect, buf: &mut Buffer) {
         self.render_ref(area, buf);
     }
 }

 impl WidgetRef for Farewell {
     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
         Line::raw("Goodbye").right_aligned().render(area, buf);
     }
 }

 /// Only needed for backwards compatibility
 impl Widget for Farewell {
     fn render(self, area: Rect, buf: &mut Buffer) {
         self.render_ref(area, buf);
     }
 }

 # fn render(area: Rect, buf: &mut Buffer) {
 let greeting = Greeting;
 let farewell = Farewell;

 // these calls do not consume the widgets, so they can be used again later
 greeting.render_ref(area, buf);
 farewell.render_ref(area, buf);

 // a collection of widgets with different types
 let widgets: Vec<Box<dyn WidgetRef>> = vec![Box::new(greeting), Box::new(farewell)];
 for widget in widgets {
     widget.render_ref(area, buf);
 }
 # }
 # }
 ```
# Stability

**This API is marked as unstable** and is only available when the `unstable-widget-ref`
crate feature is enabled. This comes with no stability guarantees, and could be changed
or removed at any time.

**Methods:**

- `render_ref`: Draws the current state of the widget in the given buffer. That is the only method required



