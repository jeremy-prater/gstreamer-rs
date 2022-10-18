// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use glib::object::IsA;

glib::wrapper! {
    #[doc(alias = "GstVideoAggregator")]
    pub struct VideoAggregator(Object<ffi::GstVideoAggregator, ffi::GstVideoAggregatorClass>) @extends gst_base::Aggregator, gst::Element, gst::Object;

    match fn {
        type_ => || ffi::gst_video_aggregator_get_type(),
    }
}

impl VideoAggregator {
    pub const NONE: Option<&'static VideoAggregator> = None;
}

unsafe impl Send for VideoAggregator {}
unsafe impl Sync for VideoAggregator {}

pub trait VideoAggregatorExt: 'static {
    //#[cfg(any(feature = "v1_20", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    //#[doc(alias = "gst_video_aggregator_get_execution_task_pool")]
    //#[doc(alias = "get_execution_task_pool")]
    //fn execution_task_pool(&self) -> /*Ignored*/gst::TaskPool;
}

impl<O: IsA<VideoAggregator>> VideoAggregatorExt for O {
    //#[cfg(any(feature = "v1_20", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    //fn execution_task_pool(&self) -> /*Ignored*/gst::TaskPool {
    //    unsafe { TODO: call ffi:gst_video_aggregator_get_execution_task_pool() }
    //}
}
