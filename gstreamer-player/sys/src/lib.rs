// Generated by gir (https://github.com/gtk-rs/gir @ b5068ede6c51)
// from gir-files (https://github.com/gtk-rs/gir-files @ 01c4ec663b3f)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 40cea11af0f3)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type GstPlayerColorBalanceType = c_int;
pub const GST_PLAYER_COLOR_BALANCE_HUE: GstPlayerColorBalanceType = 3;
pub const GST_PLAYER_COLOR_BALANCE_BRIGHTNESS: GstPlayerColorBalanceType = 0;
pub const GST_PLAYER_COLOR_BALANCE_SATURATION: GstPlayerColorBalanceType = 2;
pub const GST_PLAYER_COLOR_BALANCE_CONTRAST: GstPlayerColorBalanceType = 1;

pub type GstPlayerError = c_int;
pub const GST_PLAYER_ERROR_FAILED: GstPlayerError = 0;

pub type GstPlayerSnapshotFormat = c_int;
pub const GST_PLAYER_THUMBNAIL_RAW_NATIVE: GstPlayerSnapshotFormat = 0;
pub const GST_PLAYER_THUMBNAIL_RAW_xRGB: GstPlayerSnapshotFormat = 1;
pub const GST_PLAYER_THUMBNAIL_RAW_BGRx: GstPlayerSnapshotFormat = 2;
pub const GST_PLAYER_THUMBNAIL_JPG: GstPlayerSnapshotFormat = 3;
pub const GST_PLAYER_THUMBNAIL_PNG: GstPlayerSnapshotFormat = 4;

pub type GstPlayerState = c_int;
pub const GST_PLAYER_STATE_STOPPED: GstPlayerState = 0;
pub const GST_PLAYER_STATE_BUFFERING: GstPlayerState = 1;
pub const GST_PLAYER_STATE_PAUSED: GstPlayerState = 2;
pub const GST_PLAYER_STATE_PLAYING: GstPlayerState = 3;

// Callbacks
pub type GstPlayerSignalDispatcherFunc = Option<unsafe extern "C" fn(gpointer)>;

// Records
#[repr(C)]
pub struct _GstPlayerAudioInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayerAudioInfoClass = *mut _GstPlayerAudioInfoClass;

#[repr(C)]
pub struct _GstPlayerClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayerClass = *mut _GstPlayerClass;

#[repr(C)]
pub struct _GstPlayerGMainContextSignalDispatcherClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayerGMainContextSignalDispatcherClass =
    *mut _GstPlayerGMainContextSignalDispatcherClass;

#[repr(C)]
pub struct _GstPlayerMediaInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayerMediaInfoClass = *mut _GstPlayerMediaInfoClass;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstPlayerSignalDispatcherInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub dispatch: Option<
        unsafe extern "C" fn(
            *mut GstPlayerSignalDispatcher,
            *mut GstPlayer,
            GstPlayerSignalDispatcherFunc,
            gpointer,
            glib::GDestroyNotify,
        ),
    >,
}

impl ::std::fmt::Debug for GstPlayerSignalDispatcherInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayerSignalDispatcherInterface @ {:p}", self))
            .field("parent_iface", &self.parent_iface)
            .field("dispatch", &self.dispatch)
            .finish()
    }
}

#[repr(C)]
pub struct _GstPlayerStreamInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayerStreamInfoClass = *mut _GstPlayerStreamInfoClass;

#[repr(C)]
pub struct _GstPlayerSubtitleInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayerSubtitleInfoClass = *mut _GstPlayerSubtitleInfoClass;

#[repr(C)]
pub struct _GstPlayerVideoInfoClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayerVideoInfoClass = *mut _GstPlayerVideoInfoClass;

#[repr(C)]
pub struct _GstPlayerVideoOverlayVideoRendererClass {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type GstPlayerVideoOverlayVideoRendererClass = *mut _GstPlayerVideoOverlayVideoRendererClass;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstPlayerVideoRendererInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub create_video_sink: Option<
        unsafe extern "C" fn(*mut GstPlayerVideoRenderer, *mut GstPlayer) -> *mut gst::GstElement,
    >,
}

