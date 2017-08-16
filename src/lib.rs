#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
extern crate gdk;
extern crate gdk_sys as gdk_ffi;
extern crate gio_sys as gio_ffi;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate gtk_sys as gtk_ffi;
extern crate libc;
extern crate pango;
extern crate pango_sys as pango_ffi;
extern crate serde;


macro_rules! assert_initialized_main_thread {
    () => (
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

macro_rules! skip_assert_initialized {
    () => ()
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

use glib::Error;
pub use auto::*;

mod ffi;
mod auto;
mod regex;
mod terminal;
mod pty;

mod enums;
pub use self::enums::CursorBlinkMode;
pub use self::enums::CursorShape;
pub use self::enums::EraseBinding;
pub use self::enums::PtyError;
pub use self::enums::WriteFlags;
pub use self::enums::ModifierKey;

mod flags;
pub use self::flags::PtyFlags;
pub use self::flags::PTY_NO_LASTLOG;
pub use self::flags::PTY_NO_UTMP;
pub use self::flags::PTY_NO_WTMP;
pub use self::flags::PTY_NO_HELPER;
pub use self::flags::PTY_NO_FALLBACK;
pub use self::flags::PTY_DEFAULT;

pub mod traits {
    pub use super::pty::PtyExt;
    pub use super::terminal::TerminalExt;

}
