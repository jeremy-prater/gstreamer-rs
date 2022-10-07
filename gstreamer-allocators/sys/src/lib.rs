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

// Constants
pub const GST_ALLOCATOR_DMABUF: *const c_char = b"dmabuf\0" as *const u8 as *const c_char;
pub const GST_ALLOCATOR_FD: *const c_char = b"fd\0" as *const u8 as *const c_char;
pub const GST_CAPS_FEATURE_MEMORY_DMABUF: *const c_char =
    b"memory:DMABuf\0" as *const u8 as *const c_char;

// Flags
pub type GstFdMemoryFlags = c_uint;
pub const GST_FD_MEMORY_FLAG_NONE: GstFdMemoryFlags = 0;
pub const GST_FD_MEMORY_FLAG_KEEP_MAPPED: GstFdMemoryFlags = 1;
pub const GST_FD_MEMORY_FLAG_MAP_PRIVATE: GstFdMemoryFlags = 2;
pub const GST_FD_MEMORY_FLAG_DONT_CLOSE: GstFdMemoryFlags = 4;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstDmaBufAllocatorClass {
    pub parent_class: GstFdAllocatorClass,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstDmaBufAllocatorClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstDmaBufAllocatorClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstFdAllocatorClass {
    pub parent_class: gst::GstAllocatorClass,
}

impl ::std::fmt::Debug for GstFdAllocatorClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstFdAllocatorClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstPhysMemoryAllocatorInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub get_phys_addr:
        Option<unsafe extern "C" fn(*mut GstPhysMemoryAllocator, *mut gst::GstMemory) -> uintptr_t>,
}

impl ::std::fmt::Debug for GstPhysMemoryAllocatorInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstPhysMemoryAllocatorInterface @ {:p}", self))
            .field("get_phys_addr", &self.get_phys_addr)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstDmaBufAllocator {
    pub parent: GstFdAllocator,
    pub _gst_reserved: [gpointer; 4],
}

impl ::std::fmt::Debug for GstDmaBufAllocator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstDmaBufAllocator @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GstFdAllocator {
    pub parent: gst::GstAllocator,
}

impl ::std::fmt::Debug for GstFdAllocator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("GstFdAllocator @ {:p}", self))
            .field("parent", &self.parent)
            .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct GstPhysMemoryAllocator {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for GstPhysMemoryAllocator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "GstPhysMemoryAllocator @ {:p}", self)
    }
}

#[link(name = "gstallocators-1.0")]
extern "C" {

    //=========================================================================
    // GstDmaBufAllocator
    //=========================================================================
    pub fn gst_dmabuf_allocator_get_type() -> GType;
    pub fn gst_dmabuf_allocator_new() -> *mut gst::GstAllocator;
    pub fn gst_dmabuf_allocator_alloc(
        allocator: *mut gst::GstAllocator,
        fd: c_int,
        size: size_t,
    ) -> *mut gst::GstMemory;
    #[cfg(any(feature = "v1_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_16")))]
    pub fn gst_dmabuf_allocator_alloc_with_flags(
        allocator: *mut gst::GstAllocator,
        fd: c_int,
        size: size_t,
        flags: GstFdMemoryFlags,
    ) -> *mut gst::GstMemory;

    //=========================================================================
    // GstFdAllocator
    //=========================================================================
    pub fn gst_fd_allocator_get_type() -> GType;
    pub fn gst_fd_allocator_new() -> *mut gst::GstAllocator;
    pub fn gst_fd_allocator_alloc(
        allocator: *mut gst::GstAllocator,
        fd: c_int,
        size: size_t,
        flags: GstFdMemoryFlags,
    ) -> *mut gst::GstMemory;

    //=========================================================================
    // GstPhysMemoryAllocator
    //=========================================================================
    pub fn gst_phys_memory_allocator_get_type() -> GType;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn gst_dmabuf_memory_get_fd(mem: *mut gst::GstMemory) -> c_int;
    pub fn gst_fd_memory_get_fd(mem: *mut gst::GstMemory) -> c_int;
    pub fn gst_is_dmabuf_memory(mem: *mut gst::GstMemory) -> gboolean;
    pub fn gst_is_fd_memory(mem: *mut gst::GstMemory) -> gboolean;
    pub fn gst_is_phys_memory(mem: *mut gst::GstMemory) -> gboolean;
    pub fn gst_phys_memory_get_phys_addr(mem: *mut gst::GstMemory) -> uintptr_t;

}
