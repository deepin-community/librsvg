// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    AsyncResult, BufferedInputStream, Cancellable, DataStreamByteOrder, DataStreamNewlineType,
    FilterInputStream, InputStream, Seekable,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GDataInputStream")]
    pub struct DataInputStream(Object<ffi::GDataInputStream, ffi::GDataInputStreamClass>) @extends BufferedInputStream, FilterInputStream, InputStream, @implements Seekable;

    match fn {
        type_ => || ffi::g_data_input_stream_get_type(),
    }
}

impl DataInputStream {
    pub const NONE: Option<&'static DataInputStream> = None;

    #[doc(alias = "g_data_input_stream_new")]
    pub fn new(base_stream: &impl IsA<InputStream>) -> DataInputStream {
        unsafe {
            from_glib_full(ffi::g_data_input_stream_new(
                base_stream.as_ref().to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DataInputStream`] objects.
    ///
    /// This method returns an instance of [`DataInputStreamBuilder`](crate::builders::DataInputStreamBuilder) which can be used to create [`DataInputStream`] objects.
    pub fn builder() -> DataInputStreamBuilder {
        DataInputStreamBuilder::new()
    }
}

impl Default for DataInputStream {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DataInputStream`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DataInputStreamBuilder {
    builder: glib::object::ObjectBuilder<'static, DataInputStream>,
}

impl DataInputStreamBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn byte_order(self, byte_order: DataStreamByteOrder) -> Self {
        Self {
            builder: self.builder.property("byte-order", byte_order),
        }
    }

    pub fn newline_type(self, newline_type: DataStreamNewlineType) -> Self {
        Self {
            builder: self.builder.property("newline-type", newline_type),
        }
    }

    pub fn buffer_size(self, buffer_size: u32) -> Self {
        Self {
            builder: self.builder.property("buffer-size", buffer_size),
        }
    }

    pub fn base_stream(self, base_stream: &impl IsA<InputStream>) -> Self {
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
    /// Build the [`DataInputStream`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DataInputStream {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DataInputStream>> Sealed for T {}
}

pub trait DataInputStreamExt: IsA<DataInputStream> + sealed::Sealed + 'static {
    #[doc(alias = "g_data_input_stream_get_byte_order")]
    #[doc(alias = "get_byte_order")]
    fn byte_order(&self) -> DataStreamByteOrder {
        unsafe {
            from_glib(ffi::g_data_input_stream_get_byte_order(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_data_input_stream_get_newline_type")]
    #[doc(alias = "get_newline_type")]
    fn newline_type(&self) -> DataStreamNewlineType {
        unsafe {
            from_glib(ffi::g_data_input_stream_get_newline_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_data_input_stream_read_byte")]
    fn read_byte(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u8, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_byte(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_data_input_stream_read_int16")]
    fn read_int16(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i16, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int16(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_data_input_stream_read_int32")]
    fn read_int32(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int32(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_data_input_stream_read_int64")]
    fn read_int64(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<i64, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int64(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_data_input_stream_read_uint16")]
    fn read_uint16(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u16, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint16(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_data_input_stream_read_uint32")]
    fn read_uint32(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u32, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint32(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_data_input_stream_read_uint64")]
    fn read_uint64(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<u64, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint64(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_data_input_stream_set_byte_order")]
    fn set_byte_order(&self, order: DataStreamByteOrder) {
        unsafe {
            ffi::g_data_input_stream_set_byte_order(
                self.as_ref().to_glib_none().0,
                order.into_glib(),
            );
        }
    }

    #[doc(alias = "g_data_input_stream_set_newline_type")]
    fn set_newline_type(&self, type_: DataStreamNewlineType) {
        unsafe {
            ffi::g_data_input_stream_set_newline_type(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
            );
        }
    }

    #[doc(alias = "byte-order")]
    fn connect_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_byte_order_trampoline<
            P: IsA<DataInputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDataInputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DataInputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::byte-order\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_byte_order_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "newline-type")]
    fn connect_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_newline_type_trampoline<
            P: IsA<DataInputStream>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GDataInputStream,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DataInputStream::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::newline-type\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_newline_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DataInputStream>> DataInputStreamExt for O {}
