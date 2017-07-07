use Pty;
use ffi;
use glib::translate::*;

impl Pty {
    pub fn new() -> Pty {
        unsafe {
            from_glib_none(ffi::vte_pty_new_sync(ffi::VTE_PTY_DEFAULT, ::std::ptr::null_mut(), ::std::ptr::null_mut()))
        }
    }
}
