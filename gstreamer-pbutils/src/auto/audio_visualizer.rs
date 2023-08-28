// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::AudioVisualizerShader;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstAudioVisualizer")]
    pub struct AudioVisualizer(Object<ffi::GstAudioVisualizer, ffi::GstAudioVisualizerClass>) @extends gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_audio_visualizer_get_type(),
    }
}

impl AudioVisualizer {
    pub const NONE: Option<&'static AudioVisualizer> = None;
}

unsafe impl Send for AudioVisualizer {}
unsafe impl Sync for AudioVisualizer {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AudioVisualizer>> Sealed for T {}
}

pub trait AudioVisualizerExt: IsA<AudioVisualizer> + sealed::Sealed + 'static {
    #[doc(alias = "shade-amount")]
    fn shade_amount(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "shade-amount")
    }

    #[doc(alias = "shade-amount")]
    fn set_shade_amount(&self, shade_amount: u32) {
        ObjectExt::set_property(self.as_ref(), "shade-amount", shade_amount)
    }

    fn shader(&self) -> AudioVisualizerShader {
        ObjectExt::property(self.as_ref(), "shader")
    }

    fn set_shader(&self, shader: AudioVisualizerShader) {
        ObjectExt::set_property(self.as_ref(), "shader", shader)
    }

    #[doc(alias = "shade-amount")]
    fn connect_shade_amount_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_shade_amount_trampoline<
            P: IsA<AudioVisualizer>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioVisualizer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioVisualizer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shade-amount\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_shade_amount_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "shader")]
    fn connect_shader_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shader_trampoline<
            P: IsA<AudioVisualizer>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GstAudioVisualizer,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AudioVisualizer::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shader\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_shader_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<AudioVisualizer>> AudioVisualizerExt for O {}
