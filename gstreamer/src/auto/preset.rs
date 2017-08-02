// This file was generated by gir (ef05cf1) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Preset(Object<ffi::GstPreset>);

    match fn {
        get_type => || ffi::gst_preset_get_type(),
    }
}

impl Preset {
    pub fn get_app_dir() -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_preset_get_app_dir())
        }
    }

    pub fn set_app_dir(app_dir: &str) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gst_preset_set_app_dir(app_dir.to_glib_none().0))
        }
    }
}

unsafe impl Send for Preset {}
unsafe impl Sync for Preset {}

pub trait PresetExt {
    fn delete_preset(&self, name: &str) -> bool;

    fn get_meta(&self, name: &str, tag: &str) -> Option<String>;

    fn get_preset_names(&self) -> Vec<String>;

    fn get_property_names(&self) -> Vec<String>;

    fn is_editable(&self) -> bool;

    fn load_preset(&self, name: &str) -> bool;

    fn rename_preset(&self, old_name: &str, new_name: &str) -> bool;

    fn save_preset(&self, name: &str) -> bool;

    fn set_meta<'a, P: Into<Option<&'a str>>>(&self, name: &str, tag: &str, value: P) -> bool;
}

impl<O: IsA<Preset>> PresetExt for O {
    fn delete_preset(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_preset_delete_preset(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn get_meta(&self, name: &str, tag: &str) -> Option<String> {
        unsafe {
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::gst_preset_get_meta(self.to_glib_none().0, name.to_glib_none().0, tag.to_glib_none().0, &mut value));
            if ret { Some(from_glib_full(value)) } else { None }
        }
    }

    fn get_preset_names(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_preset_get_preset_names(self.to_glib_none().0))
        }
    }

    fn get_property_names(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_preset_get_property_names(self.to_glib_none().0))
        }
    }

    fn is_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_preset_is_editable(self.to_glib_none().0))
        }
    }

    fn load_preset(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_preset_load_preset(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn rename_preset(&self, old_name: &str, new_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_preset_rename_preset(self.to_glib_none().0, old_name.to_glib_none().0, new_name.to_glib_none().0))
        }
    }

    fn save_preset(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_preset_save_preset(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn set_meta<'a, P: Into<Option<&'a str>>>(&self, name: &str, tag: &str, value: P) -> bool {
        let value = value.into();
        let value = value.to_glib_none();
        unsafe {
            from_glib(ffi::gst_preset_set_meta(self.to_glib_none().0, name.to_glib_none().0, tag.to_glib_none().0, value.0))
        }
    }
}
