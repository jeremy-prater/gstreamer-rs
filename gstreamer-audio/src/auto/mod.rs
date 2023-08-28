// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

mod audio_aggregator;
pub use self::audio_aggregator::AudioAggregator;

mod audio_aggregator_convert_pad;
pub use self::audio_aggregator_convert_pad::AudioAggregatorConvertPad;

mod audio_aggregator_pad;
pub use self::audio_aggregator_pad::AudioAggregatorPad;

mod audio_base_sink;
pub use self::audio_base_sink::AudioBaseSink;

mod audio_base_src;
pub use self::audio_base_src::AudioBaseSrc;

mod audio_decoder;
pub use self::audio_decoder::AudioDecoder;

mod audio_encoder;
pub use self::audio_encoder::AudioEncoder;

mod audio_filter;
pub use self::audio_filter::AudioFilter;

mod audio_sink;
pub use self::audio_sink::AudioSink;

mod audio_src;
pub use self::audio_src::AudioSrc;

mod stream_volume;
pub use self::stream_volume::StreamVolume;

mod audio_stream_align;
pub use self::audio_stream_align::AudioStreamAlign;

mod enums;
pub use self::enums::AudioDitherMethod;
pub use self::enums::AudioFormat;
pub use self::enums::AudioLayout;
pub use self::enums::AudioNoiseShapingMethod;
pub use self::enums::AudioResamplerMethod;
pub use self::enums::AudioRingBufferFormatType;
pub use self::enums::StreamVolumeFormat;

mod flags;
pub use self::flags::AudioFlags;
pub use self::flags::AudioFormatFlags;
pub use self::flags::AudioPackFlags;

pub(crate) mod traits {
    pub use super::audio_aggregator::AudioAggregatorExt;
    pub use super::audio_aggregator_convert_pad::AudioAggregatorConvertPadExt;
    pub use super::audio_aggregator_pad::AudioAggregatorPadExt;
    pub use super::audio_base_sink::AudioBaseSinkExt;
    pub use super::audio_base_src::AudioBaseSrcExt;
    pub use super::audio_decoder::AudioDecoderExt;
    pub use super::audio_encoder::AudioEncoderExt;
    pub use super::stream_volume::StreamVolumeExt;
}
