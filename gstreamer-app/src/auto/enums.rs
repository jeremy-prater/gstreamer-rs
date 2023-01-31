// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{translate::*, value::FromValue, value::ToValue, StaticType, Type};

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAppLeakyType")]
pub enum AppLeakyType {
    #[doc(alias = "GST_APP_LEAKY_TYPE_NONE")]
    None,
    #[doc(alias = "GST_APP_LEAKY_TYPE_UPSTREAM")]
    Upstream,
    #[doc(alias = "GST_APP_LEAKY_TYPE_DOWNSTREAM")]
    Downstream,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for AppLeakyType {
    type GlibType = ffi::GstAppLeakyType;

    #[inline]
    fn into_glib(self) -> ffi::GstAppLeakyType {
        match self {
            Self::None => ffi::GST_APP_LEAKY_TYPE_NONE,
            Self::Upstream => ffi::GST_APP_LEAKY_TYPE_UPSTREAM,
            Self::Downstream => ffi::GST_APP_LEAKY_TYPE_DOWNSTREAM,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstAppLeakyType> for AppLeakyType {
    #[inline]
    unsafe fn from_glib(value: ffi::GstAppLeakyType) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::GST_APP_LEAKY_TYPE_NONE => Self::None,
            ffi::GST_APP_LEAKY_TYPE_UPSTREAM => Self::Upstream,
            ffi::GST_APP_LEAKY_TYPE_DOWNSTREAM => Self::Downstream,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl StaticType for AppLeakyType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_app_leaky_type_get_type()) }
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl glib::HasParamSpec for AppLeakyType {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for AppLeakyType {
    type Type = Self;
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
unsafe impl<'a> FromValue<'a> for AppLeakyType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl ToValue for AppLeakyType {
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

#[cfg(any(feature = "v1_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
impl From<AppLeakyType> for glib::Value {
    #[inline]
    fn from(v: AppLeakyType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAppStreamType")]
pub enum AppStreamType {
    #[doc(alias = "GST_APP_STREAM_TYPE_STREAM")]
    Stream,
    #[doc(alias = "GST_APP_STREAM_TYPE_SEEKABLE")]
    Seekable,
    #[doc(alias = "GST_APP_STREAM_TYPE_RANDOM_ACCESS")]
    RandomAccess,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for AppStreamType {
    type GlibType = ffi::GstAppStreamType;

    #[inline]
    fn into_glib(self) -> ffi::GstAppStreamType {
        match self {
            Self::Stream => ffi::GST_APP_STREAM_TYPE_STREAM,
            Self::Seekable => ffi::GST_APP_STREAM_TYPE_SEEKABLE,
            Self::RandomAccess => ffi::GST_APP_STREAM_TYPE_RANDOM_ACCESS,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstAppStreamType> for AppStreamType {
    #[inline]
    unsafe fn from_glib(value: ffi::GstAppStreamType) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::GST_APP_STREAM_TYPE_STREAM => Self::Stream,
            ffi::GST_APP_STREAM_TYPE_SEEKABLE => Self::Seekable,
            ffi::GST_APP_STREAM_TYPE_RANDOM_ACCESS => Self::RandomAccess,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for AppStreamType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_app_stream_type_get_type()) }
    }
}

impl glib::HasParamSpec for AppStreamType {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for AppStreamType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AppStreamType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AppStreamType {
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

impl From<AppStreamType> for glib::Value {
    #[inline]
    fn from(v: AppStreamType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
