<!-- file * -->
<!-- enum CursorBlinkMode::variant System -->
Follow GTK+ settings for cursor blinking.
<!-- enum CursorBlinkMode::variant On -->
Cursor blinks.
<!-- enum CursorBlinkMode::variant Off -->
Cursor does not blink.
<!-- enum CursorShape::variant Block -->
Draw a block cursor. This is the default.
<!-- enum CursorShape::variant Ibeam -->
Draw a vertical bar on the left side of character.
This is similar to the default cursor for other GTK+ widgets.
<!-- enum CursorShape::variant Underline -->
Draw a horizontal bar below the character.
<!-- enum EraseBinding::variant Auto -->
For backspace, attempt to determine the right value from the terminal's IO settings. For delete, use the control sequence.
<!-- enum EraseBinding::variant AsciiBackspace -->
Send an ASCII backspace character (0x08).
<!-- enum EraseBinding::variant AsciiDelete -->
Send an ASCII delete character (0x7F).
<!-- enum EraseBinding::variant DeleteSequence -->
Send the "@`7`" control sequence.
<!-- enum EraseBinding::variant Tty -->
Send terminal's "erase" setting.
<!-- struct Pty -->


# Implements

[`PtyExt`](trait.PtyExt.html)
<!-- impl Pty::fn new_foreign_sync -->
Creates a new `Pty` for the PTY master `fd`.

No entry will be made in the lastlog, utmp or wtmp system files.

Note that the newly created `Pty` will take ownership of `fd`
and close it on finalize.
## `fd`
a file descriptor to the PTY
## `cancellable`
a `gio::Cancellable`, or `None`

# Returns

a new `Pty` for `fd`, or `None` on error with `error` filled in
<!-- impl Pty::fn new_sync -->
Allocates a new pseudo-terminal.

You can later use `fork` or the `g_spawn_async` family of functions
to start a process on the PTY.

If using `fork`, you MUST call `PtyExt::child_setup` in the child.

If using `g_spawn_async` and friends, you MUST either use
`PtyExt::child_setup` directly as the child setup function, or call
`PtyExt::child_setup` from your own child setup function supplied.

When using `TerminalExt::spawn_sync` with a custom child setup
function, `PtyExt::child_setup` will be called before the supplied
function; you must not call it again.

Also, you MUST pass the `glib::SpawnFlags::DoNotReapChild` flag.

If GNOME PTY Helper is available and
unless some of the `PtyFlags::NoLastlog`, `PtyFlags::NoUtmp` or
`PtyFlags::NoWtmp` flags are passed in `flags`, the
session is logged in the corresponding lastlog, utmp or wtmp
system files. When passing `PtyFlags::NoHelper` in `flags`, the
GNOME PTY Helper is bypassed entirely.

When passing `PtyFlags::NoFallback` in `flags`,
and opening a PTY using the PTY helper fails, there will
be no fallback to allocate a PTY using Unix98 PTY functions.
## `flags`
flags from `PtyFlags`
## `cancellable`
a `gio::Cancellable`, or `None`

# Returns

a new `Pty`, or `None` on error with `error` filled in
<!-- enum PtyError::variant PtyHelperFailed -->
failure when using the GNOME PTY helper to
 allocate the PTY
<!-- enum PtyError::variant Pty98Failed -->
failure when using PTY98 to allocate the PTY
<!-- struct Terminal -->


# Implements

[`TerminalExt`](trait.TerminalExt.html), [`WidgetExt`](trait.WidgetExt.html)
<!-- trait TerminalExt::fn feed -->
Interprets `data` as if it were data received from a child process. This
can either be used to drive the terminal without a child process, or just
to mess with your users.
## `data`
a string in the terminal's current encoding
## `length`
the length of the string, or -1 to use the full length or a nul-terminated string
<!-- trait TerminalExt::fn get_geometry_hints -->
Fills in some `hints` from `self`'s geometry. The hints
filled are those covered by the `gdk::WindowHints::ResizeInc`,
`gdk::WindowHints::MinSize` and `gdk::WindowHints::BaseSize` flags.

See `gtk::Window::set_geometry_hints` for more information.

`self` must be realized (see `gtk::WidgetExt::get_realized`).
## `hints`
a `gdk::Geometry` to fill in
## `min_rows`
the minimum number of rows to request
## `min_columns`
the minimum number of columns to request
<!-- trait TerminalExt::fn get_text -->
Extracts a view of the visible part of the terminal. If `is_selected` is not
`None`, characters will only be read if `is_selected` returns `true` after being
passed the column and row, respectively. A `CharAttributes` structure
is added to `attributes` for each byte added to the returned string detailing
the character's position, colors, and other characteristics.
## `is_selected`
a `VteSelectionFunc` callback
## `user_data`
user data to be passed to the callback
## `attributes`
location for storing text attributes

