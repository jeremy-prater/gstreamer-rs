// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::Object;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GstTaskPool")]
    pub struct TaskPool(Object<ffi::GstTaskPool, ffi::GstTaskPoolClass>) @extends Object;

    match fn {
        type_ => || ffi::gst_task_pool_get_type(),
    }
}

impl TaskPool {
    pub const NONE: Option<&'static TaskPool> = None;

    #[doc(alias = "gst_task_pool_new")]
    pub fn new() -> TaskPool {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_task_pool_new()) }
    }
}

impl Default for TaskPool {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for TaskPool {}
unsafe impl Sync for TaskPool {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::TaskPool>> Sealed for T {}
}

pub trait TaskPoolExt: IsA<TaskPool> + sealed::Sealed + 'static {
    #[doc(alias = "gst_task_pool_cleanup")]
    fn cleanup(&self) {
        unsafe {
            ffi::gst_task_pool_cleanup(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gst_task_pool_prepare")]
    fn prepare(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::gst_task_pool_prepare(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<O: IsA<TaskPool>> TaskPoolExt for O {}
