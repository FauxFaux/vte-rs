
use ffi;
use gdk_ffi;
use glib::translate::*;
use glib_ffi;
use pango_ffi;
use std::ffi::{CStr, CString};
use std::str;

pub fn get_major_version() -> u32 {
    unsafe {
        ffi::vte_get_major_version()
    }
}
