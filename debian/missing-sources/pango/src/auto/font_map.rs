// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Context, Font, FontDescription, FontFamily, Fontset, Language};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "PangoFontMap")]
    pub struct FontMap(Object<ffi::PangoFontMap, ffi::PangoFontMapClass>) @implements gio::ListModel;

    match fn {
        type_ => || ffi::pango_font_map_get_type(),
    }
}

impl FontMap {
    pub const NONE: Option<&'static FontMap> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::FontMap>> Sealed for T {}
}

pub trait FontMapExt: IsA<FontMap> + sealed::Sealed + 'static {
    #[doc(alias = "pango_font_map_changed")]
    fn changed(&self) {
        unsafe {
            ffi::pango_font_map_changed(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "pango_font_map_create_context")]
    fn create_context(&self) -> Context {
        unsafe {
            from_glib_full(ffi::pango_font_map_create_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_46")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_font_map_get_family")]
    #[doc(alias = "get_family")]
    fn family(&self, name: &str) -> FontFamily {
        unsafe {
            from_glib_none(ffi::pango_font_map_get_family(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_map_get_serial")]
    #[doc(alias = "get_serial")]
    fn serial(&self) -> u32 {
        unsafe { ffi::pango_font_map_get_serial(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "pango_font_map_list_families")]
    fn list_families(&self) -> Vec<FontFamily> {
        unsafe {
            let mut families = std::ptr::null_mut();
            let mut n_families = std::mem::MaybeUninit::uninit();
            ffi::pango_font_map_list_families(
                self.as_ref().to_glib_none().0,
                &mut families,
                n_families.as_mut_ptr(),
            );
            FromGlibContainer::from_glib_container_num(families, n_families.assume_init() as _)
        }
    }

    #[doc(alias = "pango_font_map_load_font")]
    fn load_font(&self, context: &Context, desc: &FontDescription) -> Option<Font> {
        unsafe {
            from_glib_full(ffi::pango_font_map_load_font(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                desc.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_font_map_load_fontset")]
    fn load_fontset(
        &self,
        context: &Context,
        desc: &FontDescription,
        language: &Language,
    ) -> Option<Fontset> {
        unsafe {
            from_glib_full(ffi::pango_font_map_load_fontset(
                self.as_ref().to_glib_none().0,
                context.to_glib_none().0,
                desc.to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }
}

impl<O: IsA<FontMap>> FontMapExt for O {}