# Returns

a newly allocated text string, or `None`.
<!-- trait TerminalExt::fn get_text_include_trailing_spaces -->
Extracts a view of the visible part of the terminal. If `is_selected` is not
`None`, characters will only be read if `is_selected` returns `true` after being
passed the column and row, respectively. A `CharAttributes` structure
is added to `attributes` for each byte added to the returned string detailing
the character's position, colors, and other characteristics. This function
differs from `TerminalExt::get_text` in that trailing spaces at the end of
lines are included.
## `is_selected`
a `VteSelectionFunc` callback
## `user_data`
user data to be passed to the callback
## `attributes`
location for storing text attributes

# Returns

a newly allocated text string, or `None`.
<!-- trait TerminalExt::fn get_text_range -->
Extracts a view of the visible part of the terminal. If `is_selected` is not
`None`, characters will only be read if `is_selected` returns `true` after being
passed the column and row, respectively. A `CharAttributes` structure
is added to `attributes` for each byte added to the returned string detailing
the character's position, colors, and other characteristics. The
entire scrollback buffer is scanned, so it is possible to read the entire
contents of the buffer using this function.
## `start_row`
first row to search for data
## `start_col`
first column to search for data
## `end_row`
last row to search for data
## `end_col`
last column to search for data
## `is_selected`
a `VteSelectionFunc` callback
## `user_data`
user data to be passed to the callback
## `attributes`
location for storing text attributes

# Returns

a newly allocated text string, or `None`.
<!-- trait TerminalExt::fn match_add_gregex -->
Adds the regular expression `regex` to the list of matching expressions. When the
user moves the mouse cursor over a section of displayed text which matches
this expression, the text will be highlighted.
## `regex`
a `glib::Regex`
## `flags`
the `glib::RegexMatchFlags` to use when matching the regex

# Returns

an integer associated with this expression
<!-- trait TerminalExt::fn match_check_event -->
Checks if the text in and around the position of the event matches any of the
regular expressions previously set using `vte_terminal_match_add`. If a
match exists, the text string is returned and if `tag` is not `None`, the number
associated with the matched regular expression will be stored in `tag`.

If more than one regular expression has been set with
`vte_terminal_match_add`, then expressions are checked in the order in
which they were added.
## `event`
a ``GdkEvent``
## `tag`
a location to store the tag, or `None`

# Returns

a newly allocated string which matches one of the previously
 set regular expressions
<!-- trait TerminalExt::fn match_set_cursor -->
Sets which cursor the terminal will use if the pointer is over the pattern
specified by `tag`. The terminal keeps a reference to `cursor`.

# Deprecated since 0.40

Use `TerminalExt::match_set_cursor_type` or `vte_terminal_match_set_cursor_named` instead.
## `tag`
the tag of the regex which should use the specified cursor
## `cursor`
the `gdk::Cursor` which the terminal should use when the pattern is
 highlighted, or `None` to use the standard cursor
<!-- trait TerminalExt::fn pty_new_sync -->
Creates a new `Pty`, and sets the emulation property
from `Terminal:emulation`.

See `vte_pty_new` for more information.
## `flags`
flags from `PtyFlags`
## `cancellable`
a `gio::Cancellable`, or `None`

# Returns

a new `Pty`
<!-- trait TerminalExt::fn search_get_gregex -->

# Returns

the search `glib::Regex` regex set in `self`, or `None`
<!-- trait TerminalExt::fn search_set_gregex -->
Sets the `glib::Regex` regex to search for. Unsets the search regex when passed `None`.
## `regex`
a `glib::Regex`, or `None`
## `flags`
flags from `glib::RegexMatchFlags`
<!-- trait TerminalExt::fn set_color_background -->
Sets the background color for text which does not have a specific background
color assigned. Only has effect when no background image is set and when
the terminal is not transparent.
## `background`
the new background color
<!-- trait TerminalExt::fn set_color_bold -->
Sets the color used to draw bold text in the default foreground color.
If `bold` is `None` then the default color is used.
## `bold`
the new bold color or `None`
<!-- trait TerminalExt::fn set_color_cursor -->
Sets the background color for text which is under the cursor. If `None`, text
under the cursor will be drawn with foreground and background colors
reversed.
## `cursor_background`
the new color to use for the text cursor, or `None`
<!-- trait TerminalExt::fn set_color_foreground -->
Sets the foreground color used to draw normal text.
## `foreground`
the new foreground color
<!-- trait TerminalExt::fn set_color_highlight -->
Sets the background color for text which is highlighted. If `None`,
it is unset. If neither highlight background nor highlight foreground are set,
highlighted text (which is usually highlighted because it is selected) will
be drawn with foreground and background colors reversed.
## `highlight_background`
the new color to use for highlighted text, or `None`
<!-- trait TerminalExt::fn set_color_highlight_foreground -->
Sets the foreground color for text which is highlighted. If `None`,
it is unset. If neither highlight background nor highlight foreground are set,
highlighted text (which is usually highlighted because it is selected) will
be drawn with foreground and background colors reversed.
## `highlight_foreground`
the new color to use for highlighted text, or `None`
<!-- trait TerminalExt::fn set_colors -->
`palette` specifies the new values for the 256 palette colors: 8 standard colors,
their 8 bright counterparts, 6x6x6 color cube, and 24 grayscale colors.
Omitted entries will default to a hardcoded value.

