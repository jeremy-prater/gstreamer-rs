// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{translate::*, Cast, IsA};

use crate::Allocator;

impl Allocator {
    #[doc(alias = "gst_allocator_register")]
    pub fn register(name: &str, allocator: impl IsA<Allocator>) {
        skip_assert_initialized!();
        unsafe {
            // See https://gitlab.freedesktop.org/gstreamer/gstreamer/-/merge_requests/3364
            if crate::version() < (1, 20, 5, 0) {
                ffi::gst_allocator_register(
                    name.to_glib_full(),
                    allocator.upcast().into_glib_ptr(),
                );
            } else {
                ffi::gst_allocator_register(
                    name.to_glib_none().0,
                    allocator.upcast().into_glib_ptr(),
                );
            }
        }
    }
}
