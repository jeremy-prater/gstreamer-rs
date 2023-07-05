// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::NavigationCommand;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstNavigation")]
    pub struct Navigation(Interface<ffi::GstNavigation, ffi::GstNavigationInterface>);

    match fn {
        type_ => || ffi::gst_navigation_get_type(),
    }
}

impl Navigation {
    pub const NONE: Option<&'static Navigation> = None;

    //#[doc(alias = "gst_navigation_query_set_commands")]
    //pub fn query_set_commands(query: &gst::Query, n_cmds: i32, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gst_navigation_query_set_commands() }
    //}

    //#[doc(alias = "gst_navigation_query_set_commandsv")]
    //pub fn query_set_commandsv(query: &gst::Query, cmds: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 6 }) {
    //    unsafe { TODO: call ffi:gst_navigation_query_set_commandsv() }
    //}
}

unsafe impl Send for Navigation {}
unsafe impl Sync for Navigation {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Navigation>> Sealed for T {}
}

pub trait NavigationExt: IsA<Navigation> + sealed::Sealed + 'static {
    #[doc(alias = "gst_navigation_send_command")]
    fn send_command(&self, command: NavigationCommand) {
        unsafe {
            ffi::gst_navigation_send_command(self.as_ref().to_glib_none().0, command.into_glib());
        }
    }

    #[doc(alias = "gst_navigation_send_event")]
    fn send_event(&self, structure: gst::Structure) {
        unsafe {
            ffi::gst_navigation_send_event(
                self.as_ref().to_glib_none().0,
                structure.into_glib_ptr(),
            );
        }
    }

    #[cfg(feature = "v1_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[doc(alias = "gst_navigation_send_event_simple")]
    fn send_event_simple(&self, event: gst::Event) {
        unsafe {
            ffi::gst_navigation_send_event_simple(
                self.as_ref().to_glib_none().0,
                event.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "gst_navigation_send_key_event")]
    fn send_key_event(&self, event: &str, key: &str) {
        unsafe {
            ffi::gst_navigation_send_key_event(
                self.as_ref().to_glib_none().0,
                event.to_glib_none().0,
                key.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_navigation_send_mouse_event")]
    fn send_mouse_event(&self, event: &str, button: i32, x: f64, y: f64) {
        unsafe {
            ffi::gst_navigation_send_mouse_event(
                self.as_ref().to_glib_none().0,
                event.to_glib_none().0,
                button,
                x,
                y,
            );
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "gst_navigation_send_mouse_scroll_event")]
    fn send_mouse_scroll_event(&self, x: f64, y: f64, delta_x: f64, delta_y: f64) {
        unsafe {
            ffi::gst_navigation_send_mouse_scroll_event(
                self.as_ref().to_glib_none().0,
                x,
                y,
                delta_x,
                delta_y,
            );
        }
    }
}

impl<O: IsA<Navigation>> NavigationExt for O {}
