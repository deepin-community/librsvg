// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Font, Fontset, Language};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "PangoFontsetSimple")]
    pub struct FontsetSimple(Object<ffi::PangoFontsetSimple, ffi::PangoFontsetSimpleClass>) @extends Fontset;

    match fn {
        type_ => || ffi::pango_fontset_simple_get_type(),
    }
}

impl FontsetSimple {
    #[doc(alias = "pango_fontset_simple_new")]
    pub fn new(language: &mut Language) -> FontsetSimple {
        unsafe { from_glib_full(ffi::pango_fontset_simple_new(language.to_glib_none_mut().0)) }
    }

    #[doc(alias = "pango_fontset_simple_append")]
    pub fn append(&self, font: impl IsA<Font>) {
        unsafe {
            ffi::pango_fontset_simple_append(self.to_glib_none().0, font.upcast().into_glib_ptr());
        }
    }

    #[doc(alias = "pango_fontset_simple_size")]
    pub fn size(&self) -> i32 {
        unsafe { ffi::pango_fontset_simple_size(self.to_glib_none().0) }
    }
}