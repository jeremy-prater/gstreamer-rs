// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{GLDisplay, GLPlatform, GLSLProfile, GLSLVersion, GLWindow, GLAPI};
use glib::{prelude::*, translate::*};
use std::{mem, ptr};

glib::wrapper! {
    #[doc(alias = "GstGLContext")]
    pub struct GLContext(Object<ffi::GstGLContext, ffi::GstGLContextClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_gl_context_get_type(),
    }
}

impl GLContext {
    pub const NONE: Option<&'static GLContext> = None;

    #[doc(alias = "gst_gl_context_new")]
    pub fn new(display: &impl IsA<GLDisplay>) -> GLContext {
        skip_assert_initialized!();
        unsafe { from_glib_none(ffi::gst_gl_context_new(display.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_gl_context_get_current")]
    #[doc(alias = "get_current")]
    pub fn current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_gl_context_get_current()) }
    }

    #[doc(alias = "gst_gl_context_get_current_gl_api")]
    #[doc(alias = "get_current_gl_api")]
    pub fn current_gl_api(platform: GLPlatform) -> (GLAPI, u32, u32) {
        assert_initialized_main_thread!();
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gst_gl_context_get_current_gl_api(
                platform.into_glib(),
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            ));
            (ret, major.assume_init(), minor.assume_init())
        }
    }
}

unsafe impl Send for GLContext {}
unsafe impl Sync for GLContext {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::GLContext>> Sealed for T {}
}

pub trait GLContextExt: IsA<GLContext> + sealed::Sealed + 'static {
    #[doc(alias = "gst_gl_context_activate")]
    fn activate(&self, activate: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_context_activate(self.as_ref().to_glib_none().0, activate.into_glib()),
                "Failed to activate OpenGL context"
            )
        }
    }

    #[doc(alias = "gst_gl_context_can_share")]
    fn can_share(&self, other_context: &impl IsA<GLContext>) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_can_share(
                self.as_ref().to_glib_none().0,
                other_context.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_context_check_feature")]
    fn check_feature(&self, feature: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_check_feature(
                self.as_ref().to_glib_none().0,
                feature.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_context_check_framebuffer_status")]
    fn check_framebuffer_status(&self, fbo_target: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_check_framebuffer_status(
                self.as_ref().to_glib_none().0,
                fbo_target,
            ))
        }
    }

    #[doc(alias = "gst_gl_context_check_gl_version")]
    fn check_gl_version(&self, api: GLAPI, maj: i32, min: i32) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_check_gl_version(
                self.as_ref().to_glib_none().0,
                api.into_glib(),
                maj,
                min,
            ))
        }
    }

    #[doc(alias = "gst_gl_context_clear_framebuffer")]
    fn clear_framebuffer(&self) {
        unsafe {
            ffi::gst_gl_context_clear_framebuffer(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_context_clear_shader")]
    fn clear_shader(&self) {
        unsafe {
            ffi::gst_gl_context_clear_shader(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_context_create")]
    fn create(&self, other_context: Option<&impl IsA<GLContext>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gst_gl_context_create(
                self.as_ref().to_glib_none().0,
                other_context.map(|p| p.as_ref()).to_glib_none().0,
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

    #[doc(alias = "gst_gl_context_destroy")]
    fn destroy(&self) {
        unsafe {
            ffi::gst_gl_context_destroy(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_context_fill_info")]
    fn fill_info(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gst_gl_context_fill_info(self.as_ref().to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_gl_context_get_config")]
    #[doc(alias = "get_config")]
    fn config(&self) -> Option<gst::Structure> {
        unsafe {
            from_glib_full(ffi::gst_gl_context_get_config(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_context_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> GLDisplay {
        unsafe {
            from_glib_full(ffi::gst_gl_context_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_context_get_gl_api")]
    #[doc(alias = "get_gl_api")]
    fn gl_api(&self) -> GLAPI {
        unsafe {
            from_glib(ffi::gst_gl_context_get_gl_api(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_context_get_gl_platform")]
    #[doc(alias = "get_gl_platform")]
    fn gl_platform(&self) -> GLPlatform {
        unsafe {
            from_glib(ffi::gst_gl_context_get_gl_platform(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_context_get_gl_platform_version")]
    #[doc(alias = "get_gl_platform_version")]
    fn gl_platform_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            ffi::gst_gl_context_get_gl_platform_version(
                self.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            (major.assume_init(), minor.assume_init())
        }
    }

    #[doc(alias = "gst_gl_context_get_gl_version")]
    #[doc(alias = "get_gl_version")]
    fn gl_version(&self) -> (i32, i32) {
        unsafe {
            let mut maj = mem::MaybeUninit::uninit();
            let mut min = mem::MaybeUninit::uninit();
            ffi::gst_gl_context_get_gl_version(
                self.as_ref().to_glib_none().0,
                maj.as_mut_ptr(),
                min.as_mut_ptr(),
            );
            (maj.assume_init(), min.assume_init())
        }
    }

    #[doc(alias = "gst_gl_context_get_window")]
    #[doc(alias = "get_window")]
    fn window(&self) -> Option<GLWindow> {
        unsafe {
            from_glib_full(ffi::gst_gl_context_get_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_context_is_shared")]
    fn is_shared(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_is_shared(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_gl_context_request_config")]
    fn request_config(&self, gl_config: Option<gst::Structure>) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_request_config(
                self.as_ref().to_glib_none().0,
                gl_config.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "gst_gl_context_set_shared_with")]
    fn set_shared_with(&self, share: &impl IsA<GLContext>) {
        unsafe {
            ffi::gst_gl_context_set_shared_with(
                self.as_ref().to_glib_none().0,
                share.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_context_set_window")]
    fn set_window(&self, window: impl IsA<GLWindow>) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_context_set_window(
                    self.as_ref().to_glib_none().0,
                    window.upcast().into_glib_ptr()
                ),
                "Failed to set window"
            )
        }
    }

    #[doc(alias = "gst_gl_context_supports_glsl_profile_version")]
    fn supports_glsl_profile_version(&self, version: GLSLVersion, profile: GLSLProfile) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_supports_glsl_profile_version(
                self.as_ref().to_glib_none().0,
                version.into_glib(),
                profile.into_glib(),
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_context_supports_precision")]
    fn supports_precision(&self, version: GLSLVersion, profile: GLSLProfile) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_supports_precision(
                self.as_ref().to_glib_none().0,
                version.into_glib(),
                profile.into_glib(),
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_context_supports_precision_highp")]
    fn supports_precision_highp(&self, version: GLSLVersion, profile: GLSLProfile) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_supports_precision_highp(
                self.as_ref().to_glib_none().0,
                version.into_glib(),
                profile.into_glib(),
            ))
        }
    }

    #[doc(alias = "gst_gl_context_swap_buffers")]
    fn swap_buffers(&self) {
        unsafe {
            ffi::gst_gl_context_swap_buffers(self.as_ref().to_glib_none().0);
        }
    }
}

impl<O: IsA<GLContext>> GLContextExt for O {}
