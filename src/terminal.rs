#[cfg(feature="v0_48")]
use std::path::PathBuf;
use std::ptr;

use ffi;
use gdk_ffi;
use glib::translate::*;
use glib_ffi;
use pango_ffi;
use Terminal;
use std::ffi::{CStr, CString};
use std::str;

impl Terminal {
    pub fn set_color_background(&self, background: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::vte_terminal_set_color_background(self.to_glib_none().0, background);
        }
    }

    pub fn set_color_bold(&self, bold: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_bold(self.to_glib_none().0, option_to_ptr(bold));
        }
    }

    pub fn set_color_cursor(&self, cursor_background: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor(self.to_glib_none().0, option_to_ptr(cursor_background));
        }
    }

    #[cfg(feature = "v0_44")]
    pub fn set_color_cursor_foreground(&self, cursor_foreground: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_cursor_foreground(self.to_glib_none().0, option_to_ptr(cursor_foreground));
        }
    }

    pub fn set_color_foreground(&self, foreground: &gdk_ffi::GdkRGBA) {
        unsafe {
            ffi::vte_terminal_set_color_foreground(self.to_glib_none().0, foreground);
        }
    }

    pub fn set_color_highlight(&self, highlight_background: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight(self.to_glib_none().0, option_to_ptr(highlight_background));
        }
    }

    pub fn set_color_highlight_foreground(&self, highlight_foreground: Option<&gdk_ffi::GdkRGBA>) {
        unsafe {
            ffi::vte_terminal_set_color_highlight_foreground(self.to_glib_none().0, option_to_ptr(highlight_foreground));
        }
    }

    /* actually not true, since this feature is deprecated, see libvte */
    #[cfg(feature="v0_48")]
    pub fn spawn_sync(&self, working_directory: Option<PathBuf>, argv: &[&str], envv: &[&str]) {
         let directory = working_directory.as_ref().map(|path_buf| path_buf.as_path());
         unsafe {
             ffi::vte_terminal_spawn_sync(
                 self.to_glib_none().0,
                 ffi::VTE_PTY_DEFAULT,
                 directory.to_glib_none().0,
                 argv.to_glib_none().0,
                 envv.to_glib_none().0,
                 glib_ffi::G_SPAWN_DEFAULT, None, ptr::null_mut(),
                 ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
         }
     }
    
    #[cfg(feature="v0_48")]
    pub fn spawn_async(&self, working_directory: Option<PathBuf>, argv: &[&str], envv: &[&str]) {
        let directory = working_directory.as_ref().map(|path_buf| path_buf.as_path());
        unsafe {
            ffi::vte_terminal_spawn_async(self.to_glib_none().0, ffi::VTE_PTY_DEFAULT,
                directory.to_glib_none().0, argv.to_glib_none().0, envv.to_glib_none().0,
                glib_ffi::G_SPAWN_DEFAULT, None, ptr::null_mut(), None, -1, ptr::null_mut(),
                None, ptr::null_mut());
        }
    }

    pub fn feed(&mut self, data: &str) {
        unsafe { ffi::vte_terminal_feed(self.to_glib_none().0, data.as_bytes().as_ptr() as *mut u8, data.len() as isize) }
    }

    pub fn watch_child(&self, child_pid: i32) {
        unsafe { ffi::vte_terminal_watch_child(self.to_glib_none().0, child_pid) }
    }

    pub fn set_font_size(&mut self, size: i32) {
        unsafe {
            let font_desc = ffi::vte_terminal_get_font(self.to_glib_none().0)
                 as *mut pango_ffi::PangoFontDescription;
            pango_ffi::pango_font_description_set_size(font_desc, size * 1024);
            ffi::vte_terminal_set_font(self.to_glib_none().0, font_desc);
        }
    }

    pub fn get_font_size(&self) -> i32 {
        unsafe {
            let font_desc = ffi::vte_terminal_get_font(self.to_glib_none().0)
                 as *const pango_ffi::PangoFontDescription;
            pango_ffi::pango_font_description_get_size(font_desc) / 1024
        }
    }

    pub fn fork_command(&mut self, working_dir: &str, args: &[&str]) -> Result<i32, String> {
        let working_dir_cstr = CString::new(working_dir).unwrap();
        let args_vec_cstr: Vec<CString> = args.iter().map(|s| CString::new(*s).unwrap()).collect();
        let mut args_vec_ptr: Vec<*const i8> = args_vec_cstr.iter().map(|s| s.as_ptr()).collect();
        args_vec_ptr.push(::std::ptr::null());

        unsafe {
            let mut pid = -1;
            let mut err: *mut glib_ffi::GError = ::std::ptr::null_mut();
            let is_success = ffi::vte_terminal_spawn_sync(
                self.to_glib_none().0, // widget
                ffi::VTE_PTY_DEFAULT, // pty_flags
                working_dir_cstr.as_ptr(), // working_directory
                args_vec_ptr.as_ptr() as *mut *mut i8, // argv
                ::std::ptr::null_mut(), // envv
                glib_ffi::G_SPAWN_SEARCH_PATH, // spawn_flags
                None, // child_setup
                ::std::ptr::null_mut(), // child_setup_data
                &mut pid, // child_pid
		::std::ptr::null_mut(), // cancellable
                &mut err // error
            );
            if is_success == glib_ffi::GTRUE {
                Ok(pid)
            } else {
                let err_slice = str::from_utf8_unchecked(CStr::from_ptr((*err).message).to_bytes());
                let err_str = String::from(err_slice);
                glib_ffi::g_error_free(err);
                Err(err_str)
            }
        }
    }
}

fn option_to_ptr<T>(value: Option<&T>) -> *const T {
    match value {
        Some(value) => value as *const _,
        None => ptr::null(),
    }
}
