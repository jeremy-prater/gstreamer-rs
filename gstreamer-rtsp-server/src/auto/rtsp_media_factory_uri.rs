// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTSPMediaFactory;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GstRTSPMediaFactoryURI")]
    pub struct RTSPMediaFactoryURI(Object<ffi::GstRTSPMediaFactoryURI, ffi::GstRTSPMediaFactoryURIClass>) @extends RTSPMediaFactory;

    match fn {
        type_ => || ffi::gst_rtsp_media_factory_uri_get_type(),
    }
}

impl RTSPMediaFactoryURI {
    pub const NONE: Option<&'static RTSPMediaFactoryURI> = None;

    #[doc(alias = "gst_rtsp_media_factory_uri_new")]
    pub fn new() -> RTSPMediaFactoryURI {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_media_factory_uri_new()) }
    }
}

impl Default for RTSPMediaFactoryURI {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPMediaFactoryURI {}
unsafe impl Sync for RTSPMediaFactoryURI {}

pub trait RTSPMediaFactoryURIExt: 'static {
    #[doc(alias = "gst_rtsp_media_factory_uri_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> glib::GString;

    #[doc(alias = "gst_rtsp_media_factory_uri_set_uri")]
    fn set_uri(&self, uri: &str);

    #[doc(alias = "use-gstpay")]
    fn uses_gstpay(&self) -> bool;

    #[doc(alias = "use-gstpay")]
    fn set_use_gstpay(&self, use_gstpay: bool);

    #[doc(alias = "uri")]
    fn connect_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "use-gstpay")]
    fn connect_use_gstpay_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<RTSPMediaFactoryURI>> RTSPMediaFactoryURIExt for O {
    fn uri(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::gst_rtsp_media_factory_uri_get_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gst_rtsp_media_factory_uri_set_uri(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            );
        }
    }

    fn uses_gstpay(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "use-gstpay")
    }

    fn set_use_gstpay(&self, use_gstpay: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "use-gstpay", &use_gstpay)
    }

    fn connect_uri_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_uri_trampoline<
            P: IsA<RTSPMediaFactoryURI>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactoryURI,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactoryURI::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::uri\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_uri_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_use_gstpay_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_gstpay_trampoline<
            P: IsA<RTSPMediaFactoryURI>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPMediaFactoryURI,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPMediaFactoryURI::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-gstpay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_gstpay_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
