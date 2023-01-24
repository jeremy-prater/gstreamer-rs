// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::{translate::*, value::FromValue, value::ToValue, StaticType, Type};

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GstAggregatorStartTimeSelection")]
pub enum AggregatorStartTimeSelection {
    #[doc(alias = "GST_AGGREGATOR_START_TIME_SELECTION_ZERO")]
    Zero,
    #[doc(alias = "GST_AGGREGATOR_START_TIME_SELECTION_FIRST")]
    First,
    #[doc(alias = "GST_AGGREGATOR_START_TIME_SELECTION_SET")]
    Set,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
#[doc(hidden)]
impl IntoGlib for AggregatorStartTimeSelection {
    type GlibType = ffi::GstAggregatorStartTimeSelection;

    #[inline]
    fn into_glib(self) -> ffi::GstAggregatorStartTimeSelection {
        match self {
            Self::Zero => ffi::GST_AGGREGATOR_START_TIME_SELECTION_ZERO,
            Self::First => ffi::GST_AGGREGATOR_START_TIME_SELECTION_FIRST,
            Self::Set => ffi::GST_AGGREGATOR_START_TIME_SELECTION_SET,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
#[doc(hidden)]
impl FromGlib<ffi::GstAggregatorStartTimeSelection> for AggregatorStartTimeSelection {
    #[inline]
    unsafe fn from_glib(value: ffi::GstAggregatorStartTimeSelection) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::GST_AGGREGATOR_START_TIME_SELECTION_ZERO => Self::Zero,
            ffi::GST_AGGREGATOR_START_TIME_SELECTION_FIRST => Self::First,
            ffi::GST_AGGREGATOR_START_TIME_SELECTION_SET => Self::Set,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl StaticType for AggregatorStartTimeSelection {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gst_aggregator_start_time_selection_get_type()) }
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl glib::value::ValueType for AggregatorStartTimeSelection {
    type Type = Self;
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
unsafe impl<'a> FromValue<'a> for AggregatorStartTimeSelection {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl ToValue for AggregatorStartTimeSelection {
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

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
impl From<AggregatorStartTimeSelection> for glib::Value {
    #[inline]
    fn from(v: AggregatorStartTimeSelection) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
