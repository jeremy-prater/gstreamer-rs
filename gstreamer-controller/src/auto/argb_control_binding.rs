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
    #[doc(alias = "GstARGBControlBinding")]
    pub struct ARGBControlBinding(Object<ffi::GstARGBControlBinding, ffi::GstARGBControlBindingClass>) @extends gst::ControlBinding, gst::Object;

    match fn {
        type_ => || ffi::gst_argb_control_binding_get_type(),
    }
}

impl ARGBControlBinding {
    pub const NONE: Option<&'static ARGBControlBinding> = None;

    #[doc(alias = "gst_argb_control_binding_new")]
    pub fn new(
        object: &impl IsA<gst::Object>,
        property_name: &str,
        cs_a: &impl IsA<gst::ControlSource>,
        cs_r: &impl IsA<gst::ControlSource>,
        cs_g: &impl IsA<gst::ControlSource>,
        cs_b: &impl IsA<gst::ControlSource>,
    ) -> ARGBControlBinding {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlBinding::from_glib_none(ffi::gst_argb_control_binding_new(
                object.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                cs_a.as_ref().to_glib_none().0,
                cs_r.as_ref().to_glib_none().0,
                cs_g.as_ref().to_glib_none().0,
                cs_b.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

unsafe impl Send for ARGBControlBinding {}
unsafe impl Sync for ARGBControlBinding {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ARGBControlBinding>> Sealed for T {}
}

pub trait ARGBControlBindingExt: IsA<ARGBControlBinding> + sealed::Sealed + 'static {
    #[doc(alias = "control-source-a")]
    fn control_source_a(&self) -> Option<gst::ControlSource> {
        ObjectExt::property(self.as_ref(), "control-source-a")
    }

    #[doc(alias = "control-source-a")]
    fn set_control_source_a<P: IsA<gst::ControlSource>>(&self, control_source_a: Option<&P>) {
        ObjectExt::set_property(self.as_ref(), "control-source-a", control_source_a)
    }

    #[doc(alias = "control-source-b")]
    fn control_source_b(&self) -> Option<gst::ControlSource> {
        ObjectExt::property(self.as_ref(), "control-source-b")
    }

    #[doc(alias = "control-source-b")]
    fn set_control_source_b<P: IsA<gst::ControlSource>>(&self, control_source_b: Option<&P>) {
        ObjectExt::set_property(self.as_ref(), "control-source-b", control_source_b)
    }

    #[doc(alias = "control-source-g")]
    fn control_source_g(&self) -> Option<gst::ControlSource> {
        ObjectExt::property(self.as_ref(), "control-source-g")
    }

    #[doc(alias = "control-source-g")]
    fn set_control_source_g<P: IsA<gst::ControlSource>>(&self, control_source_g: Option<&P>) {
        ObjectExt::set_property(self.as_ref(), "control-source-g", control_source_g)
    }

    #[doc(alias = "control-source-r")]
    fn control_source_r(&self) -> Option<gst::ControlSource> {
        ObjectExt::property(self.as_ref(), "control-source-r")
    }

    #[doc(alias = "control-source-r")]
    fn set_control_source_r<P: IsA<gst::ControlSource>>(&self, control_source_r: Option<&P>) {
        ObjectExt::set_property(self.as_ref(), "control-source-r", control_source_r)
    }

    #[doc(alias = "control-source-a")]
    fn connect_control_source_a_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_a_trampoline<
            P: IsA<ARGBControlBinding>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstARGBControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ARGBControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source-a\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_control_source_a_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "control-source-b")]
    fn connect_control_source_b_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_b_trampoline<
            P: IsA<ARGBControlBinding>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstARGBControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ARGBControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source-b\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_control_source_b_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "control-source-g")]
    fn connect_control_source_g_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_g_trampoline<
            P: IsA<ARGBControlBinding>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstARGBControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ARGBControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source-g\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_control_source_g_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "control-source-r")]
    fn connect_control_source_r_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_control_source_r_trampoline<
            P: IsA<ARGBControlBinding>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstARGBControlBinding,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ARGBControlBinding::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::control-source-r\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_control_source_r_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ARGBControlBinding>> ARGBControlBindingExt for O {}
