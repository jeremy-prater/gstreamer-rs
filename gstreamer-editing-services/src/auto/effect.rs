// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::{BaseEffect, Extractable, MetaContainer, Operation, TimelineElement, TrackElement};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GESEffect")]
    pub struct Effect(Object<ffi::GESEffect, ffi::GESEffectClass>) @extends BaseEffect, Operation, TrackElement, TimelineElement, @implements Extractable, MetaContainer;

    match fn {
        type_ => || ffi::ges_effect_get_type(),
    }
}

impl Effect {
    pub const NONE: Option<&'static Effect> = None;

    #[doc(alias = "ges_effect_new")]
    pub fn new(bin_description: &str) -> Result<Effect, glib::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            Option::<_>::from_glib_none(ffi::ges_effect_new(bin_description.to_glib_none().0))
                .ok_or_else(|| glib::bool_error!("Failed to create effect from description"))
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Effect>> Sealed for T {}
}

pub trait EffectExt: IsA<Effect> + sealed::Sealed + 'static {
    #[doc(alias = "bin-description")]
    fn bin_description(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "bin-description")
    }
}

impl<O: IsA<Effect>> EffectExt for O {}