impl ::std::fmt::Debug for GstPlayerVideoRendererInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayerVideoRendererInterface @ {:p}", self))
            .field("parent_iface", &self.parent_iface)
            .field("create_video_sink", &self.create_video_sink)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstPlayerVisualization {
    pub name: *mut c_char,
    pub description: *mut c_char,
}

impl ::std::fmt::Debug for GstPlayerVisualization {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayerVisualization @ {:p}", self))
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

// Classes
#[repr(C)]
pub struct GstPlayer {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayer @ {:p}", self)).finish()
    }
}

#[repr(C)]
pub struct GstPlayerAudioInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayerAudioInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayerAudioInfo @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct GstPlayerGMainContextSignalDispatcher {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayerGMainContextSignalDispatcher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!(
            "GstPlayerGMainContextSignalDispatcher @ {:p}",
            self
        ))
        .finish()
    }
}

#[repr(C)]
pub struct GstPlayerMediaInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayerMediaInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayerMediaInfo @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct GstPlayerStreamInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayerStreamInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayerStreamInfo @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct GstPlayerSubtitleInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayerSubtitleInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayerSubtitleInfo @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct GstPlayerVideoInfo {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayerVideoInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayerVideoInfo @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct GstPlayerVideoOverlayVideoRenderer {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayerVideoOverlayVideoRenderer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPlayerVideoOverlayVideoRenderer @ {:p}", self))
            .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct GstPlayerSignalDispatcher {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayerSignalDispatcher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "GstPlayerSignalDispatcher @ {:p}", self)
    }
}

#[repr(C)]
pub struct GstPlayerVideoRenderer {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPlayerVideoRenderer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "GstPlayerVideoRenderer @ {:p}", self)
    }
}

