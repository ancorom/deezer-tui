**crossterm > event**

# Module: event

## Contents

**Structs**

- [`DisableBracketedPaste`](#disablebracketedpaste) - A command that disables bracketed paste mode.
- [`DisableFocusChange`](#disablefocuschange) - A command that disables focus event emission.
- [`DisableMouseCapture`](#disablemousecapture) - A command that disables mouse event capturing.
- [`EnableBracketedPaste`](#enablebracketedpaste) - A command that enables [bracketed paste mode](https://en.wikipedia.org/wiki/Bracketed-paste).
- [`EnableFocusChange`](#enablefocuschange) - A command that enables focus event emission.
- [`EnableMouseCapture`](#enablemousecapture) - A command that enables mouse event capturing.
- [`KeyEvent`](#keyevent) - Represents a key event.
- [`KeyEventState`](#keyeventstate) - Represents extra state about the key event.
- [`KeyModifiers`](#keymodifiers) - Represents key modifiers (shift, control, alt, etc.).
- [`KeyboardEnhancementFlags`](#keyboardenhancementflags) - Represents special flags that tell compatible terminals to add extra information to keyboard events.
- [`MouseEvent`](#mouseevent) - Represents a mouse event.
- [`PopKeyboardEnhancementFlags`](#popkeyboardenhancementflags) - A command that disables extra kinds of keyboard events.
- [`PushKeyboardEnhancementFlags`](#pushkeyboardenhancementflags) - A command that enables the [kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/), which adds extra information to keyboard events and removes ambiguity for modifier keys.

**Enums**

- [`Event`](#event) - Represents an event.
- [`KeyCode`](#keycode) - Represents a key.
- [`KeyEventKind`](#keyeventkind) - Represents a keyboard event kind.
- [`MediaKeyCode`](#mediakeycode) - Represents a media key (as part of [`KeyCode::Media`]).
- [`ModifierKeyCode`](#modifierkeycode) - Represents a modifier key (as part of [`KeyCode::Modifier`]).
- [`MouseButton`](#mousebutton) - Represents a mouse button.
- [`MouseEventKind`](#mouseeventkind) - A mouse event kind.

**Functions**

- [`poll`](#poll) - Checks if there is an [`Event`](enum.Event.html) available.
- [`read`](#read) - Reads a single [`Event`](enum.Event.html).

---

## crossterm::event::DisableBracketedPaste

*Struct*

A command that disables bracketed paste mode.

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## crossterm::event::DisableFocusChange

*Struct*

A command that disables focus event emission.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## crossterm::event::DisableMouseCapture

*Struct*

A command that disables mouse event capturing.

Mouse events can be captured with [read](./fn.read.html)/[poll](./fn.poll.html).

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`



## crossterm::event::EnableBracketedPaste

*Struct*

A command that enables [bracketed paste mode](https://en.wikipedia.org/wiki/Bracketed-paste).

It should be paired with [`DisableBracketedPaste`] at the end of execution.

This is not supported in older Windows terminals without
[virtual terminal sequences](https://docs.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences).

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## crossterm::event::EnableFocusChange

*Struct*

A command that enables focus event emission.

It should be paired with [`DisableFocusChange`] at the end of execution.

Focus events can be captured with [read](./fn.read.html)/[poll](./fn.poll.html).

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## crossterm::event::EnableMouseCapture

*Struct*

A command that enables mouse event capturing.

Mouse events can be captured with [read](./fn.read.html)/[poll](./fn.poll.html).

**Unit Struct**

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`



## crossterm::event::Event

*Enum*

Represents an event.

**Variants:**
- `FocusGained` - The terminal gained focus
- `FocusLost` - The terminal lost focus
- `Key(KeyEvent)` - A single key event with additional pressed modifiers.
- `Mouse(MouseEvent)` - A single mouse event with additional pressed modifiers.
- `Paste(String)` - A string that was pasted into the terminal. Only emitted if bracketed paste has been
- `Resize(u16, u16)` - An resize event with new dimensions after resize (columns, rows).

**Methods:**

- `fn is_focus_gained(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_focus_lost(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_key(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_mouse(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_paste(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_resize(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_key_press(self: &Self) -> bool` - Returns `true` if the event is a key press event.
- `fn is_key_release(self: &Self) -> bool` - Returns `true` if the event is a key release event.
- `fn is_key_repeat(self: &Self) -> bool` - Returns `true` if the event is a key repeat event.
- `fn as_key_event(self: &Self) -> Option<KeyEvent>` - Returns the key event if the event is a key event, otherwise `None`.
- `fn as_key_press_event(self: &Self) -> Option<KeyEvent>` - Returns an Option containing the KeyEvent if the event is a key press event.
- `fn as_key_release_event(self: &Self) -> Option<KeyEvent>` - Returns an Option containing the `KeyEvent` if the event is a key release event.
- `fn as_key_repeat_event(self: &Self) -> Option<KeyEvent>` - Returns an Option containing the `KeyEvent` if the event is a key repeat event.
- `fn as_mouse_event(self: &Self) -> Option<MouseEvent>` - Returns the mouse event if the event is a mouse event, otherwise `None`.
- `fn as_paste_event(self: &Self) -> Option<&str>` - Returns the pasted string if the event is a paste event, otherwise `None`.
- `fn as_resize_event(self: &Self) -> Option<(u16, u16)>` - Returns the size as a tuple if the event is a resize event, otherwise `None`.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## crossterm::event::KeyCode

*Enum*

Represents a key.

**Variants:**
- `Backspace` - Backspace key (Delete on macOS, Backspace on other platforms).
- `Enter` - Enter key.
- `Left` - Left arrow key.
- `Right` - Right arrow key.
- `Up` - Up arrow key.
- `Down` - Down arrow key.
- `Home` - Home key.
- `End` - End key.
- `PageUp` - Page up key.
- `PageDown` - Page down key.
- `Tab` - Tab key.
- `BackTab` - Shift + Tab key.
- `Delete` - Delete key. (Fn+Delete on macOS, Delete on other platforms)
- `Insert` - Insert key.
- `F(u8)` - F key.
- `Char(char)` - A character.
- `Null` - Null.
- `Esc` - Escape key.
- `CapsLock` - Caps Lock key.
- `ScrollLock` - Scroll Lock key.
- `NumLock` - Num Lock key.
- `PrintScreen` - Print Screen key.
- `Pause` - Pause key.
- `Menu` - Menu key.
- `KeypadBegin` - The "Begin" key (often mapped to the 5 key when Num Lock is turned on).
- `Media(MediaKeyCode)` - A media key.
- `Modifier(ModifierKeyCode)` - A modifier key.

**Methods:**

- `fn is_backspace(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_enter(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_left(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_right(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_up(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_down(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_home(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_end(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_page_up(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_page_down(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_tab(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_back_tab(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_delete(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_insert(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_null(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_esc(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_caps_lock(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_scroll_lock(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_num_lock(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_print_screen(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_pause(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_menu(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_keypad_begin(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_function_key(self: &Self, n: u8) -> bool` - Returns `true` if the key code is the given function key.
- `fn is_char(self: &Self, c: char) -> bool` - Returns `true` if the key code is the given character.
- `fn as_char(self: &Self) -> Option<char>` - Returns the character if the key code is a character key.
- `fn is_media_key(self: &Self, media: MediaKeyCode) -> bool` - Returns `true` if the key code is the given media key.
- `fn is_modifier(self: &Self, modifier: ModifierKeyCode) -> bool` - Returns `true` if the key code is the given modifier key.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Formats the `KeyCode` using the given formatter.
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## crossterm::event::KeyEvent

*Struct*

Represents a key event.

**Fields:**
- `code: KeyCode` - The key itself.
- `modifiers: KeyModifiers` - Additional key modifiers.
- `kind: KeyEventKind` - Kind of event.
- `state: KeyEventState` - Keyboard state.

**Methods:**

- `fn new(code: KeyCode, modifiers: KeyModifiers) -> KeyEvent`
- `fn new_with_kind(code: KeyCode, modifiers: KeyModifiers, kind: KeyEventKind) -> KeyEvent`
- `fn new_with_kind_and_state(code: KeyCode, modifiers: KeyModifiers, kind: KeyEventKind, state: KeyEventState) -> KeyEvent`
- `fn is_press(self: &Self) -> bool` - Returns whether the key event is a press event.
- `fn is_release(self: &Self) -> bool` - Returns whether the key event is a release event.
- `fn is_repeat(self: &Self) -> bool` - Returns whether the key event is a repeat event.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &KeyEvent) -> bool`
- **Hash**
  - `fn hash<H>(self: &Self, hash_state: & mut H)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **From**
  - `fn from(code: KeyCode) -> Self`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## crossterm::event::KeyEventKind

*Enum*

Represents a keyboard event kind.

**Variants:**
- `Press`
- `Repeat`
- `Release`

**Methods:**

- `fn is_press(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_repeat(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_release(self: &Self) -> bool` - Returns `true` if this value is of type `

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## crossterm::event::KeyEventState

*Struct*

Represents extra state about the key event.

**Note:** This state can only be read if
[`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
[`PushKeyboardEnhancementFlags`].

**Tuple Struct**: `()`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u8` - Get the underlying bits value.
- `fn from_bits(bits: u8) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u8) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u8) -> Self` - Convert from a bits value exactly.
- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>` - Get a flags value with the bits of a flag with the given name set.
- `fn is_empty(self: &Self) -> bool` - Whether all bits in `self` are unset.
- `fn is_all(self: &Self) -> bool` - Whether all known bits in this flags value are set.
- `fn intersects(self: &Self, other: Self) -> bool` - Whether any set bits in `other` are also set in `self`.
- `fn contains(self: &Self, other: Self) -> bool` - Whether all set bits in `other` are also set in `self`.
- `fn insert(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn remove(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- `fn toggle(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn set(self: & mut Self, other: Self, value: bool)` - Call `insert` when `value` is `true` or `remove` when `value` is `false`.
- `fn intersection(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- `fn union(self: Self, other: Self) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn difference(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- `fn symmetric_difference(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn complement(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.
- `fn iter(self: &Self) -> $crate::iter::Iter<KeyEventState>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<KeyEventState>` - Yield a set of contained named flags values.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Flags**
  - `fn bits(self: &Self) -> u8`
  - `fn from_bits_retain(bits: u8) -> KeyEventState`
  - `fn all_named() -> KeyEventState`
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: KeyEventState) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).



## crossterm::event::KeyModifiers

*Struct*

Represents key modifiers (shift, control, alt, etc.).

**Note:** `SUPER`, `HYPER`, and `META` can only be read if
[`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
[`PushKeyboardEnhancementFlags`].

**Tuple Struct**: `()`

**Methods:**

- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u8` - Get the underlying bits value.
- `fn from_bits(bits: u8) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u8) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u8) -> Self` - Convert from a bits value exactly.
- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>` - Get a flags value with the bits of a flag with the given name set.
- `fn is_empty(self: &Self) -> bool` - Whether all bits in `self` are unset.
- `fn is_all(self: &Self) -> bool` - Whether all known bits in this flags value are set.
- `fn intersects(self: &Self, other: Self) -> bool` - Whether any set bits in `other` are also set in `self`.
- `fn contains(self: &Self, other: Self) -> bool` - Whether all set bits in `other` are also set in `self`.
- `fn insert(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn remove(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- `fn toggle(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn set(self: & mut Self, other: Self, value: bool)` - Call `insert` when `value` is `true` or `remove` when `value` is `false`.
- `fn intersection(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- `fn union(self: Self, other: Self) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn difference(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- `fn symmetric_difference(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn complement(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.
- `fn iter(self: &Self) -> $crate::iter::Iter<KeyModifiers>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<KeyModifiers>` - Yield a set of contained named flags values.

**Traits:** PublicFlags, Eq, Copy

**Trait Implementations:**

- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **BitOr**
  - `fn bitor(self: Self, other: KeyModifiers) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Formats the key modifiers using the given formatter.
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Flags**
  - `fn bits(self: &Self) -> u8`
  - `fn from_bits_retain(bits: u8) -> KeyModifiers`
  - `fn all_named() -> KeyModifiers`
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.



## crossterm::event::KeyboardEnhancementFlags

*Struct*

Represents special flags that tell compatible terminals to add extra information to keyboard events.

See <https://sw.kovidgoyal.net/kitty/keyboard-protocol/#progressive-enhancement> for more information.

Alternate keys and Unicode codepoints are not yet supported by crossterm.

**Tuple Struct**: `()`

**Methods:**

- `fn iter(self: &Self) -> $crate::iter::Iter<KeyboardEnhancementFlags>` - Yield a set of contained flags values.
- `fn iter_names(self: &Self) -> $crate::iter::IterNames<KeyboardEnhancementFlags>` - Yield a set of contained named flags values.
- `fn empty() -> Self` - Get a flags value with all bits unset.
- `fn all() -> Self` - Get a flags value with all known bits set.
- `fn bits(self: &Self) -> u8` - Get the underlying bits value.
- `fn from_bits(bits: u8) -> $crate::__private::core::option::Option<Self>` - Convert from a bits value.
- `fn from_bits_truncate(bits: u8) -> Self` - Convert from a bits value, unsetting any unknown bits.
- `fn from_bits_retain(bits: u8) -> Self` - Convert from a bits value exactly.
- `fn from_name(name: &str) -> $crate::__private::core::option::Option<Self>` - Get a flags value with the bits of a flag with the given name set.
- `fn is_empty(self: &Self) -> bool` - Whether all bits in `self` are unset.
- `fn is_all(self: &Self) -> bool` - Whether all known bits in this flags value are set.
- `fn intersects(self: &Self, other: Self) -> bool` - Whether any set bits in `other` are also set in `self`.
- `fn contains(self: &Self, other: Self) -> bool` - Whether all set bits in `other` are also set in `self`.
- `fn insert(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn remove(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- `fn toggle(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn set(self: & mut Self, other: Self, value: bool)` - Call `insert` when `value` is `true` or `remove` when `value` is `false`.
- `fn intersection(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- `fn union(self: Self, other: Self) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- `fn difference(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- `fn symmetric_difference(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- `fn complement(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.

**Traits:** Eq, Copy, PublicFlags

**Trait Implementations:**

- **BitAnd**
  - `fn bitand(self: Self, other: Self) -> Self` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Binary**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Flags**
  - `fn bits(self: &Self) -> u8`
  - `fn from_bits_retain(bits: u8) -> KeyboardEnhancementFlags`
  - `fn all_named() -> KeyboardEnhancementFlags`
- **BitAndAssign**
  - `fn bitand_assign(self: & mut Self, other: Self)` - The bitwise and (`&`) of the bits in `self` and `other`.
- **Octal**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Not**
  - `fn not(self: Self) -> Self` - The bitwise negation (`!`) of the bits in `self`, truncating the result.
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **UpperHex**
  - `fn fmt(self: &Self, f: & mut $crate::__private::core::fmt::Formatter) -> $crate::__private::core::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **BitXor**
  - `fn bitxor(self: Self, other: Self) -> Self` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **BitXorAssign**
  - `fn bitxor_assign(self: & mut Self, other: Self)` - The bitwise exclusive-or (`^`) of the bits in `self` and `other`.
- **Sub**
  - `fn sub(self: Self, other: Self) -> Self` - The intersection of `self` with the complement of `other` (`&!`).
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Extend**
  - `fn extend<T>(self: & mut Self, iterator: T)` - The bitwise or (`|`) of the bits in each flags value.
- **SubAssign**
  - `fn sub_assign(self: & mut Self, other: Self)` - The intersection of `self` with the complement of `other` (`&!`).
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **FromIterator**
  - `fn from_iter<T>(iterator: T) -> Self` - The bitwise or (`|`) of the bits in each flags value.
- **BitOr**
  - `fn bitor(self: Self, other: KeyboardEnhancementFlags) -> Self` - The bitwise or (`|`) of the bits in `self` and `other`.
- **BitOrAssign**
  - `fn bitor_assign(self: & mut Self, other: Self)` - The bitwise or (`|`) of the bits in `self` and `other`.



## crossterm::event::MediaKeyCode

*Enum*

Represents a media key (as part of [`KeyCode::Media`]).

**Variants:**
- `Play` - Play media key.
- `Pause` - Pause media key.
- `PlayPause` - Play/Pause media key.
- `Reverse` - Reverse media key.
- `Stop` - Stop media key.
- `FastForward` - Fast-forward media key.
- `Rewind` - Rewind media key.
- `TrackNext` - Next-track media key.
- `TrackPrevious` - Previous-track media key.
- `Record` - Record media key.
- `LowerVolume` - Lower-volume media key.
- `RaiseVolume` - Raise-volume media key.
- `MuteVolume` - Mute media key.

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## crossterm::event::ModifierKeyCode

*Enum*

Represents a modifier key (as part of [`KeyCode::Modifier`]).

**Variants:**
- `LeftShift` - Left Shift key.
- `LeftControl` - Left Control key. (Control on macOS, Ctrl on other platforms)
- `LeftAlt` - Left Alt key. (Option on macOS, Alt on other platforms)
- `LeftSuper` - Left Super key. (Command on macOS, Windows on Windows, Super on other platforms)
- `LeftHyper` - Left Hyper key.
- `LeftMeta` - Left Meta key.
- `RightShift` - Right Shift key.
- `RightControl` - Right Control key. (Control on macOS, Ctrl on other platforms)
- `RightAlt` - Right Alt key. (Option on macOS, Alt on other platforms)
- `RightSuper` - Right Super key. (Command on macOS, Windows on Windows, Super on other platforms)
- `RightHyper` - Right Hyper key.
- `RightMeta` - Right Meta key.
- `IsoLevel3Shift` - Iso Level3 Shift key.
- `IsoLevel5Shift` - Iso Level5 Shift key.

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result` - Formats the modifier key using the given formatter.



## crossterm::event::MouseButton

*Enum*

Represents a mouse button.

**Variants:**
- `Left` - Left mouse button.
- `Right` - Right mouse button.
- `Middle` - Middle mouse button.

**Methods:**

- `fn is_left(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_right(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_middle(self: &Self) -> bool` - Returns `true` if this value is of type `

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## crossterm::event::MouseEvent

*Struct*

Represents a mouse event.

# Platform-specific Notes

## Mouse Buttons

Some platforms/terminals do not report mouse button for the
`MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`
is returned if we don't know which button was used.

## Key Modifiers

Some platforms/terminals does not report all key modifiers
combinations for all mouse event types. For example - macOS reports
`Ctrl` + left mouse button click as a right mouse button click.

**Fields:**
- `kind: MouseEventKind` - The kind of mouse event that was caused.
- `column: u16` - The column that the event occurred on.
- `row: u16` - The row that the event occurred on.
- `modifiers: KeyModifiers` - The key modifiers active when the event occurred.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`



## crossterm::event::MouseEventKind

*Enum*

A mouse event kind.

# Platform-specific Notes

## Mouse Buttons

Some platforms/terminals do not report mouse button for the
`MouseEventKind::Up` and `MouseEventKind::Drag` events. `MouseButton::Left`
is returned if we don't know which button was used.

**Variants:**
- `Down(MouseButton)` - Pressed mouse button. Contains the button that was pressed.
- `Up(MouseButton)` - Released mouse button. Contains the button that was released.
- `Drag(MouseButton)` - Moved the mouse cursor while pressing the contained mouse button.
- `Moved` - Moved the mouse cursor while not pressing a mouse button.
- `ScrollDown` - Scrolled mouse wheel downwards (towards the user).
- `ScrollUp` - Scrolled mouse wheel upwards (away from the user).
- `ScrollLeft` - Scrolled mouse wheel left (mostly on a laptop touchpad).
- `ScrollRight` - Scrolled mouse wheel right (mostly on a laptop touchpad).

**Methods:**

- `fn is_down(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_up(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_drag(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_moved(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_scroll_down(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_scroll_up(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_scroll_left(self: &Self) -> bool` - Returns `true` if this value is of type `
- `fn is_scroll_right(self: &Self) -> bool` - Returns `true` if this value is of type `

**Traits:** Eq, Copy

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Self) -> $crate::option::Option<$crate::cmp::Ordering>`



## crossterm::event::PopKeyboardEnhancementFlags

*Struct*

A command that disables extra kinds of keyboard events.

Specifically, it pops one level of keyboard enhancement flags.

See [`PushKeyboardEnhancementFlags`] and <https://sw.kovidgoyal.net/kitty/keyboard-protocol/> for more information.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`
- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`



## crossterm::event::PushKeyboardEnhancementFlags

*Struct*

A command that enables the [kitty keyboard protocol](https://sw.kovidgoyal.net/kitty/keyboard-protocol/), which adds extra information to keyboard events and removes ambiguity for modifier keys.

It should be paired with [`PopKeyboardEnhancementFlags`] at the end of execution.

Example usage:
```no_run
use std::io::{Write, stdout};
use crossterm::execute;
use crossterm::event::{
    KeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
    PopKeyboardEnhancementFlags
};

let mut stdout = stdout();

execute!(
    stdout,
    PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
    )
);

// ...

execute!(stdout, PopKeyboardEnhancementFlags);
```

Note that, currently, only the following support this protocol:
* [kitty terminal](https://sw.kovidgoyal.net/kitty/)
* [foot terminal](https://codeberg.org/dnkl/foot/issues/319)
* [WezTerm terminal](https://wezfurlong.org/wezterm/config/lua/config/enable_kitty_keyboard.html)
* [alacritty terminal](https://github.com/alacritty/alacritty/issues/6378)
* [notcurses library](https://github.com/dankamongmen/notcurses/issues/2131)
* [neovim text editor](https://github.com/neovim/neovim/pull/18181)
* [kakoune text editor](https://github.com/mawww/kakoune/issues/4103)
* [dte text editor](https://gitlab.com/craigbarnes/dte/-/issues/138)

**Tuple Struct**: `(KeyboardEnhancementFlags)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Command**
  - `fn write_ansi<impl fmt::Write>(self: &Self, f: & mut impl Trait) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## crossterm::event::poll

*Function*

Checks if there is an [`Event`](enum.Event.html) available.

Returns `Ok(true)` if an [`Event`](enum.Event.html) is available otherwise it returns `Ok(false)`.

`Ok(true)` guarantees that subsequent call to the [`read`](fn.read.html) function
won't block.

# Arguments

* `timeout` - maximum waiting time for event availability

# Examples

Return immediately:

```no_run
use std::{time::Duration, io};
use crossterm::{event::poll};

fn is_event_available() -> io::Result<bool> {
    // Zero duration says that the `poll` function must return immediately
    // with an `Event` availability information
    poll(Duration::from_secs(0))
}
```

Wait up to 100ms:

```no_run
use std::{time::Duration, io};

use crossterm::event::poll;

fn is_event_available() -> io::Result<bool> {
    // Wait for an `Event` availability for 100ms. It returns immediately
    // if an `Event` is/becomes available.
    poll(Duration::from_millis(100))
}
```

```rust
fn poll(timeout: std::time::Duration) -> std::io::Result<bool>
```



## crossterm::event::read

*Function*

Reads a single [`Event`](enum.Event.html).

This function blocks until an [`Event`](enum.Event.html) is available. Combine it with the
[`poll`](fn.poll.html) function to get non-blocking reads.

# Examples

Blocking read:

```no_run
use crossterm::event::read;
use std::io;

fn print_events() -> io::Result<bool> {
    loop {
        // Blocks until an `Event` is available
        println!("{:?}", read()?);
    }
}
```

Non-blocking read:

```no_run
use std::time::Duration;
use std::io;

use crossterm::event::{read, poll};

fn print_events() -> io::Result<bool> {
    loop {
        if poll(Duration::from_millis(100))? {
            // It's guaranteed that `read` won't block, because `poll` returned
            // `Ok(true)`.
            println!("{:?}", read()?);
        } else {
            // Timeout expired, no `Event` is available
        }
    }
}
```

```rust
fn read() -> std::io::Result<Event>
```



