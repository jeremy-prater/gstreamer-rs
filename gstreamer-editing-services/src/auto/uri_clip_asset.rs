// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
use crate::SourceClipAsset;
use crate::{Asset, ClipAsset, MetaContainer, UriSourceAsset};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, mem::transmute, ptr};

#[cfg(feature = "v1_18")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
glib::wrapper! {
    #[doc(alias = "GESUriClipAsset")]
    pub struct UriClipAsset(Object<ffi::GESUriClipAsset, ffi::GESUriClipAssetClass>) @extends SourceClipAsset, ClipAsset, Asset, @implements MetaContainer;

    match fn {
        type_ => || ffi::ges_uri_clip_asset_get_type(),
    }
}

#[cfg(not(any(feature = "v1_18")))]
glib::wrapper! {
    #[doc(alias = "GESUriClipAsset")]
    pub struct UriClipAsset(Object<ffi::GESUriClipAsset, ffi::GESUriClipAssetClass>) @extends ClipAsset, Asset, @implements MetaContainer;

    match fn {
        type_ => || ffi::ges_uri_clip_asset_get_type(),
    }
}

impl UriClipAsset {
    pub const NONE: Option<&'static UriClipAsset> = None;

    //#[cfg(feature = "v1_16")]
    //#[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    //#[doc(alias = "ges_uri_clip_asset_finish")]
    //pub fn finish(res: /*Ignored*/&gio::AsyncResult) -> Result<UriClipAsset, glib::Error> {
    //    unsafe { TODO: call ffi:ges_uri_clip_asset_finish() }
    //}

    //#[doc(alias = "ges_uri_clip_asset_new")]
    //pub fn new<P: FnOnce(Result<(), glib::Error>) + 'static>(uri: &str, cancellable: Option<&impl IsA<gio::Cancellable>>, callback: P) {
    //    unsafe { TODO: call ffi:ges_uri_clip_asset_new() }
    //}

    #[doc(alias = "ges_uri_clip_asset_request_sync")]
    pub fn request_sync(uri: &str) -> Result<UriClipAsset, glib::Error> {
        assert_initialized_main_thread!();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::ges_uri_clip_asset_request_sync(uri.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

unsafe impl Send for UriClipAsset {}
unsafe impl Sync for UriClipAsset {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::UriClipAsset>> Sealed for T {}
}

pub trait UriClipAssetExt: IsA<UriClipAsset> + sealed::Sealed + 'static {
    #[doc(alias = "ges_uri_clip_asset_get_duration")]
    #[doc(alias = "get_duration")]
    fn duration(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_uri_clip_asset_get_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_uri_clip_asset_get_info")]
    #[doc(alias = "get_info")]
    fn info(&self) -> gst_pbutils::DiscovererInfo {
        unsafe {
            from_glib_none(ffi::ges_uri_clip_asset_get_info(const_override(
                self.as_ref().to_glib_none().0,
            )))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_uri_clip_asset_get_max_duration")]
    #[doc(alias = "get_max_duration")]
    fn max_duration(&self) -> Option<gst::ClockTime> {
        unsafe {
            from_glib(ffi::ges_uri_clip_asset_get_max_duration(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "ges_uri_clip_asset_get_stream_assets")]
    #[doc(alias = "get_stream_assets")]
    fn stream_assets(&self) -> Vec<UriSourceAsset> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::ges_uri_clip_asset_get_stream_assets(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "ges_uri_clip_asset_is_image")]
    fn is_image(&self) -> bool {
        unsafe {
            from_glib(ffi::ges_uri_clip_asset_is_image(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_duration(&self, duration: u64) {
        glib::ObjectExt::set_property(self.as_ref(), "duration", duration)
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "is-nested-timeline")]
    fn is_nested_timeline(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "is-nested-timeline")
    }

    #[doc(alias = "duration")]
    fn connect_duration_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_duration_trampoline<
            P: IsA<UriClipAsset>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GESUriClipAsset,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(UriClipAsset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::duration\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_duration_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "is-nested-timeline")]
    fn connect_is_nested_timeline_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_nested_timeline_trampoline<
            P: IsA<UriClipAsset>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GESUriClipAsset,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(UriClipAsset::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-nested-timeline\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_nested_timeline_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<UriClipAsset>> UriClipAssetExt for O {}