`palette_size` must be 0, 8, 16, 232 or 256.

If `foreground` is `None` and `palette_size` is greater than 0, the new foreground
color is taken from `palette`[7]. If `background` is `None` and `palette_size` is
greater than 0, the new background color is taken from `palette`[0].
## `foreground`
the new foreground color, or `None`
## `background`
the new background color, or `None`
## `palette`
the color palette
## `palette_size`
the number of entries in `palette`
<!-- trait TerminalExt::fn set_geometry_hints_for_window -->
Sets `self` as `window`'s geometry widget. See
`gtk::Window::set_geometry_hints` for more information.

`self` must be realized (see `gtk::WidgetExt::get_realized`).
## `window`
a `gtk::Window`
<!-- trait TerminalExt::fn spawn_sync -->
Starts the specified command under a newly-allocated controlling
pseudo-terminal. The `argv` and `envv` lists should be `None`-terminated.
The "TERM" environment variable is automatically set to a default value,
but can be overridden from `envv`.
`pty_flags` controls logging the session to the specified system log files.

Note that `glib::SpawnFlags::DoNotReapChild` will always be added to `spawn_flags`.

Note that unless `spawn_flags` contains `glib::SpawnFlags::LeaveDescriptorsOpen`, all file
descriptors except stdin/stdout/stderr will be closed before calling `exec`
in the child.

See `vte_pty_new`, `g_spawn_async` and `TerminalExt::watch_child` for more information.
## `pty_flags`
flags from `PtyFlags`
## `working_directory`
the name of a directory the command should start
 in, or `None` to use the current working directory
## `argv`
child's argument vector
## `envv`
a list of environment
 variables to be added to the environment before starting the process, or `None`
## `spawn_flags`
flags from `glib::SpawnFlags`
## `child_setup`
an extra child setup function to run in the child just before `exec`, or `None`
## `child_setup_data`
user data for `child_setup`
## `child_pid`
a location to store the child PID, or `None`
## `cancellable`
a `gio::Cancellable`, or `None`

# Returns

`true` on success, or `false` on error with `error` filled in
<!-- trait TerminalExt::fn watch_child -->
Watches `child_pid`. When the process exists, the `Terminal::child-exited`
signal will be called with the child's exit status.

Prior to calling this function, a `Pty` must have been set in `self`
using `TerminalExt::set_pty`.
When the child exits, the terminal's `Pty` will be set to `None`.

Note: `g_child_watch_add` or `g_child_watch_add_full` must not have
been called for `child_pid`, nor a `glib::Source` for it been created with
`g_child_watch_source_new`.

Note: when using the `g_spawn_async` family of functions,
the `glib::SpawnFlags::DoNotReapChild` flag MUST have been passed.
## `child_pid`
a `glib::Pid`
<!-- trait TerminalExt::fn write_contents_sync -->
Write contents of the current contents of `self` (including any
scrollback history) to `stream` according to `flags`.

If `cancellable` is not `None`, then the operation can be cancelled by triggering
the cancellable object from another thread. If the operation was cancelled,
the error `gio::IOErrorEnum::Cancelled` will be returned in `error`.

This is a synchronous operation and will make the widget (and input
processing) during the write operation, which may take a long time
depending on scrollback history and `stream` availability for writing.
## `stream`
a `gio::OutputStream` to write to
## `flags`
a set of `WriteFlags`
## `cancellable`
a `gio::Cancellable` object, or `None`

# Returns

`true` on success, `false` if there was an error
<!-- enum WriteFlags::variant Default -->
Write contents as UTF-8 text. This is the default.
