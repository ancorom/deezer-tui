**ratatui_widgets > calendar**

# Module: calendar

## Contents

**Structs**

- [`CalendarEventStore`](#calendareventstore) - A simple `DateStyler` based on a [`HashMap`]
- [`Monthly`](#monthly) - Display a month calendar for the month containing `display_date`

**Traits**

- [`DateStyler`](#datestyler) - Provides a method for styling a given date. [Monthly] is generic on this trait, so any type

---

## ratatui_widgets::calendar::CalendarEventStore

*Struct*

A simple `DateStyler` based on a [`HashMap`]

**Tuple Struct**: `(hashbrown::HashMap<time::Date, ratatui_core::style::Style>)`

**Methods:**

- `fn today<S>(style: S) -> Self` - Construct a store that has the current date styled.
- `fn add<S>(self: & mut Self, date: Date, style: S)` - Add a date and style to the store

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **DateStyler**
  - `fn get_style(self: &Self, date: Date) -> Style`
- **Default**
  - `fn default() -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ratatui_widgets::calendar::DateStyler

*Trait*

Provides a method for styling a given date. [Monthly] is generic on this trait, so any type
that implements this trait can be used.

**Methods:**

- `get_style`: Given a date, return a style for that date



## ratatui_widgets::calendar::Monthly

*Struct*

Display a month calendar for the month containing `display_date`

**Generic Parameters:**
- 'a
- DS

**Methods:**

- `fn new(display_date: Date, events: DS) -> Self` - Construct a calendar for the `display_date` and highlight the `events`
- `fn show_surrounding<S>(self: Self, style: S) -> Self` - Fill the calendar slots for days not in the current month also, this causes each line to be
- `fn show_weekdays_header<S>(self: Self, style: S) -> Self` - Display a header containing weekday abbreviations
- `fn show_month_header<S>(self: Self, style: S) -> Self` - Display a header containing the month and year
- `fn default_style<S>(self: Self, style: S) -> Self` - How to render otherwise unstyled dates
- `fn block(self: Self, block: Block<'a>) -> Self` - Render the calendar within a [Block]
- `fn width(self: &Self) -> u16` - Return the width required to render the calendar.
- `fn height(self: &Self) -> u16` - Return the height required to render the calendar.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **AsRef**
  - `fn as_ref(self: &Self) -> &crate::calendar::Monthly<'a, DS>`
- **Widget**
  - `fn render(self: Self, area: Rect, buf: & mut Buffer)`
- **Clone**
  - `fn clone(self: &Self) -> Self`



