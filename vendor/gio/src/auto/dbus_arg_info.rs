// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DBusArgInfo(Shared<ffi::GDBusArgInfo>);

    match fn {
        ref => |ptr| ffi::g_dbus_arg_info_ref(ptr),
        unref => |ptr| ffi::g_dbus_arg_info_unref(ptr),
        type_ => || ffi::g_dbus_arg_info_get_type(),
    }
}
