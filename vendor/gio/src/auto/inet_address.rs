// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::SocketFamily;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GInetAddress")]
    pub struct InetAddress(Object<ffi::GInetAddress, ffi::GInetAddressClass>);

    match fn {
        type_ => || ffi::g_inet_address_get_type(),
    }
}

impl InetAddress {
    pub const NONE: Option<&'static InetAddress> = None;

    #[doc(alias = "g_inet_address_new_any")]
    pub fn new_any(family: SocketFamily) -> InetAddress {
        unsafe { from_glib_full(ffi::g_inet_address_new_any(family.into_glib())) }
    }

    #[doc(alias = "g_inet_address_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(string: &str) -> Option<InetAddress> {
        unsafe { from_glib_full(ffi::g_inet_address_new_from_string(string.to_glib_none().0)) }
    }

    #[doc(alias = "g_inet_address_new_loopback")]
    pub fn new_loopback(family: SocketFamily) -> InetAddress {
        unsafe { from_glib_full(ffi::g_inet_address_new_loopback(family.into_glib())) }
    }
}

impl fmt::Display for InetAddress {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&InetAddressExt::to_str(self))
    }
}

unsafe impl Send for InetAddress {}
unsafe impl Sync for InetAddress {}

pub trait InetAddressExt: 'static {
    #[doc(alias = "g_inet_address_equal")]
    fn equal(&self, other_address: &impl IsA<InetAddress>) -> bool;

    #[doc(alias = "g_inet_address_get_family")]
    #[doc(alias = "get_family")]
    fn family(&self) -> SocketFamily;

    #[doc(alias = "g_inet_address_get_is_any")]
    #[doc(alias = "get_is_any")]
    fn is_any(&self) -> bool;

    #[doc(alias = "g_inet_address_get_is_link_local")]
    #[doc(alias = "get_is_link_local")]
    fn is_link_local(&self) -> bool;

    #[doc(alias = "g_inet_address_get_is_loopback")]
    #[doc(alias = "get_is_loopback")]
    fn is_loopback(&self) -> bool;

    #[doc(alias = "g_inet_address_get_is_mc_global")]
    #[doc(alias = "get_is_mc_global")]
    fn is_mc_global(&self) -> bool;

    #[doc(alias = "g_inet_address_get_is_mc_link_local")]
    #[doc(alias = "get_is_mc_link_local")]
    fn is_mc_link_local(&self) -> bool;

    #[doc(alias = "g_inet_address_get_is_mc_node_local")]
    #[doc(alias = "get_is_mc_node_local")]
    fn is_mc_node_local(&self) -> bool;

    #[doc(alias = "g_inet_address_get_is_mc_org_local")]
    #[doc(alias = "get_is_mc_org_local")]
    fn is_mc_org_local(&self) -> bool;

    #[doc(alias = "g_inet_address_get_is_mc_site_local")]
    #[doc(alias = "get_is_mc_site_local")]
    fn is_mc_site_local(&self) -> bool;

    #[doc(alias = "g_inet_address_get_is_multicast")]
    #[doc(alias = "get_is_multicast")]
    fn is_multicast(&self) -> bool;

    #[doc(alias = "g_inet_address_get_is_site_local")]
    #[doc(alias = "get_is_site_local")]
    fn is_site_local(&self) -> bool;

    #[doc(alias = "g_inet_address_get_native_size")]
    #[doc(alias = "get_native_size")]
    fn native_size(&self) -> usize;

    #[doc(alias = "g_inet_address_to_string")]
    #[doc(alias = "to_string")]
    fn to_str(&self) -> glib::GString;

    //fn bytes(&self) -> /*Unimplemented*/Fundamental: Pointer;

    #[doc(alias = "is-any")]
    fn connect_is_any_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "is-link-local")]
    fn connect_is_link_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "is-loopback")]
    fn connect_is_loopback_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "is-mc-global")]
    fn connect_is_mc_global_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "is-mc-link-local")]
    fn connect_is_mc_link_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "is-mc-node-local")]
    fn connect_is_mc_node_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "is-mc-org-local")]
    fn connect_is_mc_org_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "is-mc-site-local")]
    fn connect_is_mc_site_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "is-multicast")]
    fn connect_is_multicast_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "is-site-local")]
    fn connect_is_site_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<InetAddress>> InetAddressExt for O {
    fn equal(&self, other_address: &impl IsA<InetAddress>) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_equal(
                self.as_ref().to_glib_none().0,
                other_address.as_ref().to_glib_none().0,
            ))
        }
    }

    fn family(&self) -> SocketFamily {
        unsafe {
            from_glib(ffi::g_inet_address_get_family(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_any(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_any(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_link_local(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_link_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_loopback(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_loopback(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_mc_global(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_mc_global(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_mc_link_local(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_mc_link_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_mc_node_local(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_mc_node_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_mc_org_local(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_mc_org_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_mc_site_local(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_mc_site_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_multicast(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_multicast(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_site_local(&self) -> bool {
        unsafe {
            from_glib(ffi::g_inet_address_get_is_site_local(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn native_size(&self) -> usize {
        unsafe { ffi::g_inet_address_get_native_size(self.as_ref().to_glib_none().0) }
    }

    fn to_str(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::g_inet_address_to_string(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn bytes(&self) -> /*Unimplemented*/Fundamental: Pointer {
    //    glib::ObjectExt::property(self.as_ref(), "bytes")
    //}

    fn connect_is_any_notify<F: Fn(&Self) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_any_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-any\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_any_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_link_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_link_local_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-link-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_link_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_loopback_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_loopback_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-loopback\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_loopback_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_mc_global_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_global_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-global\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_global_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_mc_link_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_link_local_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-link-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_link_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_mc_node_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_node_local_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-node-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_node_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_mc_org_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_org_local_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-org-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_org_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_mc_site_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_mc_site_local_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-mc-site-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_mc_site_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_multicast_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_multicast_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-multicast\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_multicast_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_is_site_local_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_site_local_trampoline<
            P: IsA<InetAddress>,
            F: Fn(&P) + Send + Sync + 'static,
        >(
            this: *mut ffi::GInetAddress,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InetAddress::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::is-site-local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_is_site_local_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
