// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{translate::*, value::FromValue, value::ToValue, StaticType, Type};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstInterpolationMode")]
pub enum InterpolationMode {
    #[doc(alias = "GST_INTERPOLATION_MODE_NONE")]
    None,
    #[doc(alias = "GST_INTERPOLATION_MODE_LINEAR")]
    Linear,
    #[doc(alias = "GST_INTERPOLATION_MODE_CUBIC")]
    Cubic,
    #[doc(alias = "GST_INTERPOLATION_MODE_CUBIC_MONOTONIC")]
    CubicMonotonic,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for InterpolationMode {
    type GlibType = ffi::GstInterpolationMode;

    #[inline]
    fn into_glib(self) -> ffi::GstInterpolationMode {
        match self {
            Self::None => ffi::GST_INTERPOLATION_MODE_NONE,
            Self::Linear => ffi::GST_INTERPOLATION_MODE_LINEAR,
            Self::Cubic => ffi::GST_INTERPOLATION_MODE_CUBIC,
            Self::CubicMonotonic => ffi::GST_INTERPOLATION_MODE_CUBIC_MONOTONIC,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstInterpolationMode> for InterpolationMode {
    #[inline]
    unsafe fn from_glib(value: ffi::GstInterpolationMode) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::GST_INTERPOLATION_MODE_NONE => Self::None,
            ffi::GST_INTERPOLATION_MODE_LINEAR => Self::Linear,
            ffi::GST_INTERPOLATION_MODE_CUBIC => Self::Cubic,
            ffi::GST_INTERPOLATION_MODE_CUBIC_MONOTONIC => Self::CubicMonotonic,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for InterpolationMode {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_interpolation_mode_get_type()) }
    }
}

impl glib::value::ValueType for InterpolationMode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InterpolationMode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for InterpolationMode {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<InterpolationMode> for glib::Value {
    #[inline]
    fn from(v: InterpolationMode) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstLFOWaveform")]
pub enum LFOWaveform {
    #[doc(alias = "GST_LFO_WAVEFORM_SINE")]
    Sine,
    #[doc(alias = "GST_LFO_WAVEFORM_SQUARE")]
    Square,
    #[doc(alias = "GST_LFO_WAVEFORM_SAW")]
    Saw,
    #[doc(alias = "GST_LFO_WAVEFORM_REVERSE_SAW")]
    ReverseSaw,
    #[doc(alias = "GST_LFO_WAVEFORM_TRIANGLE")]
    Triangle,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for LFOWaveform {
    type GlibType = ffi::GstLFOWaveform;

    #[inline]
    fn into_glib(self) -> ffi::GstLFOWaveform {
        match self {
            Self::Sine => ffi::GST_LFO_WAVEFORM_SINE,
            Self::Square => ffi::GST_LFO_WAVEFORM_SQUARE,
            Self::Saw => ffi::GST_LFO_WAVEFORM_SAW,
            Self::ReverseSaw => ffi::GST_LFO_WAVEFORM_REVERSE_SAW,
            Self::Triangle => ffi::GST_LFO_WAVEFORM_TRIANGLE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstLFOWaveform> for LFOWaveform {
    #[inline]
    unsafe fn from_glib(value: ffi::GstLFOWaveform) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::GST_LFO_WAVEFORM_SINE => Self::Sine,
            ffi::GST_LFO_WAVEFORM_SQUARE => Self::Square,
            ffi::GST_LFO_WAVEFORM_SAW => Self::Saw,
            ffi::GST_LFO_WAVEFORM_REVERSE_SAW => Self::ReverseSaw,
            ffi::GST_LFO_WAVEFORM_TRIANGLE => Self::Triangle,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for LFOWaveform {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_lfo_waveform_get_type()) }
    }
}

impl glib::value::ValueType for LFOWaveform {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for LFOWaveform {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for LFOWaveform {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<LFOWaveform> for glib::Value {
    #[inline]
    fn from(v: LFOWaveform) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
