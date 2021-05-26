// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::BaseTransitionClip;
use crate::Clip;
use crate::Container;
use crate::Extractable;
use crate::OperationClip;
use crate::TimelineElement;
use crate::VideoStandardTransitionType;
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
    pub struct TransitionClip(Object<ffi::GESTransitionClip, ffi::GESTransitionClipClass>) @extends BaseTransitionClip, OperationClip, Clip, Container, TimelineElement, @implements Extractable;

    match fn {
        type_ => || ffi::ges_transition_clip_get_type(),
    }
}

impl TransitionClip {
    #[doc(alias = "ges_transition_clip_new")]
    pub fn new(vtype: VideoStandardTransitionType) -> Option<TransitionClip> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_transition_clip_new(vtype.into_glib())) }
    }

    #[doc(alias = "ges_transition_clip_new_for_nick")]
    #[doc(alias = "new_for_nick")]
    pub fn for_nick(nick: &str) -> Option<TransitionClip> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::ges_transition_clip_new_for_nick(nick.to_glib_none().0)) }
    }
}

pub const NONE_TRANSITION_CLIP: Option<&TransitionClip> = None;

pub trait TransitionClipExt: 'static {
    fn vtype(&self) -> VideoStandardTransitionType;

    fn set_vtype(&self, vtype: VideoStandardTransitionType);

    #[doc(alias = "vtype")]
    fn connect_vtype_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TransitionClip>> TransitionClipExt for O {
    fn vtype(&self) -> VideoStandardTransitionType {
        unsafe {
            let mut value =
                glib::Value::from_type(<VideoStandardTransitionType as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"vtype\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `vtype` getter")
        }
    }

    fn set_vtype(&self, vtype: VideoStandardTransitionType) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"vtype\0".as_ptr() as *const _,
                vtype.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "vtype")]
    fn connect_vtype_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vtype_trampoline<
            P: IsA<TransitionClip>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GESTransitionClip,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&TransitionClip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::vtype\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vtype_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