#[link(name = "gstplayer-1.0")]
extern "C" {

    //=========================================================================
    // GstPlayerColorBalanceType
    //=========================================================================
    pub fn gst_player_color_balance_type_get_type() -> GType;
    pub fn gst_player_color_balance_type_get_name(
        type_: GstPlayerColorBalanceType,
    ) -> *const c_char;

    //=========================================================================
    // GstPlayerError
    //=========================================================================
    pub fn gst_player_error_get_type() -> GType;
    pub fn gst_player_error_get_name(error: GstPlayerError) -> *const c_char;
    pub fn gst_player_error_quark() -> glib::GQuark;

    //=========================================================================
    // GstPlayerState
    //=========================================================================
    pub fn gst_player_state_get_type() -> GType;
    pub fn gst_player_state_get_name(state: GstPlayerState) -> *const c_char;

    //=========================================================================
    // GstPlayerVisualization
    //=========================================================================
    pub fn gst_player_visualization_get_type() -> GType;
    pub fn gst_player_visualization_copy(
        vis: *const GstPlayerVisualization,
    ) -> *mut GstPlayerVisualization;
    pub fn gst_player_visualization_free(vis: *mut GstPlayerVisualization);

    //=========================================================================
    // GstPlayer
    //=========================================================================
    pub fn gst_player_get_type() -> GType;
    pub fn gst_player_new(
        video_renderer: *mut GstPlayerVideoRenderer,
        signal_dispatcher: *mut GstPlayerSignalDispatcher,
    ) -> *mut GstPlayer;
    pub fn gst_player_config_get_position_update_interval(
        config: *const gst::GstStructure,
    ) -> c_uint;
    pub fn gst_player_config_get_seek_accurate(config: *const gst::GstStructure) -> gboolean;
    pub fn gst_player_config_get_user_agent(config: *const gst::GstStructure) -> *mut c_char;
    pub fn gst_player_config_set_position_update_interval(
        config: *mut gst::GstStructure,
        interval: c_uint,
    );
    pub fn gst_player_config_set_seek_accurate(config: *mut gst::GstStructure, accurate: gboolean);
    pub fn gst_player_config_set_user_agent(config: *mut gst::GstStructure, agent: *const c_char);
    pub fn gst_player_get_audio_streams(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_get_subtitle_streams(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_get_video_streams(info: *const GstPlayerMediaInfo) -> *mut glib::GList;
    pub fn gst_player_visualizations_free(viss: *mut *mut GstPlayerVisualization);
    pub fn gst_player_visualizations_get() -> *mut *mut GstPlayerVisualization;
    pub fn gst_player_get_audio_video_offset(player: *mut GstPlayer) -> i64;
    pub fn gst_player_get_color_balance(
        player: *mut GstPlayer,
        type_: GstPlayerColorBalanceType,
    ) -> c_double;
    pub fn gst_player_get_config(player: *mut GstPlayer) -> *mut gst::GstStructure;
    pub fn gst_player_get_current_audio_track(player: *mut GstPlayer) -> *mut GstPlayerAudioInfo;
    pub fn gst_player_get_current_subtitle_track(
        player: *mut GstPlayer,
    ) -> *mut GstPlayerSubtitleInfo;
    pub fn gst_player_get_current_video_track(player: *mut GstPlayer) -> *mut GstPlayerVideoInfo;
    pub fn gst_player_get_current_visualization(player: *mut GstPlayer) -> *mut c_char;
    pub fn gst_player_get_duration(player: *mut GstPlayer) -> gst::GstClockTime;
    pub fn gst_player_get_media_info(player: *mut GstPlayer) -> *mut GstPlayerMediaInfo;
    pub fn gst_player_get_multiview_flags(
        player: *mut GstPlayer,
    ) -> gst_video::GstVideoMultiviewFlags;
    pub fn gst_player_get_multiview_mode(
        player: *mut GstPlayer,
    ) -> gst_video::GstVideoMultiviewFramePacking;
    pub fn gst_player_get_mute(player: *mut GstPlayer) -> gboolean;
    pub fn gst_player_get_pipeline(player: *mut GstPlayer) -> *mut gst::GstElement;
    pub fn gst_player_get_position(player: *mut GstPlayer) -> gst::GstClockTime;
    pub fn gst_player_get_rate(player: *mut GstPlayer) -> c_double;
    pub fn gst_player_get_subtitle_uri(player: *mut GstPlayer) -> *mut c_char;
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn gst_player_get_subtitle_video_offset(player: *mut GstPlayer) -> i64;
    pub fn gst_player_get_uri(player: *mut GstPlayer) -> *mut c_char;
    pub fn gst_player_get_video_snapshot(
        player: *mut GstPlayer,
        format: GstPlayerSnapshotFormat,
        config: *const gst::GstStructure,
    ) -> *mut gst::GstSample;
    pub fn gst_player_get_volume(player: *mut GstPlayer) -> c_double;
    pub fn gst_player_has_color_balance(player: *mut GstPlayer) -> gboolean;
    pub fn gst_player_pause(player: *mut GstPlayer);
    pub fn gst_player_play(player: *mut GstPlayer);
    pub fn gst_player_seek(player: *mut GstPlayer, position: gst::GstClockTime);
    pub fn gst_player_set_audio_track(player: *mut GstPlayer, stream_index: c_int) -> gboolean;
    pub fn gst_player_set_audio_track_enabled(player: *mut GstPlayer, enabled: gboolean);
    pub fn gst_player_set_audio_video_offset(player: *mut GstPlayer, offset: i64);
    pub fn gst_player_set_color_balance(
        player: *mut GstPlayer,
        type_: GstPlayerColorBalanceType,
        value: c_double,
    );
    pub fn gst_player_set_config(
        player: *mut GstPlayer,
        config: *mut gst::GstStructure,
    ) -> gboolean;
    pub fn gst_player_set_multiview_flags(
        player: *mut GstPlayer,
        flags: gst_video::GstVideoMultiviewFlags,
    );
    pub fn gst_player_set_multiview_mode(
        player: *mut GstPlayer,
        mode: gst_video::GstVideoMultiviewFramePacking,
    );
    pub fn gst_player_set_mute(player: *mut GstPlayer, val: gboolean);
    pub fn gst_player_set_rate(player: *mut GstPlayer, rate: c_double);
    pub fn gst_player_set_subtitle_track(player: *mut GstPlayer, stream_index: c_int) -> gboolean;
    pub fn gst_player_set_subtitle_track_enabled(player: *mut GstPlayer, enabled: gboolean);
    pub fn gst_player_set_subtitle_uri(player: *mut GstPlayer, uri: *const c_char);
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn gst_player_set_subtitle_video_offset(player: *mut GstPlayer, offset: i64);
    pub fn gst_player_set_uri(player: *mut GstPlayer, uri: *const c_char);
    pub fn gst_player_set_video_track(player: *mut GstPlayer, stream_index: c_int) -> gboolean;
    pub fn gst_player_set_video_track_enabled(player: *mut GstPlayer, enabled: gboolean);
    pub fn gst_player_set_visualization(player: *mut GstPlayer, name: *const c_char) -> gboolean;
    pub fn gst_player_set_visualization_enabled(player: *mut GstPlayer, enabled: gboolean);
    pub fn gst_player_set_volume(player: *mut GstPlayer, val: c_double);
    pub fn gst_player_stop(player: *mut GstPlayer);

    //=========================================================================
    // GstPlayerAudioInfo
    //=========================================================================
    pub fn gst_player_audio_info_get_type() -> GType;
    pub fn gst_player_audio_info_get_bitrate(info: *const GstPlayerAudioInfo) -> c_int;
    pub fn gst_player_audio_info_get_channels(info: *const GstPlayerAudioInfo) -> c_int;
    pub fn gst_player_audio_info_get_language(info: *const GstPlayerAudioInfo) -> *const c_char;
    pub fn gst_player_audio_info_get_max_bitrate(info: *const GstPlayerAudioInfo) -> c_int;
    pub fn gst_player_audio_info_get_sample_rate(info: *const GstPlayerAudioInfo) -> c_int;

    //=========================================================================
    // GstPlayerGMainContextSignalDispatcher
    //=========================================================================
    pub fn gst_player_g_main_context_signal_dispatcher_get_type() -> GType;
    pub fn gst_player_g_main_context_signal_dispatcher_new(
        application_context: *mut glib::GMainContext,
    ) -> *mut GstPlayerSignalDispatcher;

    //=========================================================================
    // GstPlayerMediaInfo
    //=========================================================================
    pub fn gst_player_media_info_get_type() -> GType;
    pub fn gst_player_media_info_get_audio_streams(
        info: *const GstPlayerMediaInfo,
    ) -> *mut glib::GList;
    pub fn gst_player_media_info_get_container_format(
        info: *const GstPlayerMediaInfo,
    ) -> *const c_char;
    pub fn gst_player_media_info_get_duration(info: *const GstPlayerMediaInfo)
        -> gst::GstClockTime;
    pub fn gst_player_media_info_get_image_sample(
        info: *const GstPlayerMediaInfo,
    ) -> *mut gst::GstSample;
    pub fn gst_player_media_info_get_number_of_audio_streams(
        info: *const GstPlayerMediaInfo,
    ) -> c_uint;
    pub fn gst_player_media_info_get_number_of_streams(info: *const GstPlayerMediaInfo) -> c_uint;
    pub fn gst_player_media_info_get_number_of_subtitle_streams(
        info: *const GstPlayerMediaInfo,
    ) -> c_uint;
    pub fn gst_player_media_info_get_number_of_video_streams(
        info: *const GstPlayerMediaInfo,
    ) -> c_uint;
    pub fn gst_player_media_info_get_stream_list(
        info: *const GstPlayerMediaInfo,
    ) -> *mut glib::GList;
    pub fn gst_player_media_info_get_subtitle_streams(
        info: *const GstPlayerMediaInfo,
    ) -> *mut glib::GList;
    pub fn gst_player_media_info_get_tags(info: *const GstPlayerMediaInfo) -> *mut gst::GstTagList;
    pub fn gst_player_media_info_get_title(info: *const GstPlayerMediaInfo) -> *const c_char;
    pub fn gst_player_media_info_get_uri(info: *const GstPlayerMediaInfo) -> *const c_char;
    pub fn gst_player_media_info_get_video_streams(
        info: *const GstPlayerMediaInfo,
    ) -> *mut glib::GList;
    pub fn gst_player_media_info_is_live(info: *const GstPlayerMediaInfo) -> gboolean;
    pub fn gst_player_media_info_is_seekable(info: *const GstPlayerMediaInfo) -> gboolean;

    //=========================================================================
    // GstPlayerStreamInfo
    //=========================================================================
    pub fn gst_player_stream_info_get_type() -> GType;
    pub fn gst_player_stream_info_get_caps(info: *const GstPlayerStreamInfo) -> *mut gst::GstCaps;
    pub fn gst_player_stream_info_get_codec(info: *const GstPlayerStreamInfo) -> *const c_char;
    pub fn gst_player_stream_info_get_index(info: *const GstPlayerStreamInfo) -> c_int;
    pub fn gst_player_stream_info_get_stream_type(
        info: *const GstPlayerStreamInfo,
    ) -> *const c_char;
    pub fn gst_player_stream_info_get_tags(
        info: *const GstPlayerStreamInfo,
    ) -> *mut gst::GstTagList;

    //=========================================================================
    // GstPlayerSubtitleInfo
    //=========================================================================
    pub fn gst_player_subtitle_info_get_type() -> GType;
    pub fn gst_player_subtitle_info_get_language(
        info: *const GstPlayerSubtitleInfo,
    ) -> *const c_char;

    //=========================================================================
    // GstPlayerVideoInfo
    //=========================================================================
    pub fn gst_player_video_info_get_type() -> GType;
    pub fn gst_player_video_info_get_bitrate(info: *const GstPlayerVideoInfo) -> c_int;
    pub fn gst_player_video_info_get_framerate(
        info: *const GstPlayerVideoInfo,
        fps_n: *mut c_int,
        fps_d: *mut c_int,
    );
    pub fn gst_player_video_info_get_height(info: *const GstPlayerVideoInfo) -> c_int;
    pub fn gst_player_video_info_get_max_bitrate(info: *const GstPlayerVideoInfo) -> c_int;
    pub fn gst_player_video_info_get_pixel_aspect_ratio(
        info: *const GstPlayerVideoInfo,
        par_n: *mut c_uint,
        par_d: *mut c_uint,
    );
    pub fn gst_player_video_info_get_width(info: *const GstPlayerVideoInfo) -> c_int;

    //=========================================================================
    // GstPlayerVideoOverlayVideoRenderer
    //=========================================================================
    pub fn gst_player_video_overlay_video_renderer_get_type() -> GType;
    pub fn gst_player_video_overlay_video_renderer_new(
        window_handle: gpointer,
    ) -> *mut GstPlayerVideoRenderer;
    pub fn gst_player_video_overlay_video_renderer_new_with_sink(
        window_handle: gpointer,
        video_sink: *mut gst::GstElement,
    ) -> *mut GstPlayerVideoRenderer;
    pub fn gst_player_video_overlay_video_renderer_expose(
        self_: *mut GstPlayerVideoOverlayVideoRenderer,
    );
    pub fn gst_player_video_overlay_video_renderer_get_render_rectangle(
        self_: *mut GstPlayerVideoOverlayVideoRenderer,
        x: *mut c_int,
        y: *mut c_int,
        width: *mut c_int,
        height: *mut c_int,
    );
    pub fn gst_player_video_overlay_video_renderer_get_window_handle(
        self_: *mut GstPlayerVideoOverlayVideoRenderer,
    ) -> gpointer;
    pub fn gst_player_video_overlay_video_renderer_set_render_rectangle(
        self_: *mut GstPlayerVideoOverlayVideoRenderer,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn gst_player_video_overlay_video_renderer_set_window_handle(
        self_: *mut GstPlayerVideoOverlayVideoRenderer,
        window_handle: gpointer,
    );

    //=========================================================================
    // GstPlayerSignalDispatcher
    //=========================================================================
    pub fn gst_player_signal_dispatcher_get_type() -> GType;

    //=========================================================================
    // GstPlayerVideoRenderer
    //=========================================================================
    pub fn gst_player_video_renderer_get_type() -> GType;

}
