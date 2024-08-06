// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GAsyncResult")]
    pub struct AsyncResult(Interface<ffi::GAsyncResult, ffi::GAsyncResultIface>);

    match fn {
        type_ => || ffi::g_async_result_get_type(),
    }
}

impl AsyncResult {
    pub const NONE: Option<&'static AsyncResult> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::AsyncResult>> Sealed for T {}
}

pub trait AsyncResultExt: IsA<AsyncResult> + sealed::Sealed + 'static {
    #[doc(alias = "g_async_result_get_source_object")]
    #[doc(alias = "get_source_object")]
    fn source_object(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_full(ffi::g_async_result_get_source_object(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "g_async_result_get_user_data")]
    //#[doc(alias = "get_user_data")]
    //fn user_data(&self) -> /*Unimplemented*/Option<Basic: Pointer> {
    //    unsafe { TODO: call ffi:g_async_result_get_user_data() }
    //}

    //#[doc(alias = "g_async_result_is_tagged")]
    //fn is_tagged(&self, source_tag: /*Unimplemented*/Option<Basic: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:g_async_result_is_tagged() }
    //}

    #[doc(alias = "g_async_result_legacy_propagate_error")]
    fn legacy_propagate_error(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_async_result_legacy_propagate_error(
                self.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<O: IsA<AsyncResult>> AsyncResultExt for O {}
