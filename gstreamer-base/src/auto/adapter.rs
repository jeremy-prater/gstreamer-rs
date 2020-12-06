// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::ObjectExt;
use std::mem;

glib::glib_wrapper! {
    pub struct Adapter(Object<ffi::GstAdapter, ffi::GstAdapterClass>);

    match fn {
        get_type => || ffi::gst_adapter_get_type(),
    }
}

impl Adapter {
    pub fn new() -> Adapter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_adapter_new()) }
    }

    pub fn available(&self) -> usize {
        unsafe { ffi::gst_adapter_available(self.to_glib_none().0) }
    }

    pub fn available_fast(&self) -> usize {
        unsafe { ffi::gst_adapter_available_fast(self.to_glib_none().0) }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gst_adapter_clear(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn distance_from_discont(&self) -> u64 {
        unsafe { ffi::gst_adapter_distance_from_discont(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn dts_at_discont(&self) -> gst::ClockTime {
        unsafe { from_glib(ffi::gst_adapter_dts_at_discont(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn offset_at_discont(&self) -> u64 {
        unsafe { ffi::gst_adapter_offset_at_discont(self.to_glib_none().0) }
    }

    pub fn prev_dts(&self) -> (gst::ClockTime, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_adapter_prev_dts(
                self.to_glib_none().0,
                distance.as_mut_ptr(),
            ));
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    pub fn prev_dts_at_offset(&self, offset: usize) -> (gst::ClockTime, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_adapter_prev_dts_at_offset(
                self.to_glib_none().0,
                offset,
                distance.as_mut_ptr(),
            ));
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn prev_offset(&self) -> (u64, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = ffi::gst_adapter_prev_offset(self.to_glib_none().0, distance.as_mut_ptr());
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    pub fn prev_pts(&self) -> (gst::ClockTime, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_adapter_prev_pts(
                self.to_glib_none().0,
                distance.as_mut_ptr(),
            ));
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    pub fn prev_pts_at_offset(&self, offset: usize) -> (gst::ClockTime, u64) {
        unsafe {
            let mut distance = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_adapter_prev_pts_at_offset(
                self.to_glib_none().0,
                offset,
                distance.as_mut_ptr(),
            ));
            let distance = distance.assume_init();
            (ret, distance)
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn pts_at_discont(&self) -> gst::ClockTime {
        unsafe { from_glib(ffi::gst_adapter_pts_at_discont(self.to_glib_none().0)) }
    }
}

impl Default for Adapter {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl glib::SendUnique for Adapter {
    fn is_unique(&self) -> bool {
        self.ref_count() == 1
    }
}
