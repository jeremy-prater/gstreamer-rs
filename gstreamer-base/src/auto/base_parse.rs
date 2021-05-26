// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

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
    pub struct BaseParse(Object<ffi::GstBaseParse, ffi::GstBaseParseClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_base_parse_get_type(),
    }
}

unsafe impl Send for BaseParse {}
unsafe impl Sync for BaseParse {}

pub const NONE_BASE_PARSE: Option<&BaseParse> = None;

pub trait BaseParseExt: 'static {
    #[doc(alias = "gst_base_parse_add_index_entry")]
    fn add_index_entry(&self, offset: u64, ts: gst::ClockTime, key: bool, force: bool) -> bool;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "gst_base_parse_drain")]
    fn drain(&self);

    #[doc(alias = "gst_base_parse_merge_tags")]
    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode);

    #[doc(alias = "gst_base_parse_set_average_bitrate")]
    fn set_average_bitrate(&self, bitrate: u32);

    #[doc(alias = "gst_base_parse_set_has_timing_info")]
    fn set_has_timing_info(&self, has_timing: bool);

    #[doc(alias = "gst_base_parse_set_infer_ts")]
    fn set_infer_ts(&self, infer_ts: bool);

    #[doc(alias = "gst_base_parse_set_latency")]
    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime);

    #[doc(alias = "gst_base_parse_set_min_frame_size")]
    fn set_min_frame_size(&self, min_size: u32);

    #[doc(alias = "gst_base_parse_set_passthrough")]
    fn set_passthrough(&self, passthrough: bool);

    #[doc(alias = "gst_base_parse_set_pts_interpolation")]
    fn set_pts_interpolation(&self, pts_interpolate: bool);

    #[doc(alias = "gst_base_parse_set_syncable")]
    fn set_syncable(&self, syncable: bool);

    #[doc(alias = "gst_base_parse_set_ts_at_offset")]
    fn set_ts_at_offset(&self, offset: usize);

    #[doc(alias = "disable-passthrough")]
    fn is_disable_passthrough(&self) -> bool;

    #[doc(alias = "disable-passthrough")]
    fn set_disable_passthrough(&self, disable_passthrough: bool);

    #[doc(alias = "disable-passthrough")]
    fn connect_disable_passthrough_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<BaseParse>> BaseParseExt for O {
    fn add_index_entry(&self, offset: u64, ts: gst::ClockTime, key: bool, force: bool) -> bool {
        unsafe {
            from_glib(ffi::gst_base_parse_add_index_entry(
                self.as_ref().to_glib_none().0,
                offset,
                ts.into_glib(),
                key.into_glib(),
                force.into_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    fn drain(&self) {
        unsafe {
            ffi::gst_base_parse_drain(self.as_ref().to_glib_none().0);
        }
    }

    fn merge_tags(&self, tags: Option<&gst::TagList>, mode: gst::TagMergeMode) {
        unsafe {
            ffi::gst_base_parse_merge_tags(
                self.as_ref().to_glib_none().0,
                tags.to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    fn set_average_bitrate(&self, bitrate: u32) {
        unsafe {
            ffi::gst_base_parse_set_average_bitrate(self.as_ref().to_glib_none().0, bitrate);
        }
    }

    fn set_has_timing_info(&self, has_timing: bool) {
        unsafe {
            ffi::gst_base_parse_set_has_timing_info(
                self.as_ref().to_glib_none().0,
                has_timing.into_glib(),
            );
        }
    }

    fn set_infer_ts(&self, infer_ts: bool) {
        unsafe {
            ffi::gst_base_parse_set_infer_ts(self.as_ref().to_glib_none().0, infer_ts.into_glib());
        }
    }

    fn set_latency(&self, min_latency: gst::ClockTime, max_latency: gst::ClockTime) {
        unsafe {
            ffi::gst_base_parse_set_latency(
                self.as_ref().to_glib_none().0,
                min_latency.into_glib(),
                max_latency.into_glib(),
            );
        }
    }

    fn set_min_frame_size(&self, min_size: u32) {
        unsafe {
            ffi::gst_base_parse_set_min_frame_size(self.as_ref().to_glib_none().0, min_size);
        }
    }

    fn set_passthrough(&self, passthrough: bool) {
        unsafe {
            ffi::gst_base_parse_set_passthrough(
                self.as_ref().to_glib_none().0,
                passthrough.into_glib(),
            );
        }
    }

    fn set_pts_interpolation(&self, pts_interpolate: bool) {
        unsafe {
            ffi::gst_base_parse_set_pts_interpolation(
                self.as_ref().to_glib_none().0,
                pts_interpolate.into_glib(),
            );
        }
    }

    fn set_syncable(&self, syncable: bool) {
        unsafe {
            ffi::gst_base_parse_set_syncable(self.as_ref().to_glib_none().0, syncable.into_glib());
        }
    }

    fn set_ts_at_offset(&self, offset: usize) {
        unsafe {
            ffi::gst_base_parse_set_ts_at_offset(self.as_ref().to_glib_none().0, offset);
        }
    }

    fn is_disable_passthrough(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"disable-passthrough\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `disable-passthrough` getter")
        }
    }

    fn set_disable_passthrough(&self, disable_passthrough: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"disable-passthrough\0".as_ptr() as *const _,
                disable_passthrough.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "disable-passthrough")]
    fn connect_disable_passthrough_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_disable_passthrough_trampoline<
            P: IsA<BaseParse>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstBaseParse,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&BaseParse::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::disable-passthrough\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_disable_passthrough_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
