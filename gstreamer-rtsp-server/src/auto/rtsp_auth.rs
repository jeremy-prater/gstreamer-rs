// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git)
// DO NOT EDIT

use crate::RTSPToken;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GstRTSPAuth")]
    pub struct RTSPAuth(Object<ffi::GstRTSPAuth, ffi::GstRTSPAuthClass>);

    match fn {
        type_ => || ffi::gst_rtsp_auth_get_type(),
    }
}

impl RTSPAuth {
    pub const NONE: Option<&'static RTSPAuth> = None;

    #[doc(alias = "gst_rtsp_auth_new")]
    pub fn new() -> RTSPAuth {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gst_rtsp_auth_new()) }
    }

    #[doc(alias = "gst_rtsp_auth_check")]
    pub fn check(check: &str) -> Result<(), glib::error::BoolError> {
        assert_initialized_main_thread!();
        unsafe {
            glib::result_from_gboolean!(
                ffi::gst_rtsp_auth_check(check.to_glib_none().0),
                "Check failed"
            )
        }
    }

    #[doc(alias = "gst_rtsp_auth_make_basic")]
    pub fn make_basic(user: &str, pass: &str) -> glib::GString {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_make_basic(
                user.to_glib_none().0,
                pass.to_glib_none().0,
            ))
        }
    }
}

impl Default for RTSPAuth {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for RTSPAuth {}
unsafe impl Sync for RTSPAuth {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::RTSPAuth>> Sealed for T {}
}

pub trait RTSPAuthExt: IsA<RTSPAuth> + sealed::Sealed + 'static {
    #[doc(alias = "gst_rtsp_auth_add_basic")]
    fn add_basic(&self, basic: &str, token: &RTSPToken) {
        unsafe {
            ffi::gst_rtsp_auth_add_basic(
                self.as_ref().to_glib_none().0,
                basic.to_glib_none().0,
                token.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_auth_add_digest")]
    fn add_digest(&self, user: &str, pass: &str, token: &RTSPToken) {
        unsafe {
            ffi::gst_rtsp_auth_add_digest(
                self.as_ref().to_glib_none().0,
                user.to_glib_none().0,
                pass.to_glib_none().0,
                token.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_auth_get_default_token")]
    #[doc(alias = "get_default_token")]
    fn default_token(&self) -> Option<RTSPToken> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_get_default_token(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_auth_get_realm")]
    #[doc(alias = "get_realm")]
    fn realm(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::gst_rtsp_auth_get_realm(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gst_rtsp_auth_get_supported_methods")]
    #[doc(alias = "get_supported_methods")]
    fn supported_methods(&self) -> gst_rtsp::RTSPAuthMethod {
        unsafe {
            from_glib(ffi::gst_rtsp_auth_get_supported_methods(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_auth_get_tls_authentication_mode")]
    #[doc(alias = "get_tls_authentication_mode")]
    fn tls_authentication_mode(&self) -> gio::TlsAuthenticationMode {
        unsafe {
            from_glib(ffi::gst_rtsp_auth_get_tls_authentication_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_auth_get_tls_certificate")]
    #[doc(alias = "get_tls_certificate")]
    fn tls_certificate(&self) -> Option<gio::TlsCertificate> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_get_tls_certificate(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_auth_get_tls_database")]
    #[doc(alias = "get_tls_database")]
    fn tls_database(&self) -> Option<gio::TlsDatabase> {
        unsafe {
            from_glib_full(ffi::gst_rtsp_auth_get_tls_database(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_auth_parse_htdigest")]
    fn parse_htdigest(&self, path: impl AsRef<std::path::Path>, token: &RTSPToken) -> bool {
        unsafe {
            from_glib(ffi::gst_rtsp_auth_parse_htdigest(
                self.as_ref().to_glib_none().0,
                path.as_ref().to_glib_none().0,
                token.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gst_rtsp_auth_remove_basic")]
    fn remove_basic(&self, basic: &str) {
        unsafe {
            ffi::gst_rtsp_auth_remove_basic(self.as_ref().to_glib_none().0, basic.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_rtsp_auth_remove_digest")]
    fn remove_digest(&self, user: &str) {
        unsafe {
            ffi::gst_rtsp_auth_remove_digest(self.as_ref().to_glib_none().0, user.to_glib_none().0);
        }
    }

    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "gst_rtsp_auth_set_realm")]
    fn set_realm(&self, realm: Option<&str>) {
        unsafe {
            ffi::gst_rtsp_auth_set_realm(self.as_ref().to_glib_none().0, realm.to_glib_none().0);
        }
    }

    #[doc(alias = "gst_rtsp_auth_set_supported_methods")]
    fn set_supported_methods(&self, methods: gst_rtsp::RTSPAuthMethod) {
        unsafe {
            ffi::gst_rtsp_auth_set_supported_methods(
                self.as_ref().to_glib_none().0,
                methods.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_auth_set_tls_authentication_mode")]
    fn set_tls_authentication_mode(&self, mode: gio::TlsAuthenticationMode) {
        unsafe {
            ffi::gst_rtsp_auth_set_tls_authentication_mode(
                self.as_ref().to_glib_none().0,
                mode.into_glib(),
            );
        }
    }

    #[doc(alias = "gst_rtsp_auth_set_tls_certificate")]
    fn set_tls_certificate(&self, cert: Option<&impl IsA<gio::TlsCertificate>>) {
        unsafe {
            ffi::gst_rtsp_auth_set_tls_certificate(
                self.as_ref().to_glib_none().0,
                cert.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gst_rtsp_auth_set_tls_database")]
    fn set_tls_database(&self, database: Option<&impl IsA<gio::TlsDatabase>>) {
        unsafe {
            ffi::gst_rtsp_auth_set_tls_database(
                self.as_ref().to_glib_none().0,
                database.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "accept-certificate")]
    fn connect_accept_certificate<
        F: Fn(&Self, &gio::TlsConnection, &gio::TlsCertificate, gio::TlsCertificateFlags) -> bool
            + Send
            + Sync
            + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn accept_certificate_trampoline<
            P: IsA<RTSPAuth>,
            F: Fn(&P, &gio::TlsConnection, &gio::TlsCertificate, gio::TlsCertificateFlags) -> bool
                + Send
                + Sync
                + 'static,
        >(
            this: *mut ffi::GstRTSPAuth,
            connection: *mut gio::ffi::GTlsConnection,
            peer_cert: *mut gio::ffi::GTlsCertificate,
            errors: gio::ffi::GTlsCertificateFlags,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                RTSPAuth::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(connection),
                &from_glib_borrow(peer_cert),
                from_glib(errors),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept-certificate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    accept_certificate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<RTSPAuth>> RTSPAuthExt for O {}
