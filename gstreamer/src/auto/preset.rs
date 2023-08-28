// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstPreset")]
    pub struct Preset(Interface<ffi::GstPreset, ffi::GstPresetInterface>);

    match fn {
        type_ => || ffi::gst_preset_get_type(),
    }
}

impl Preset {
    pub const NONE: Option<&'static Preset> = None;

    #[doc(alias = "gst_preset_get_app_dir")]
    #[doc(alias = "get_app_dir")]
    pub fn app_dir() -> Option<std::path::PathBuf> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gst_preset_get_app_dir()) }
    }

    #[doc(alias = "gst_preset_set_app_dir")]
    pub fn set_app_dir(app_dir: impl AsRef<std::path::Path>) -> Result<(), glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_preset_set_app_dir(app_dir.as_ref().to_glib_none().0),
                "Failed to set app preset directory"
            )
        }
    }
}

unsafe impl Send for Preset {}
unsafe impl Sync for Preset {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Preset>> Sealed for T {}
}

pub trait PresetExt: IsA<Preset> + sealed::Sealed + 'static {
    #[doc(alias = "gst_preset_delete_preset")]
    fn delete_preset(&self, name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_preset_delete_preset(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0
                ),
                "Failed to delete preset"
            )
        }
    }

    #[doc(alias = "gst_preset_get_meta")]
    #[doc(alias = "get_meta")]
    fn meta(&self, name: &str, tag: &str) -> Option<glib::GString> {
        unsafe {
            let mut value = std::ptr::null_mut();
            let ret = from_glib(ffi::gst_preset_get_meta(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                tag.to_glib_none().0,
                &mut value,
            ));
            if ret {
                Some(from_glib_full(value))
            } else {
                None
            }
        }
    }

    #[doc(alias = "gst_preset_get_preset_names")]
    #[doc(alias = "get_preset_names")]
    fn preset_names(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_preset_get_preset_names(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_preset_get_property_names")]
    #[doc(alias = "get_property_names")]
    fn property_names(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gst_preset_get_property_names(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_preset_is_editable")]
    fn is_editable(&self) -> bool {
        unsafe { from_glib(ffi::gst_preset_is_editable(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_preset_load_preset")]
    fn load_preset(&self, name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_preset_load_preset(self.as_ref().to_glib_none().0, name.to_glib_none().0),
                "Failed to load preset"
            )
        }
    }

    #[doc(alias = "gst_preset_rename_preset")]
    fn rename_preset(&self, old_name: &str, new_name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_preset_rename_preset(
                    self.as_ref().to_glib_none().0,
                    old_name.to_glib_none().0,
                    new_name.to_glib_none().0
                ),
                "Failed to rename preset"
            )
        }
    }

    #[doc(alias = "gst_preset_save_preset")]
    fn save_preset(&self, name: &str) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_preset_save_preset(self.as_ref().to_glib_none().0, name.to_glib_none().0),
                "Failed to save preset"
            )
        }
    }

    #[doc(alias = "gst_preset_set_meta")]
    fn set_meta(
        &self,
        name: &str,
        tag: &str,
        value: Option<&str>,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_preset_set_meta(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0,
                    tag.to_glib_none().0,
                    value.to_glib_none().0
                ),
                "Failed to set preset meta"
            )
        }
    }
}

impl<O: IsA<Preset>> PresetExt for O {}
