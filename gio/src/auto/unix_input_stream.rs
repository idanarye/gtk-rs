// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::InputStream;
use crate::PollableInputStream;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct UnixInputStream(Object<ffi::GUnixInputStream, ffi::GUnixInputStreamClass>) @extends InputStream, @implements PollableInputStream;

    match fn {
        get_type => || ffi::g_unix_input_stream_get_type(),
    }
}

pub const NONE_UNIX_INPUT_STREAM: Option<&UnixInputStream> = None;

pub trait UnixInputStreamExt: 'static {
    fn get_close_fd(&self) -> bool;
}

impl<O: IsA<UnixInputStream>> UnixInputStreamExt for O {
    fn get_close_fd(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_input_stream_get_close_fd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for UnixInputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnixInputStream")
    }
}
