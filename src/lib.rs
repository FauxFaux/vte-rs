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
