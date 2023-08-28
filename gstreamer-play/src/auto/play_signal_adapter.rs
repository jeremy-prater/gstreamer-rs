// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{Play, PlayMediaInfo, PlayState};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstPlaySignalAdapter")]
    pub struct PlaySignalAdapter(Object<ffi::GstPlaySignalAdapter, ffi::GstPlaySignalAdapterClass>);

    match fn {
        type_ => || ffi::gst_play_signal_adapter_get_type(),
    }
}

impl PlaySignalAdapter {
    #[doc(alias = "gst_play_signal_adapter_new")]
    pub fn new(play: &Play) -> PlaySignalAdapter {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_play_signal_adapter_new(play.to_glib_none().0)) }
    }

    #[doc(alias = "gst_play_signal_adapter_new_sync_emit")]
    pub fn new_sync_emit(play: &Play) -> PlaySignalAdapter {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_play_signal_adapter_new_sync_emit(
                play.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_play_signal_adapter_new_with_main_context")]
    #[doc(alias = "new_with_main_context")]
    pub fn with_main_context(play: &Play, context: &glib::MainContext) -> PlaySignalAdapter {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_play_signal_adapter_new_with_main_context(
                play.to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_play_signal_adapter_get_play")]
    #[doc(alias = "get_play")]
    pub fn play(&self) -> Play {
        unsafe { from_glib_none(ffi::gst_play_signal_adapter_get_play(self.to_glib_none().0)) }
    }

    #[doc(alias = "buffering")]
    pub fn connect_buffering<F: Fn(&Self, i32) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn buffering_trampoline<
            F: Fn(&PlaySignalAdapter, i32) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"buffering\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    buffering_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "end-of-stream")]
    pub fn connect_end_of_stream<F: Fn(&Self) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn end_of_stream_trampoline<
            F: Fn(&PlaySignalAdapter) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"end-of-stream\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    end_of_stream_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "error")]
    pub fn connect_error<F: Fn(&Self, &glib::Error, Option<&gst::Structure>) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn error_trampoline<
            F: Fn(&PlaySignalAdapter, &glib::Error, Option<&gst::Structure>) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            error: *mut glib::ffi::GError,
            details: *mut gst::ffi::GstStructure,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(error),
                Option::<gst::Structure>::from_glib_borrow(details)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"error\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    error_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "media-info-updated")]
    pub fn connect_media_info_updated<F: Fn(&Self, &PlayMediaInfo) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn media_info_updated_trampoline<
            F: Fn(&PlaySignalAdapter, &PlayMediaInfo) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: *mut ffi::GstPlayMediaInfo,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"media-info-updated\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    media_info_updated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mute-changed")]
    pub fn connect_mute_changed<F: Fn(&Self, bool) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn mute_changed_trampoline<
            F: Fn(&PlaySignalAdapter, bool) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"mute-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    mute_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "state-changed")]
    pub fn connect_state_changed<F: Fn(&Self, PlayState) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn state_changed_trampoline<
            F: Fn(&PlaySignalAdapter, PlayState) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: ffi::GstPlayState,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"state-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    state_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "uri-loaded")]
    pub fn connect_uri_loaded<F: Fn(&Self, &str) + Send + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn uri_loaded_trampoline<
            F: Fn(&PlaySignalAdapter, &str) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"uri-loaded\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    uri_loaded_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "video-dimensions-changed")]
    pub fn connect_video_dimensions_changed<F: Fn(&Self, u32, u32) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn video_dimensions_changed_trampoline<
            F: Fn(&PlaySignalAdapter, u32, u32) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: libc::c_uint,
            p0: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object, p0)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"video-dimensions-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    video_dimensions_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "volume-changed")]
    pub fn connect_volume_changed<F: Fn(&Self, f64) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn volume_changed_trampoline<
            F: Fn(&PlaySignalAdapter, f64) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            object: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), object)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"volume-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    volume_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "warning")]
    pub fn connect_warning<F: Fn(&Self, &glib::Error, Option<&gst::Structure>) + Send + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn warning_trampoline<
            F: Fn(&PlaySignalAdapter, &glib::Error, Option<&gst::Structure>) + Send + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            error: *mut glib::ffi::GError,
            details: *mut gst::ffi::GstStructure,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &from_glib_borrow(error),
                Option::<gst::Structure>::from_glib_borrow(details)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"warning\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    warning_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "play")]
    pub fn connect_play_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_play_trampoline<
            F: Fn(&PlaySignalAdapter) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstPlaySignalAdapter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::play\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_play_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for PlaySignalAdapter {}
unsafe impl Sync for PlaySignalAdapter {}
