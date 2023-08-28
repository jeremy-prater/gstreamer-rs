// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstAggregatorPad")]
    pub struct AggregatorPad(Object<ffi::GstAggregatorPad, ffi::GstAggregatorPadClass>) @extends gst::Pad, gst::Object;

    match fn {
        type_ => || ffi::gst_aggregator_pad_get_type(),
    }
}

impl AggregatorPad {
    pub const NONE: Option<&'static AggregatorPad> = None;
}

unsafe impl Send for AggregatorPad {}
unsafe impl Sync for AggregatorPad {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AggregatorPad>> Sealed for T {}
}

pub trait AggregatorPadExt: IsA<AggregatorPad> + sealed::Sealed + 'static {
    #[doc(alias = "gst_aggregator_pad_drop_buffer")]
    fn drop_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_pad_drop_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_14_1")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_14_1")))]
    #[doc(alias = "gst_aggregator_pad_has_buffer")]
    fn has_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_pad_has_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_aggregator_pad_is_eos")]
    fn is_eos(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_pad_is_eos(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_aggregator_pad_is_inactive")]
    fn is_inactive(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_aggregator_pad_is_inactive(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_aggregator_pad_peek_buffer")]
    fn peek_buffer(&self) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_aggregator_pad_peek_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_aggregator_pad_pop_buffer")]
    fn pop_buffer(&self) -> Option<gst::Buffer> {
        unsafe {
            from_glib_full(ffi::gst_aggregator_pad_pop_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "emit-signals")]
    fn emits_signals(&self) -> bool {
        ObjectExt::property(self.as_ref(), "emit-signals")
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "emit-signals")]
    fn set_emit_signals(&self, emit_signals: bool) {
        ObjectExt::set_property(self.as_ref(), "emit-signals", emit_signals)
    }

    #[doc(alias = "buffer-consumed")]
    fn connect_buffer_consumed<F: Fn(&Self, &gst::Buffer) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn buffer_consumed_trampoline<
            P: IsA<AggregatorPad>,
            F: Fn(&P, &gst::Buffer) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAggregatorPad,
            object: *mut gst::ffi::GstBuffer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                AggregatorPad::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"buffer-consumed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    buffer_consumed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "emit-signals")]
    fn connect_emit_signals_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_emit_signals_trampoline<
            P: IsA<AggregatorPad>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAggregatorPad,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AggregatorPad::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::emit-signals\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_emit_signals_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<AggregatorPad>> AggregatorPadExt for O {}
