// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Converter, FilterOutputStream, OutputStream, PollableOutputStream};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GConverterOutputStream")]
    pub struct ConverterOutputStream(Object<ffi::GConverterOutputStream, ffi::GConverterOutputStreamClass>) @extends FilterOutputStream, OutputStream, @implements PollableOutputStream;

    match fn {
        type_ => || ffi::g_converter_output_stream_get_type(),
    }
}

impl ConverterOutputStream {
    pub const NONE: Option<&'static ConverterOutputStream> = None;

    #[doc(alias = "g_converter_output_stream_new")]
    pub fn new(
        base_stream: &impl IsA<OutputStream>,
        converter: &impl IsA<Converter>,
    ) -> ConverterOutputStream {
        unsafe {
            OutputStream::from_glib_full(ffi::g_converter_output_stream_new(
                base_stream.as_ref().to_glib_none().0,
                converter.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ConverterOutputStream`] objects.
    ///
    /// This method returns an instance of [`ConverterOutputStreamBuilder`](crate::builders::ConverterOutputStreamBuilder) which can be used to create [`ConverterOutputStream`] objects.
    pub fn builder() -> ConverterOutputStreamBuilder {
        ConverterOutputStreamBuilder::new()
    }
}

impl Default for ConverterOutputStream {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ConverterOutputStream`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ConverterOutputStreamBuilder {
    builder: glib::object::ObjectBuilder<'static, ConverterOutputStream>,
}

impl ConverterOutputStreamBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn converter(self, converter: &impl IsA<Converter>) -> Self {
        Self {
            builder: self
                .builder
                .property("converter", converter.clone().upcast()),
        }
    }

    pub fn base_stream(self, base_stream: &impl IsA<OutputStream>) -> Self {
        Self {
            builder: self
                .builder
                .property("base-stream", base_stream.clone().upcast()),
        }
    }

    pub fn close_base_stream(self, close_base_stream: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("close-base-stream", close_base_stream),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ConverterOutputStream`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ConverterOutputStream {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ConverterOutputStream>> Sealed for T {}
}

pub trait ConverterOutputStreamExt: IsA<ConverterOutputStream> + sealed::Sealed + 'static {
    #[doc(alias = "g_converter_output_stream_get_converter")]
    #[doc(alias = "get_converter")]
    fn converter(&self) -> Converter {
        unsafe {
            from_glib_none(ffi::g_converter_output_stream_get_converter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<ConverterOutputStream>> ConverterOutputStreamExt for O {}
