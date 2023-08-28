// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::{bitflags::bitflags, prelude::*, translate::*};

#[cfg(feature = "v1_22")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstNavigationModifierType")]
    pub struct NavigationModifierType: u32 {
        #[doc(alias = "GST_NAVIGATION_MODIFIER_SHIFT_MASK")]
        const SHIFT_MASK = ffi::GST_NAVIGATION_MODIFIER_SHIFT_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_LOCK_MASK")]
        const LOCK_MASK = ffi::GST_NAVIGATION_MODIFIER_LOCK_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_CONTROL_MASK")]
        const CONTROL_MASK = ffi::GST_NAVIGATION_MODIFIER_CONTROL_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_MOD1_MASK")]
        const MOD1_MASK = ffi::GST_NAVIGATION_MODIFIER_MOD1_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_MOD2_MASK")]
        const MOD2_MASK = ffi::GST_NAVIGATION_MODIFIER_MOD2_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_MOD3_MASK")]
        const MOD3_MASK = ffi::GST_NAVIGATION_MODIFIER_MOD3_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_MOD4_MASK")]
        const MOD4_MASK = ffi::GST_NAVIGATION_MODIFIER_MOD4_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_MOD5_MASK")]
        const MOD5_MASK = ffi::GST_NAVIGATION_MODIFIER_MOD5_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_BUTTON1_MASK")]
        const BUTTON1_MASK = ffi::GST_NAVIGATION_MODIFIER_BUTTON1_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_BUTTON2_MASK")]
        const BUTTON2_MASK = ffi::GST_NAVIGATION_MODIFIER_BUTTON2_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_BUTTON3_MASK")]
        const BUTTON3_MASK = ffi::GST_NAVIGATION_MODIFIER_BUTTON3_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_BUTTON4_MASK")]
        const BUTTON4_MASK = ffi::GST_NAVIGATION_MODIFIER_BUTTON4_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_BUTTON5_MASK")]
        const BUTTON5_MASK = ffi::GST_NAVIGATION_MODIFIER_BUTTON5_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_SUPER_MASK")]
        const SUPER_MASK = ffi::GST_NAVIGATION_MODIFIER_SUPER_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_HYPER_MASK")]
        const HYPER_MASK = ffi::GST_NAVIGATION_MODIFIER_HYPER_MASK as _;
        #[doc(alias = "GST_NAVIGATION_MODIFIER_META_MASK")]
        const META_MASK = ffi::GST_NAVIGATION_MODIFIER_META_MASK as _;
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
#[doc(hidden)]
impl IntoGlib for NavigationModifierType {
    type GlibType = ffi::GstNavigationModifierType;

    #[inline]
    fn into_glib(self) -> ffi::GstNavigationModifierType {
        self.bits()
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
#[doc(hidden)]
impl FromGlib<ffi::GstNavigationModifierType> for NavigationModifierType {
    #[inline]
    unsafe fn from_glib(value: ffi::GstNavigationModifierType) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl StaticType for NavigationModifierType {
    #[inline]
    #[doc(alias = "gst_navigation_modifier_type_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_navigation_modifier_type_get_type()) }
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl glib::HasParamSpec for NavigationModifierType {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl glib::value::ValueType for NavigationModifierType {
    type Type = Self;
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
unsafe impl<'a> glib::value::FromValue<'a> for NavigationModifierType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl ToValue for NavigationModifierType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
impl From<NavigationModifierType> for glib::Value {
    #[inline]
    fn from(v: NavigationModifierType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoBufferFlags")]
    pub struct VideoBufferFlags: u32 {
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_INTERLACED")]
        const INTERLACED = ffi::GST_VIDEO_BUFFER_FLAG_INTERLACED as _;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_TFF")]
        const TFF = ffi::GST_VIDEO_BUFFER_FLAG_TFF as _;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_RFF")]
        const RFF = ffi::GST_VIDEO_BUFFER_FLAG_RFF as _;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_ONEFIELD")]
        const ONEFIELD = ffi::GST_VIDEO_BUFFER_FLAG_ONEFIELD as _;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_MULTIPLE_VIEW")]
        const MULTIPLE_VIEW = ffi::GST_VIDEO_BUFFER_FLAG_MULTIPLE_VIEW as _;
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_FIRST_IN_BUNDLE")]
        const FIRST_IN_BUNDLE = ffi::GST_VIDEO_BUFFER_FLAG_FIRST_IN_BUNDLE as _;
        #[cfg(feature = "v1_16")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_TOP_FIELD")]
        const TOP_FIELD = ffi::GST_VIDEO_BUFFER_FLAG_TOP_FIELD as _;
        #[cfg(feature = "v1_16")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_BOTTOM_FIELD")]
        const BOTTOM_FIELD = ffi::GST_VIDEO_BUFFER_FLAG_BOTTOM_FIELD as _;
        #[cfg(feature = "v1_18")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
        #[doc(alias = "GST_VIDEO_BUFFER_FLAG_MARKER")]
        const MARKER = ffi::GST_VIDEO_BUFFER_FLAG_MARKER as _;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoBufferFlags {
    type GlibType = ffi::GstVideoBufferFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoBufferFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoBufferFlags> for VideoBufferFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoBufferFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoBufferFlags {
    #[inline]
    #[doc(alias = "gst_video_buffer_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_buffer_flags_get_type()) }
    }
}

impl glib::HasParamSpec for VideoBufferFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for VideoBufferFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for VideoBufferFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoBufferFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<VideoBufferFlags> for glib::Value {
    #[inline]
    fn from(v: VideoBufferFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoChromaSite")]
    pub struct VideoChromaSite: u32 {
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_NONE")]
        const NONE = ffi::GST_VIDEO_CHROMA_SITE_NONE as _;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_H_COSITED")]
        const H_COSITED = ffi::GST_VIDEO_CHROMA_SITE_H_COSITED as _;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_V_COSITED")]
        const V_COSITED = ffi::GST_VIDEO_CHROMA_SITE_V_COSITED as _;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_ALT_LINE")]
        const ALT_LINE = ffi::GST_VIDEO_CHROMA_SITE_ALT_LINE as _;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_COSITED")]
        const COSITED = ffi::GST_VIDEO_CHROMA_SITE_COSITED as _;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_JPEG")]
        const JPEG = ffi::GST_VIDEO_CHROMA_SITE_JPEG as _;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_MPEG2")]
        const MPEG2 = ffi::GST_VIDEO_CHROMA_SITE_MPEG2 as _;
        #[doc(alias = "GST_VIDEO_CHROMA_SITE_DV")]
        const DV = ffi::GST_VIDEO_CHROMA_SITE_DV as _;
    }
}

impl VideoChromaSite {
    #[cfg(feature = "v1_20")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[doc(alias = "gst_video_chroma_site_from_string")]
    pub fn from_string(s: &str) -> VideoChromaSite {
        assert_initialized_main_thread!();
        unsafe { from_glib(ffi::gst_video_chroma_site_from_string(s.to_glib_none().0)) }
    }
}

impl std::fmt::Display for VideoChromaSite {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_str())
    }
}

#[doc(hidden)]
impl IntoGlib for VideoChromaSite {
    type GlibType = ffi::GstVideoChromaSite;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoChromaSite {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoChromaSite> for VideoChromaSite {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoChromaSite) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoChromaSite {
    #[inline]
    #[doc(alias = "gst_video_chroma_site_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_chroma_site_get_type()) }
    }
}

impl glib::HasParamSpec for VideoChromaSite {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for VideoChromaSite {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for VideoChromaSite {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoChromaSite {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<VideoChromaSite> for glib::Value {
    #[inline]
    fn from(v: VideoChromaSite) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoCodecFrameFlags")]
    pub struct VideoCodecFrameFlags: u32 {
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_DECODE_ONLY")]
        const DECODE_ONLY = ffi::GST_VIDEO_CODEC_FRAME_FLAG_DECODE_ONLY as _;
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_SYNC_POINT")]
        const SYNC_POINT = ffi::GST_VIDEO_CODEC_FRAME_FLAG_SYNC_POINT as _;
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME")]
        const FORCE_KEYFRAME = ffi::GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME as _;
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME_HEADERS")]
        const FORCE_KEYFRAME_HEADERS = ffi::GST_VIDEO_CODEC_FRAME_FLAG_FORCE_KEYFRAME_HEADERS as _;
        #[cfg(feature = "v1_20")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
        #[doc(alias = "GST_VIDEO_CODEC_FRAME_FLAG_CORRUPTED")]
        const CORRUPTED = ffi::GST_VIDEO_CODEC_FRAME_FLAG_CORRUPTED as _;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoCodecFrameFlags {
    type GlibType = ffi::GstVideoCodecFrameFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoCodecFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoCodecFrameFlags> for VideoCodecFrameFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoCodecFrameFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl StaticType for VideoCodecFrameFlags {
    #[inline]
    #[doc(alias = "gst_video_codec_frame_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_codec_frame_flags_get_type()) }
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::HasParamSpec for VideoCodecFrameFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for VideoCodecFrameFlags {
    type Type = Self;
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl<'a> glib::value::FromValue<'a> for VideoCodecFrameFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl ToValue for VideoCodecFrameFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl From<VideoCodecFrameFlags> for glib::Value {
    #[inline]
    fn from(v: VideoCodecFrameFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(feature = "v1_20")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoDecoderRequestSyncPointFlags")]
    pub struct VideoDecoderRequestSyncPointFlags: u32 {
        #[doc(alias = "GST_VIDEO_DECODER_REQUEST_SYNC_POINT_DISCARD_INPUT")]
        const DISCARD_INPUT = ffi::GST_VIDEO_DECODER_REQUEST_SYNC_POINT_DISCARD_INPUT as _;
        #[doc(alias = "GST_VIDEO_DECODER_REQUEST_SYNC_POINT_CORRUPT_OUTPUT")]
        const CORRUPT_OUTPUT = ffi::GST_VIDEO_DECODER_REQUEST_SYNC_POINT_CORRUPT_OUTPUT as _;
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl IntoGlib for VideoDecoderRequestSyncPointFlags {
    type GlibType = ffi::GstVideoDecoderRequestSyncPointFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoDecoderRequestSyncPointFlags {
        self.bits()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
#[doc(hidden)]
impl FromGlib<ffi::GstVideoDecoderRequestSyncPointFlags> for VideoDecoderRequestSyncPointFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoDecoderRequestSyncPointFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl StaticType for VideoDecoderRequestSyncPointFlags {
    #[inline]
    #[doc(alias = "gst_video_decoder_request_sync_point_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_decoder_request_sync_point_flags_get_type()) }
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::HasParamSpec for VideoDecoderRequestSyncPointFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl glib::value::ValueType for VideoDecoderRequestSyncPointFlags {
    type Type = Self;
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
unsafe impl<'a> glib::value::FromValue<'a> for VideoDecoderRequestSyncPointFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl ToValue for VideoDecoderRequestSyncPointFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_20")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_20")))]
impl From<VideoDecoderRequestSyncPointFlags> for glib::Value {
    #[inline]
    fn from(v: VideoDecoderRequestSyncPointFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoFlags")]
    pub struct VideoFlags: u32 {
        #[doc(alias = "GST_VIDEO_FLAG_VARIABLE_FPS")]
        const VARIABLE_FPS = ffi::GST_VIDEO_FLAG_VARIABLE_FPS as _;
        #[doc(alias = "GST_VIDEO_FLAG_PREMULTIPLIED_ALPHA")]
        const PREMULTIPLIED_ALPHA = ffi::GST_VIDEO_FLAG_PREMULTIPLIED_ALPHA as _;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoFlags {
    type GlibType = ffi::GstVideoFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFlags> for VideoFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoFlags {
    #[inline]
    #[doc(alias = "gst_video_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_flags_get_type()) }
    }
}

impl glib::HasParamSpec for VideoFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for VideoFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for VideoFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<VideoFlags> for glib::Value {
    #[inline]
    fn from(v: VideoFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoFormatFlags")]
    pub struct VideoFormatFlags: u32 {
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_YUV")]
        const YUV = ffi::GST_VIDEO_FORMAT_FLAG_YUV as _;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_RGB")]
        const RGB = ffi::GST_VIDEO_FORMAT_FLAG_RGB as _;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_GRAY")]
        const GRAY = ffi::GST_VIDEO_FORMAT_FLAG_GRAY as _;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_ALPHA")]
        const ALPHA = ffi::GST_VIDEO_FORMAT_FLAG_ALPHA as _;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_LE")]
        const LE = ffi::GST_VIDEO_FORMAT_FLAG_LE as _;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_PALETTE")]
        const PALETTE = ffi::GST_VIDEO_FORMAT_FLAG_PALETTE as _;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_COMPLEX")]
        const COMPLEX = ffi::GST_VIDEO_FORMAT_FLAG_COMPLEX as _;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_UNPACK")]
        const UNPACK = ffi::GST_VIDEO_FORMAT_FLAG_UNPACK as _;
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_TILED")]
        const TILED = ffi::GST_VIDEO_FORMAT_FLAG_TILED as _;
        #[cfg(feature = "v1_22")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_22")))]
        #[doc(alias = "GST_VIDEO_FORMAT_FLAG_SUBTILES")]
        const SUBTILES = ffi::GST_VIDEO_FORMAT_FLAG_SUBTILES as _;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoFormatFlags {
    type GlibType = ffi::GstVideoFormatFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFormatFlags> for VideoFormatFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoFormatFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoFormatFlags {
    #[inline]
    #[doc(alias = "gst_video_format_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_format_flags_get_type()) }
    }
}

impl glib::HasParamSpec for VideoFormatFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for VideoFormatFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for VideoFormatFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoFormatFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<VideoFormatFlags> for glib::Value {
    #[inline]
    fn from(v: VideoFormatFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoFrameFlags")]
    pub struct VideoFrameFlags: u32 {
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_INTERLACED")]
        const INTERLACED = ffi::GST_VIDEO_FRAME_FLAG_INTERLACED as _;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_TFF")]
        const TFF = ffi::GST_VIDEO_FRAME_FLAG_TFF as _;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_RFF")]
        const RFF = ffi::GST_VIDEO_FRAME_FLAG_RFF as _;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_ONEFIELD")]
        const ONEFIELD = ffi::GST_VIDEO_FRAME_FLAG_ONEFIELD as _;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_MULTIPLE_VIEW")]
        const MULTIPLE_VIEW = ffi::GST_VIDEO_FRAME_FLAG_MULTIPLE_VIEW as _;
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_FIRST_IN_BUNDLE")]
        const FIRST_IN_BUNDLE = ffi::GST_VIDEO_FRAME_FLAG_FIRST_IN_BUNDLE as _;
        #[cfg(feature = "v1_16")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_TOP_FIELD")]
        const TOP_FIELD = ffi::GST_VIDEO_FRAME_FLAG_TOP_FIELD as _;
        #[cfg(feature = "v1_16")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
        #[doc(alias = "GST_VIDEO_FRAME_FLAG_BOTTOM_FIELD")]
        const BOTTOM_FIELD = ffi::GST_VIDEO_FRAME_FLAG_BOTTOM_FIELD as _;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoFrameFlags {
    type GlibType = ffi::GstVideoFrameFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoFrameFlags> for VideoFrameFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoFrameFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoFrameFlags {
    #[inline]
    #[doc(alias = "gst_video_frame_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_frame_flags_get_type()) }
    }
}

impl glib::HasParamSpec for VideoFrameFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for VideoFrameFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for VideoFrameFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoFrameFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<VideoFrameFlags> for glib::Value {
    #[inline]
    fn from(v: VideoFrameFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoMultiviewFlags")]
    pub struct VideoMultiviewFlags: u32 {
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_VIEW_FIRST")]
        const RIGHT_VIEW_FIRST = ffi::GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_VIEW_FIRST as _;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLIPPED")]
        const LEFT_FLIPPED = ffi::GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLIPPED as _;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLOPPED")]
        const LEFT_FLOPPED = ffi::GST_VIDEO_MULTIVIEW_FLAGS_LEFT_FLOPPED as _;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLIPPED")]
        const RIGHT_FLIPPED = ffi::GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLIPPED as _;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLOPPED")]
        const RIGHT_FLOPPED = ffi::GST_VIDEO_MULTIVIEW_FLAGS_RIGHT_FLOPPED as _;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_HALF_ASPECT")]
        const HALF_ASPECT = ffi::GST_VIDEO_MULTIVIEW_FLAGS_HALF_ASPECT as _;
        #[doc(alias = "GST_VIDEO_MULTIVIEW_FLAGS_MIXED_MONO")]
        const MIXED_MONO = ffi::GST_VIDEO_MULTIVIEW_FLAGS_MIXED_MONO as _;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoMultiviewFlags {
    type GlibType = ffi::GstVideoMultiviewFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoMultiviewFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoMultiviewFlags> for VideoMultiviewFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoMultiviewFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoMultiviewFlags {
    #[inline]
    #[doc(alias = "gst_video_multiview_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_multiview_flags_get_type()) }
    }
}

impl glib::HasParamSpec for VideoMultiviewFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for VideoMultiviewFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for VideoMultiviewFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoMultiviewFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<VideoMultiviewFlags> for glib::Value {
    #[inline]
    fn from(v: VideoMultiviewFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoOverlayFormatFlags")]
    pub struct VideoOverlayFormatFlags: u32 {
        #[doc(alias = "GST_VIDEO_OVERLAY_FORMAT_FLAG_PREMULTIPLIED_ALPHA")]
        const PREMULTIPLIED_ALPHA = ffi::GST_VIDEO_OVERLAY_FORMAT_FLAG_PREMULTIPLIED_ALPHA as _;
        #[doc(alias = "GST_VIDEO_OVERLAY_FORMAT_FLAG_GLOBAL_ALPHA")]
        const GLOBAL_ALPHA = ffi::GST_VIDEO_OVERLAY_FORMAT_FLAG_GLOBAL_ALPHA as _;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoOverlayFormatFlags {
    type GlibType = ffi::GstVideoOverlayFormatFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoOverlayFormatFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoOverlayFormatFlags> for VideoOverlayFormatFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoOverlayFormatFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl StaticType for VideoOverlayFormatFlags {
    #[inline]
    #[doc(alias = "gst_video_overlay_format_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_overlay_format_flags_get_type()) }
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl glib::HasParamSpec for VideoOverlayFormatFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl glib::value::ValueType for VideoOverlayFormatFlags {
    type Type = Self;
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
unsafe impl<'a> glib::value::FromValue<'a> for VideoOverlayFormatFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl ToValue for VideoOverlayFormatFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_16")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
impl From<VideoOverlayFormatFlags> for glib::Value {
    #[inline]
    fn from(v: VideoOverlayFormatFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoPackFlags")]
    pub struct VideoPackFlags: u32 {
        #[doc(alias = "GST_VIDEO_PACK_FLAG_TRUNCATE_RANGE")]
        const TRUNCATE_RANGE = ffi::GST_VIDEO_PACK_FLAG_TRUNCATE_RANGE as _;
        #[doc(alias = "GST_VIDEO_PACK_FLAG_INTERLACED")]
        const INTERLACED = ffi::GST_VIDEO_PACK_FLAG_INTERLACED as _;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoPackFlags {
    type GlibType = ffi::GstVideoPackFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoPackFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoPackFlags> for VideoPackFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoPackFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

impl StaticType for VideoPackFlags {
    #[inline]
    #[doc(alias = "gst_video_pack_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_pack_flags_get_type()) }
    }
}

impl glib::HasParamSpec for VideoPackFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl glib::value::ValueType for VideoPackFlags {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for VideoPackFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

impl ToValue for VideoPackFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<VideoPackFlags> for glib::Value {
    #[inline]
    fn from(v: VideoPackFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GstVideoTimeCodeFlags")]
    pub struct VideoTimeCodeFlags: u32 {
        #[doc(alias = "GST_VIDEO_TIME_CODE_FLAGS_DROP_FRAME")]
        const DROP_FRAME = ffi::GST_VIDEO_TIME_CODE_FLAGS_DROP_FRAME as _;
        #[doc(alias = "GST_VIDEO_TIME_CODE_FLAGS_INTERLACED")]
        const INTERLACED = ffi::GST_VIDEO_TIME_CODE_FLAGS_INTERLACED as _;
    }
}

#[doc(hidden)]
impl IntoGlib for VideoTimeCodeFlags {
    type GlibType = ffi::GstVideoTimeCodeFlags;

    #[inline]
    fn into_glib(self) -> ffi::GstVideoTimeCodeFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GstVideoTimeCodeFlags> for VideoTimeCodeFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GstVideoTimeCodeFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl StaticType for VideoTimeCodeFlags {
    #[inline]
    #[doc(alias = "gst_video_time_code_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gst_video_time_code_flags_get_type()) }
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl glib::HasParamSpec for VideoTimeCodeFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl glib::value::ValueType for VideoTimeCodeFlags {
    type Type = Self;
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
unsafe impl<'a> glib::value::FromValue<'a> for VideoTimeCodeFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl ToValue for VideoTimeCodeFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
impl From<VideoTimeCodeFlags> for glib::Value {
    #[inline]
    fn from(v: VideoTimeCodeFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
