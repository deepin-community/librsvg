// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::InetAddress;
use crate::InetSocketAddress;
use crate::SocketAddress;
use crate::SocketConnectable;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GProxyAddress")]
    pub struct ProxyAddress(Object<ffi::GProxyAddress, ffi::GProxyAddressClass>) @extends InetSocketAddress, SocketAddress, @implements SocketConnectable;

    match fn {
        type_ => || ffi::g_proxy_address_get_type(),
    }
}

impl ProxyAddress {
    pub const NONE: Option<&'static ProxyAddress> = None;

    #[doc(alias = "g_proxy_address_new")]
    pub fn new(
        inetaddr: &impl IsA<InetAddress>,
        port: u16,
        protocol: &str,
        dest_hostname: &str,
        dest_port: u16,
        username: Option<&str>,
        password: Option<&str>,
    ) -> ProxyAddress {
        unsafe {
            SocketAddress::from_glib_full(ffi::g_proxy_address_new(
                inetaddr.as_ref().to_glib_none().0,
                port,
                protocol.to_glib_none().0,
                dest_hostname.to_glib_none().0,
                dest_port,
                username.to_glib_none().0,
                password.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

unsafe impl Send for ProxyAddress {}
unsafe impl Sync for ProxyAddress {}

pub trait ProxyAddressExt: 'static {
    #[doc(alias = "g_proxy_address_get_destination_hostname")]
    #[doc(alias = "get_destination_hostname")]
    fn destination_hostname(&self) -> glib::GString;

    #[doc(alias = "g_proxy_address_get_destination_port")]
    #[doc(alias = "get_destination_port")]
    fn destination_port(&self) -> u16;

    #[doc(alias = "g_proxy_address_get_destination_protocol")]
    #[doc(alias = "get_destination_protocol")]
    fn destination_protocol(&self) -> glib::GString;

    #[doc(alias = "g_proxy_address_get_password")]
    #[doc(alias = "get_password")]
    fn password(&self) -> Option<glib::GString>;

    #[doc(alias = "g_proxy_address_get_protocol")]
    #[doc(alias = "get_protocol")]
    fn protocol(&self) -> glib::GString;

    #[doc(alias = "g_proxy_address_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString>;

    #[doc(alias = "g_proxy_address_get_username")]
    #[doc(alias = "get_username")]
    fn username(&self) -> Option<glib::GString>;
}

impl<O: IsA<ProxyAddress>> ProxyAddressExt for O {
    fn destination_hostname(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_destination_hostname(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn destination_port(&self) -> u16 {
        unsafe { ffi::g_proxy_address_get_destination_port(self.as_ref().to_glib_none().0) }
    }

    fn destination_protocol(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_destination_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn password(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_password(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn protocol(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_protocol(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_proxy_address_get_uri(self.as_ref().to_glib_none().0)) }
    }

    fn username(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_username(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for ProxyAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ProxyAddress")
    }
}
