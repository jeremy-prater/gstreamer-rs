// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GstChildProxy")]
    pub struct ChildProxy(Interface<ffi::GstChildProxy, ffi::GstChildProxyInterface>);

    match fn {
        type_ => || ffi::gst_child_proxy_get_type(),
    }
}

impl ChildProxy {
    pub const NONE: Option<&'static ChildProxy> = None;
}

unsafe impl Send for ChildProxy {}
unsafe impl Sync for ChildProxy {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ChildProxy>> Sealed for T {}
}

pub trait ChildProxyExt: IsA<ChildProxy> + sealed::Sealed + 'static {
    #[doc(alias = "gst_child_proxy_child_added")]
    fn child_added(&self, child: &impl IsA<glib::Object>, name: &str) {
        unsafe {
            ffi::gst_child_proxy_child_added(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_child_proxy_child_removed")]
    fn child_removed(&self, child: &impl IsA<glib::Object>, name: &str) {
        unsafe {
            ffi::gst_child_proxy_child_removed(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "gst_child_proxy_get")]
    //fn get(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gst_child_proxy_get() }
    //}

    #[doc(alias = "gst_child_proxy_get_child_by_index")]
    #[doc(alias = "get_child_by_index")]
    fn child_by_index(&self, index: u32) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gst_child_proxy_get_child_by_index(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    #[doc(alias = "gst_child_proxy_get_child_by_name")]
    #[doc(alias = "get_child_by_name")]
    fn child_by_name(&self, name: &str) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gst_child_proxy_get_child_by_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_child_proxy_get_child_by_name_recurse")]
    #[doc(alias = "get_child_by_name_recurse")]
    fn child_by_name_recurse(&self, name: &str) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::gst_child_proxy_get_child_by_name_recurse(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_child_proxy_get_children_count")]
    #[doc(alias = "get_children_count")]
    fn children_count(&self) -> u32 {
        unsafe { ffi::gst_child_proxy_get_children_count(self.as_ref().to_glib_none().0) }
    }

    //#[doc(alias = "gst_child_proxy_get_valist")]
    //#[doc(alias = "get_valist")]
    //fn valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gst_child_proxy_get_valist() }
    //}

    //#[doc(alias = "gst_child_proxy_set")]
    //fn set(&self, first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gst_child_proxy_set() }
    //}

    //#[doc(alias = "gst_child_proxy_set_valist")]
    //fn set_valist(&self, first_property_name: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gst_child_proxy_set_valist() }
    //}

    #[doc(alias = "child-added")]
    fn connect_child_added<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_added_trampoline<
            P: IsA<ChildProxy>,
            F: Fn(&P, &glib::Object, &str) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstChildProxy,
            object: *mut glib::gobject_ffi::GObject,
            name: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                ChildProxy::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
                &glib::GString::from_glib_borrow(name),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "child-removed")]
    fn connect_child_removed<F: Fn(&Self, &glib::Object, &str) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn child_removed_trampoline<
            P: IsA<ChildProxy>,
            F: Fn(&P, &glib::Object, &str) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstChildProxy,
            object: *mut glib::gobject_ffi::GObject,
            name: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                ChildProxy::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
                &glib::GString::from_glib_borrow(name),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"child-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    child_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ChildProxy>> ChildProxyExt for O {}
