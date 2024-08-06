// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::MenuModel;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GMenuLinkIter")]
    pub struct MenuLinkIter(Object<ffi::GMenuLinkIter, ffi::GMenuLinkIterClass>);

    match fn {
        type_ => || ffi::g_menu_link_iter_get_type(),
    }
}

impl MenuLinkIter {
    pub const NONE: Option<&'static MenuLinkIter> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::MenuLinkIter>> Sealed for T {}
}

pub trait MenuLinkIterExt: IsA<MenuLinkIter> + sealed::Sealed + 'static {
    #[doc(alias = "g_menu_link_iter_get_next")]
    #[doc(alias = "get_next")]
    fn next(&self) -> Option<(glib::GString, MenuModel)> {
        unsafe {
            let mut out_link = std::ptr::null();
            let mut value = std::ptr::null_mut();
            let ret = from_glib(ffi::g_menu_link_iter_get_next(
                self.as_ref().to_glib_none().0,
                &mut out_link,
                &mut value,
            ));
            if ret {
                Some((from_glib_none(out_link), from_glib_full(value)))
            } else {
                None
            }
        }
    }
}

impl<O: IsA<MenuLinkIter>> MenuLinkIterExt for O {}
