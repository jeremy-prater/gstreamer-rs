// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{GLContext, GLDisplay};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstGLWindow")]
    pub struct GLWindow(Object<ffi::GstGLWindow, ffi::GstGLWindowClass>) @extends gst::Object;

    match fn {
        type_ => || ffi::gst_gl_window_get_type(),
    }
}

impl GLWindow {
    pub const NONE: Option<&'static GLWindow> = None;

    #[doc(alias = "gst_gl_window_new")]
    pub fn new(display: &impl IsA<GLDisplay>) -> GLWindow {
        skip_assert_initialized!();
        unsafe { from_glib_full(ffi::gst_gl_window_new(display.as_ref().to_glib_none().0)) }
    }
}

unsafe impl Send for GLWindow {}
unsafe impl Sync for GLWindow {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::GLWindow>> Sealed for T {}
}

pub trait GLWindowExt: IsA<GLWindow> + sealed::Sealed + 'static {
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_gl_window_controls_viewport")]
    fn controls_viewport(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_window_controls_viewport(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_window_draw")]
    fn draw(&self) {
        unsafe {
            ffi::gst_gl_window_draw(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_window_get_context")]
    #[doc(alias = "get_context")]
    fn context(&self) -> GLContext {
        unsafe {
            from_glib_full(ffi::gst_gl_window_get_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_window_get_surface_dimensions")]
    #[doc(alias = "get_surface_dimensions")]
    fn surface_dimensions(&self) -> (u32, u32) {
        unsafe {
            let mut width = std::mem::MaybeUninit::uninit();
            let mut height = std::mem::MaybeUninit::uninit();
            ffi::gst_gl_window_get_surface_dimensions(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    #[doc(alias = "gst_gl_window_handle_events")]
    fn handle_events(&self, handle_events: bool) {
        unsafe {
            ffi::gst_gl_window_handle_events(
                self.as_ref().to_glib_none().0,
                handle_events.into_glib(),
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_gl_window_has_output_surface")]
    fn has_output_surface(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_window_has_output_surface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_gl_window_queue_resize")]
    fn queue_resize(&self) {
        unsafe {
            ffi::gst_gl_window_queue_resize(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_window_quit")]
    fn quit(&self) {
        unsafe {
            ffi::gst_gl_window_quit(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_window_resize")]
    fn resize(&self, width: u32, height: u32) {
        unsafe {
            ffi::gst_gl_window_resize(self.as_ref().to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "gst_gl_window_run")]
    fn run(&self) {
        unsafe {
            ffi::gst_gl_window_run(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_gl_window_send_key_event")]
    fn send_key_event(&self, event_type: &str, key_str: &str) {
        unsafe {
            ffi::gst_gl_window_send_key_event(
                self.as_ref().to_glib_none().0,
                event_type.to_glib_none().0,
                key_str.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_gl_window_send_mouse_event")]
    fn send_mouse_event(&self, event_type: &str, button: i32, posx: f64, posy: f64) {
        unsafe {
            ffi::gst_gl_window_send_mouse_event(
                self.as_ref().to_glib_none().0,
                event_type.to_glib_none().0,
                button,
                posx,
                posy,
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_gl_window_send_scroll_event")]
    fn send_scroll_event(&self, posx: f64, posy: f64, delta_x: f64, delta_y: f64) {
        unsafe {
            ffi::gst_gl_window_send_scroll_event(
                self.as_ref().to_glib_none().0,
                posx,
                posy,
                delta_x,
                delta_y,
            );
        }
    }

    #[doc(alias = "gst_gl_window_set_preferred_size")]
    fn set_preferred_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gst_gl_window_set_preferred_size(self.as_ref().to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "gst_gl_window_set_render_rectangle")]
    fn set_render_rectangle(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_gl_window_set_render_rectangle(
                    self.as_ref().to_glib_none().0,
                    x,
                    y,
                    width,
                    height
                ),
                "Failed to set the specified region"
            )
        }
    }

    #[doc(alias = "gst_gl_window_show")]
    fn show(&self) {
        unsafe {
            ffi::gst_gl_window_show(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "key-event")]
    fn connect_key_event<F: Fn(&Self, &str, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn key_event_trampoline<
            P: IsA<GLWindow>,
            F: Fn(&P, &str, &str) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLWindow,
            id: *mut libc::c_char,
            key: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                GLWindow::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(id),
                &glib::GString::from_glib_borrow(key),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"key-event\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    key_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mouse-event")]
    fn connect_mouse_event<F: Fn(&Self, &str, i32, f64, f64) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn mouse_event_trampoline<
            P: IsA<GLWindow>,
            F: Fn(&P, &str, i32, f64, f64) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLWindow,
            id: *mut libc::c_char,
            button: libc::c_int,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                GLWindow::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(id),
                button,
                x,
                y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"mouse-event\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    mouse_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "scroll-event")]
    fn connect_scroll_event<F: Fn(&Self, f64, f64, f64, f64) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn scroll_event_trampoline<
            P: IsA<GLWindow>,
            F: Fn(&P, f64, f64, f64, f64) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLWindow,
            x: libc::c_double,
            y: libc::c_double,
            delta_x: libc::c_double,
            delta_y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                GLWindow::from_glib_borrow(this).unsafe_cast_ref(),
                x,
                y,
                delta_x,
                delta_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"scroll-event\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    scroll_event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "window-handle-changed")]
    fn connect_window_handle_changed<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn window_handle_changed_trampoline<
            P: IsA<GLWindow>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstGLWindow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLWindow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-handle-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    window_handle_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<GLWindow>> GLWindowExt for O {}
