// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{RTSPFilterResult, RTSPSession};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstRTSPSessionPool")]
    pub struct RTSPSessionPool(Object<ffi::GstRTSPSessionPool, ffi::GstRTSPSessionPoolClass>);

    match fn {
        type_ => || ffi::gst_rtsp_session_pool_get_type(),
    }
}

impl RTSPSessionPool {
    pub const NONE: Option<&'static RTSPSessionPool> = None;

    #[doc(alias = "gst_rtsp_session_pool_new")]
    pub fn new() -> RTSPSessionPool {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_session_pool_new()) }
    }
}

impl Default for RTSPSessionPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPSessionPool {}
unsafe impl Sync for RTSPSessionPool {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPSessionPool>> Sealed for T {}
}

pub trait RTSPSessionPoolExt: IsA<RTSPSessionPool> + sealed::Sealed + 'static {
    #[doc(alias = "gst_rtsp_session_pool_cleanup")]
    fn cleanup(&self) -> u32 {
        unsafe { ffi::gst_rtsp_session_pool_cleanup(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_session_pool_create")]
    fn create(&self) -> Result<RTSPSession, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_full(ffi::gst_rtsp_session_pool_create(
                self.as_ref().to_glib_none().0,
            ))
            .ok_or_else(|| glib::bool_error!("Failed to create session pool"))
        }
    }

    #[doc(alias = "gst_rtsp_session_pool_filter")]
    fn filter(
        &self,
        func: Option<&mut dyn (FnMut(&RTSPSessionPool, &RTSPSession) -> RTSPFilterResult)>,
    ) -> Vec<RTSPSession> {
        let func_data: Option<
            &mut dyn (FnMut(&RTSPSessionPool, &RTSPSession) -> RTSPFilterResult),
        > = func;
        unsafe extern "C" fn func_func(
            pool: *mut ffi::GstRTSPSessionPool,
            session: *mut ffi::GstRTSPSession,
            user_data: glib::ffi::gpointer,
        ) -> ffi::GstRTSPFilterResult {
            let pool = from_glib_borrow(pool);
            let session = from_glib_borrow(session);
            let callback: *mut Option<
                &mut dyn (FnMut(&RTSPSessionPool, &RTSPSession) -> RTSPFilterResult),
            > = user_data as *const _ as usize
                as *mut Option<
                    &mut dyn (FnMut(&RTSPSessionPool, &RTSPSession) -> RTSPFilterResult),
                >;
            if let Some(ref mut callback) = *callback {
                callback(&pool, &session)
            } else {
                panic!("cannot get closure...")
            }
            .into_glib()
        }
        let func = if func_data.is_some() {
            Some(func_func as _)
        } else {
            None
        };
        let super_callback0: &Option<
            &mut dyn (FnMut(&RTSPSessionPool, &RTSPSession) -> RTSPFilterResult),
        > = &func_data;
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_rtsp_session_pool_filter(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_session_pool_find")]
    fn find(&self, sessionid: &str) -> Option<RTSPSession> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_session_pool_find(
                self.as_ref().to_glib_none().0,
                sessionid.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_session_pool_get_max_sessions")]
    #[doc(alias = "get_max_sessions")]
    fn max_sessions(&self) -> u32 {
        unsafe { ffi::gst_rtsp_session_pool_get_max_sessions(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_session_pool_get_n_sessions")]
    #[doc(alias = "get_n_sessions")]
    fn n_sessions(&self) -> u32 {
        unsafe { ffi::gst_rtsp_session_pool_get_n_sessions(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gst_rtsp_session_pool_remove")]
    fn remove(&self, sess: &impl IsA<RTSPSession>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_session_pool_remove(
                    self.as_ref().to_glib_none().0,
                    sess.as_ref().to_glib_none().0
                ),
                "Failed to remove session from pool"
            )
        }
    }

    #[doc(alias = "gst_rtsp_session_pool_set_max_sessions")]
    fn set_max_sessions(&self, max: u32) {
        unsafe {
            ffi::gst_rtsp_session_pool_set_max_sessions(self.as_ref().to_glib_none().0, max);
        }
    }

    #[doc(alias = "session-removed")]
    fn connect_session_removed<F: Fn(&Self, &RTSPSession) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn session_removed_trampoline<
            P: IsA<RTSPSessionPool>,
            F: Fn(&P, &RTSPSession) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPSessionPool,
            object: *mut ffi::GstRTSPSession,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                RTSPSessionPool::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"session-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    session_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-sessions")]
    fn connect_max_sessions_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_sessions_trampoline<
            P: IsA<RTSPSessionPool>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstRTSPSessionPool,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RTSPSessionPool::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-sessions\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_sessions_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<RTSPSessionPool>> RTSPSessionPoolExt for O {}
