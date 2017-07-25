// This file was generated by gir (0f7cd61) from gir-files (0bcaef9)
// DO NOT EDIT

use ffi;
use glib_ffi;
use glib::error::ErrorDomain;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;
use std;

/// An enumerated type which can be used to indicate the cursor blink mode
/// for the terminal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CursorBlinkMode {
    System,
    On,
    Off,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for CursorBlinkMode {
    type GlibType = ffi::VteCursorBlinkMode;

    fn to_glib(&self) -> ffi::VteCursorBlinkMode {
        match *self {
            CursorBlinkMode::System => ffi::VTE_CURSOR_BLINK_SYSTEM,
            CursorBlinkMode::On => ffi::VTE_CURSOR_BLINK_ON,
            CursorBlinkMode::Off => ffi::VTE_CURSOR_BLINK_OFF,
            CursorBlinkMode::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::VteCursorBlinkMode> for CursorBlinkMode {
    fn from_glib(value: ffi::VteCursorBlinkMode) -> Self {
        skip_assert_initialized!();
        match value as i32 {
            0 => CursorBlinkMode::System,
            1 => CursorBlinkMode::On,
            2 => CursorBlinkMode::Off,
            value => CursorBlinkMode::__Unknown(value),
        }
    }
}

impl StaticType for CursorBlinkMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::vte_cursor_blink_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CursorBlinkMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CursorBlinkMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(std::mem::transmute::<i32, ffi::VteCursorBlinkMode>(gobject_ffi::g_value_get_enum(value.to_glib_none().0)))
    }
}

impl SetValue for CursorBlinkMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib() as i32)
    }
}

/// An enumerated type which can be used to indicate what should the terminal
/// draw at the cursor position.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CursorShape {
    Block,
    Ibeam,
    Underline,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for CursorShape {
    type GlibType = ffi::VteCursorShape;

    fn to_glib(&self) -> ffi::VteCursorShape {
        match *self {
            CursorShape::Block => ffi::VTE_CURSOR_SHAPE_BLOCK,
            CursorShape::Ibeam => ffi::VTE_CURSOR_SHAPE_IBEAM,
            CursorShape::Underline => ffi::VTE_CURSOR_SHAPE_UNDERLINE,
            CursorShape::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::VteCursorShape> for CursorShape {
    fn from_glib(value: ffi::VteCursorShape) -> Self {
        skip_assert_initialized!();
        match value as i32 {
            0 => CursorShape::Block,
            1 => CursorShape::Ibeam,
            2 => CursorShape::Underline,
            value => CursorShape::__Unknown(value),
        }
    }
}

impl StaticType for CursorShape {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::vte_cursor_shape_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CursorShape {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CursorShape {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(std::mem::transmute::<i32, ffi::VteCursorShape>(gobject_ffi::g_value_get_enum(value.to_glib_none().0)))
    }
}

impl SetValue for CursorShape {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib() as i32)
    }
}

/// An enumerated type which can be used to indicate which string the terminal
/// should send to an application when the user presses the Delete or Backspace
/// keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EraseBinding {
    Auto,
    AsciiBackspace,
    AsciiDelete,
    DeleteSequence,
    Tty,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for EraseBinding {
    type GlibType = ffi::VteEraseBinding;

    fn to_glib(&self) -> ffi::VteEraseBinding {
        match *self {
            EraseBinding::Auto => ffi::VTE_ERASE_AUTO,
            EraseBinding::AsciiBackspace => ffi::VTE_ERASE_ASCII_BACKSPACE,
            EraseBinding::AsciiDelete => ffi::VTE_ERASE_ASCII_DELETE,
            EraseBinding::DeleteSequence => ffi::VTE_ERASE_DELETE_SEQUENCE,
            EraseBinding::Tty => ffi::VTE_ERASE_TTY,
            EraseBinding::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::VteEraseBinding> for EraseBinding {
    fn from_glib(value: ffi::VteEraseBinding) -> Self {
        skip_assert_initialized!();
        match value as i32 {
            0 => EraseBinding::Auto,
            1 => EraseBinding::AsciiBackspace,
            2 => EraseBinding::AsciiDelete,
            3 => EraseBinding::DeleteSequence,
            4 => EraseBinding::Tty,
            value => EraseBinding::__Unknown(value),
        }
    }
}

impl StaticType for EraseBinding {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::vte_erase_binding_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for EraseBinding {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for EraseBinding {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(std::mem::transmute::<i32, ffi::VteEraseBinding>(gobject_ffi::g_value_get_enum(value.to_glib_none().0)))
    }
}

impl SetValue for EraseBinding {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib() as i32)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PtyError {
    PtyHelperFailed,
    Pty98Failed,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for PtyError {
    type GlibType = ffi::VtePtyError;

    fn to_glib(&self) -> ffi::VtePtyError {
        match *self {
            PtyError::PtyHelperFailed => ffi::VTE_PTY_ERROR_PTY_HELPER_FAILED,
            PtyError::Pty98Failed => ffi::VTE_PTY_ERROR_PTY98_FAILED,
            PtyError::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::VtePtyError> for PtyError {
    fn from_glib(value: ffi::VtePtyError) -> Self {
        skip_assert_initialized!();
        match value as i32 {
            0 => PtyError::PtyHelperFailed,
            1 => PtyError::Pty98Failed,
            value => PtyError::__Unknown(value),
        }
    }
}

impl ErrorDomain for PtyError {
    fn domain() -> glib_ffi::GQuark {
        skip_assert_initialized!();
        unsafe { ffi::vte_pty_error_quark() }
    }

    fn code(self) -> i32 {
        self.to_glib() as i32
    }

    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match code {
            0 => Some(PtyError::PtyHelperFailed),
            1 => Some(PtyError::Pty98Failed),
            value => Some(PtyError::__Unknown(value)),
        }
    }
}

impl StaticType for PtyError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::vte_pty_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for PtyError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for PtyError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(std::mem::transmute::<i32, ffi::VtePtyError>(gobject_ffi::g_value_get_enum(value.to_glib_none().0)))
    }
}

impl SetValue for PtyError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib() as i32)
    }
}

/// A flag type to determine how terminal contents should be written
/// to an output stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WriteFlags {
    Default,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for WriteFlags {
    type GlibType = ffi::VteWriteFlags;

    fn to_glib(&self) -> ffi::VteWriteFlags {
        match *self {
            WriteFlags::Default => ffi::VTE_WRITE_DEFAULT,
            WriteFlags::__Unknown(value) => unsafe{std::mem::transmute(value)}
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::VteWriteFlags> for WriteFlags {
    fn from_glib(value: ffi::VteWriteFlags) -> Self {
        skip_assert_initialized!();
        match value as i32 {
            0 => WriteFlags::Default,
            value => WriteFlags::__Unknown(value),
        }
    }
}

impl StaticType for WriteFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::vte_write_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for WriteFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for WriteFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(std::mem::transmute::<i32, ffi::VteWriteFlags>(gobject_ffi::g_value_get_enum(value.to_glib_none().0)))
    }
}

impl SetValue for WriteFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib() as i32)
    }
}

