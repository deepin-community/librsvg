// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::InputStream;
use crate::ResourceLookupFlags;
use glib::translate::*;
use std::mem;
use std::ptr;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Resource(Shared<ffi::GResource>);

    match fn {
        ref => |ptr| ffi::g_resource_ref(ptr),
        unref => |ptr| ffi::g_resource_unref(ptr),
        type_ => || ffi::g_resource_get_type(),
    }
}

impl Resource {
    #[doc(alias = "g_resource_enumerate_children")]
    pub fn enumerate_children(
        &self,
        path: &str,
        lookup_flags: ResourceLookupFlags,
    ) -> Result<Vec<glib::GString>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resource_enumerate_children(
                self.to_glib_none().0,
                path.to_glib_none().0,
                lookup_flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_resource_get_info")]
    #[doc(alias = "get_info")]
    pub fn info(
        &self,
        path: &str,
        lookup_flags: ResourceLookupFlags,
    ) -> Result<(usize, u32), glib::Error> {
        unsafe {
            let mut size = mem::MaybeUninit::uninit();
            let mut flags = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_resource_get_info(
                self.to_glib_none().0,
                path.to_glib_none().0,
                lookup_flags.into_glib(),
                size.as_mut_ptr(),
                flags.as_mut_ptr(),
                &mut error,
            );
            let size = size.assume_init();
            let flags = flags.assume_init();
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((size, flags))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_resource_lookup_data")]
    pub fn lookup_data(
        &self,
        path: &str,
        lookup_flags: ResourceLookupFlags,
    ) -> Result<glib::Bytes, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resource_lookup_data(
                self.to_glib_none().0,
                path.to_glib_none().0,
                lookup_flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_resource_open_stream")]
    pub fn open_stream(
        &self,
        path: &str,
        lookup_flags: ResourceLookupFlags,
    ) -> Result<InputStream, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resource_open_stream(
                self.to_glib_none().0,
                path.to_glib_none().0,
                lookup_flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_resource_load")]
    pub fn load(filename: impl AsRef<std::path::Path>) -> Result<Resource, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resource_load(filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
