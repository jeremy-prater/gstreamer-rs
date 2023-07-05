// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{RTSPMedia, RTSPStreamTransport};
use glib::{prelude::*, translate::*};
use std::mem;

glib::wrapper! {
    #[doc(alias = "GstRTSPSessionMedia")]
    pub struct RTSPSessionMedia(Object<ffi::GstRTSPSessionMedia, ffi::GstRTSPSessionMediaClass>);

    match fn {
        type_ => || ffi::gst_rtsp_session_media_get_type(),
    }
}

impl RTSPSessionMedia {
    pub const NONE: Option<&'static RTSPSessionMedia> = None;

    #[doc(alias = "gst_rtsp_session_media_new")]
    pub fn new(path: &str, media: impl IsA<RTSPMedia>) -> RTSPSessionMedia {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_media_new(
                path.to_glib_none().0,
                media.upcast().into_glib_ptr(),
            ))
        }
    }
}

unsafe impl Send for RTSPSessionMedia {}
unsafe impl Sync for RTSPSessionMedia {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPSessionMedia>> Sealed for T {}
}

pub trait RTSPSessionMediaExt: IsA<RTSPSessionMedia> + sealed::Sealed + 'static {
    //#[doc(alias = "gst_rtsp_session_media_alloc_channels")]
    //fn alloc_channels(&self, range: /*Ignored*/gst_rtsp::RTSPRange) -> bool {
    //    unsafe { TODO: call ffi:gst_rtsp_session_media_alloc_channels() }
    //}

    #[doc(alias = "gst_rtsp_session_media_get_base_time")]
    #[doc(alias = "get_base_time")]
    fn base_time(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::gst_rtsp_session_media_get_base_time(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_session_media_get_media")]
    #[doc(alias = "get_media")]
    fn media(&self) -> Option<RTSPMedia> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_session_media_get_media(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_session_media_get_rtpinfo")]
    #[doc(alias = "get_rtpinfo")]
    fn rtpinfo(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_media_get_rtpinfo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "gst_rtsp_session_media_get_rtsp_state")]
    //#[doc(alias = "get_rtsp_state")]
    //fn rtsp_state(&self) -> /*Ignored*/gst_rtsp::RTSPState {
    //    unsafe { TODO: call ffi:gst_rtsp_session_media_get_rtsp_state() }
    //}

    #[doc(alias = "gst_rtsp_session_media_get_transport")]
    #[doc(alias = "get_transport")]
    fn transport(&self, idx: u32) -> Option<RTSPStreamTransport> {
        unsafe {
            from_glib_none(ffi::gst_rtsp_session_media_get_transport(
                self.as_ref().to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_session_media_get_transports")]
    #[doc(alias = "get_transports")]
    fn transports(&self) -> Vec<RTSPStreamTransport> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_rtsp_session_media_get_transports(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_session_media_matches")]
    fn matches(&self, path: &str) -> Option<i32> {
        unsafe {
            let mut matched = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_rtsp_session_media_matches(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                matched.as_mut_ptr(),
            ));
            if ret {
                Some(matched.assume_init())
            } else {
                None
            }
        }
    }

    //#[doc(alias = "gst_rtsp_session_media_set_rtsp_state")]
    //fn set_rtsp_state(&self, state: /*Ignored*/gst_rtsp::RTSPState) {
    //    unsafe { TODO: call ffi:gst_rtsp_session_media_set_rtsp_state() }
    //}

    #[doc(alias = "gst_rtsp_session_media_set_state")]
    fn set_state(&self, state: gst::State) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_session_media_set_state(
                    self.as_ref().to_glib_none().0,
                    state.into_glib()
                ),
                "Failed to set state of session media"
            )
        }
    }

    //#[doc(alias = "gst_rtsp_session_media_set_transport")]
    //fn set_transport(&self, stream: &impl IsA<RTSPStream>, tr: /*Ignored*/gst_rtsp::RTSPTransport) -> RTSPStreamTransport {
    //    unsafe { TODO: call ffi:gst_rtsp_session_media_set_transport() }
    //}
}

impl<O: IsA<RTSPSessionMedia>> RTSPSessionMediaExt for O {}
