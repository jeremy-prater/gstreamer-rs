// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::URIType;
use glib::{prelude::*, translate::*};
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GstURIHandler")]
    pub struct URIHandler(Interface<ffi::GstURIHandler, ffi::GstURIHandlerInterface>);

    match fn {
        type_ => || ffi::gst_uri_handler_get_type(),
    }
}

impl URIHandler {
    pub const NONE: Option<&'static URIHandler> = None;
}

unsafe impl Send for URIHandler {}
unsafe impl Sync for URIHandler {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::URIHandler>> Sealed for T {}
}

pub trait URIHandlerExt: IsA<URIHandler> + sealed::Sealed + 'static {
    #[doc(alias = "gst_uri_handler_get_protocols")]
    #[doc(alias = "get_protocols")]
    fn protocols(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gst_uri_handler_get_protocols(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_uri_handler_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gst_uri_handler_get_uri(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_uri_handler_get_uri_type")]
    #[doc(alias = "get_uri_type")]
    fn uri_type(&self) -> URIType {
        unsafe {
            from_glib(ffi::gst_uri_handler_get_uri_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_uri_handler_set_uri")]
    fn set_uri(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gst_uri_handler_set_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<O: IsA<URIHandler>> URIHandlerExt for O {}
