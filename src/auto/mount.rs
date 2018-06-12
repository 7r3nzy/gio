// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Drive;
use Error;
use File;
use Icon;
use MountMountFlags;
use MountOperation;
use MountUnmountFlags;
use Volume;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Mount(Object<ffi::GMount, ffi::GMountIface>);

    match fn {
        get_type => || ffi::g_mount_get_type(),
    }
}

pub trait MountExt: Sized {
    fn can_eject(&self) -> bool;

    fn can_unmount(&self) -> bool;

    #[deprecated]
    fn eject<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, cancellable: P, callback: Q);

    #[deprecated]
    #[cfg(feature = "futures")]
    fn eject_future(&self, flags: MountUnmountFlags) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    fn eject_with_operation<'a, 'b, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, mount_operation: P, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn eject_with_operation_future<'a, P: Into<Option<&'a MountOperation>>>(&self, flags: MountUnmountFlags, mount_operation: P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    fn get_default_location(&self) -> Option<File>;

    fn get_drive(&self) -> Option<Drive>;

    fn get_icon(&self) -> Option<Icon>;

    fn get_name(&self) -> Option<String>;

    fn get_root(&self) -> Option<File>;

    fn get_sort_key(&self) -> Option<String>;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_symbolic_icon(&self) -> Option<Icon>;

    fn get_uuid(&self) -> Option<String>;

    fn get_volume(&self) -> Option<Volume>;

    fn guess_content_type<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<Vec<String>, Error>) + Send + 'static>(&self, force_rescan: bool, cancellable: P, callback: Q);

    #[cfg(feature = "futures")]
    fn guess_content_type_future(&self, force_rescan: bool) -> Box_<futures_core::Future<Item = (Self, Vec<String>), Error = (Self, Error)>>;

    fn guess_content_type_sync<'a, P: Into<Option<&'a Cancellable>>>(&self, force_rescan: bool, cancellable: P) -> Result<Vec<String>, Error>;

    fn is_shadowed(&self) -> bool;

    fn remount<'a, 'b, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountMountFlags, mount_operation: P, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn remount_future<'a, P: Into<Option<&'a MountOperation>>>(&self, flags: MountMountFlags, mount_operation: P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    fn shadow(&self);

    #[deprecated]
    fn unmount<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, cancellable: P, callback: Q);

    #[deprecated]
    #[cfg(feature = "futures")]
    fn unmount_future(&self, flags: MountUnmountFlags) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    fn unmount_with_operation<'a, 'b, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, mount_operation: P, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn unmount_with_operation_future<'a, P: Into<Option<&'a MountOperation>>>(&self, flags: MountUnmountFlags, mount_operation: P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>>;

    fn unshadow(&self);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_pre_unmount<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_unmounted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Mount> + IsA<glib::object::Object> + Clone + 'static> MountExt for O {
    fn can_eject(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mount_can_eject(self.to_glib_none().0))
        }
    }

    fn can_unmount(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mount_can_unmount(self.to_glib_none().0))
        }
    }

    fn eject<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn eject_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::g_mount_eject_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = eject_trampoline::<Q>;
        unsafe {
            ffi::g_mount_eject(self.to_glib_none().0, flags.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn eject_future(&self, flags: MountUnmountFlags) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.eject(
                 flags,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn eject_with_operation<'a, 'b, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, mount_operation: P, cancellable: Q, callback: R) {
        let mount_operation = mount_operation.into();
        let mount_operation = mount_operation.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn eject_with_operation_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::g_mount_eject_with_operation_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = eject_with_operation_trampoline::<R>;
        unsafe {
            ffi::g_mount_eject_with_operation(self.to_glib_none().0, flags.to_glib(), mount_operation.0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn eject_with_operation_future<'a, P: Into<Option<&'a MountOperation>>>(&self, flags: MountUnmountFlags, mount_operation: P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        let mount_operation = mount_operation.into();
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.eject_with_operation(
                 flags,
                 mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn get_default_location(&self) -> Option<File> {
        unsafe {
            from_glib_full(ffi::g_mount_get_default_location(self.to_glib_none().0))
        }
    }

    fn get_drive(&self) -> Option<Drive> {
        unsafe {
            from_glib_full(ffi::g_mount_get_drive(self.to_glib_none().0))
        }
    }

    fn get_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_mount_get_icon(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_mount_get_name(self.to_glib_none().0))
        }
    }

    fn get_root(&self) -> Option<File> {
        unsafe {
            from_glib_full(ffi::g_mount_get_root(self.to_glib_none().0))
        }
    }

    fn get_sort_key(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mount_get_sort_key(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_symbolic_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_mount_get_symbolic_icon(self.to_glib_none().0))
        }
    }

    fn get_uuid(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_mount_get_uuid(self.to_glib_none().0))
        }
    }

    fn get_volume(&self) -> Option<Volume> {
        unsafe {
            from_glib_full(ffi::g_mount_get_volume(self.to_glib_none().0))
        }
    }

    fn guess_content_type<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<Vec<String>, Error>) + Send + 'static>(&self, force_rescan: bool, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn guess_content_type_trampoline<Q: FnOnce(Result<Vec<String>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let ret = ffi::g_mount_guess_content_type_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = guess_content_type_trampoline::<Q>;
        unsafe {
            ffi::g_mount_guess_content_type(self.to_glib_none().0, force_rescan.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn guess_content_type_future(&self, force_rescan: bool) -> Box_<futures_core::Future<Item = (Self, Vec<String>), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.guess_content_type(
                 force_rescan,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn guess_content_type_sync<'a, P: Into<Option<&'a Cancellable>>>(&self, force_rescan: bool, cancellable: P) -> Result<Vec<String>, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_mount_guess_content_type_sync(self.to_glib_none().0, force_rescan.to_glib(), cancellable.0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_shadowed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mount_is_shadowed(self.to_glib_none().0))
        }
    }

    fn remount<'a, 'b, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountMountFlags, mount_operation: P, cancellable: Q, callback: R) {
        let mount_operation = mount_operation.into();
        let mount_operation = mount_operation.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn remount_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::g_mount_remount_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = remount_trampoline::<R>;
        unsafe {
            ffi::g_mount_remount(self.to_glib_none().0, flags.to_glib(), mount_operation.0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn remount_future<'a, P: Into<Option<&'a MountOperation>>>(&self, flags: MountMountFlags, mount_operation: P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        let mount_operation = mount_operation.into();
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.remount(
                 flags,
                 mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn shadow(&self) {
        unsafe {
            ffi::g_mount_shadow(self.to_glib_none().0);
        }
    }

    fn unmount<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn unmount_trampoline<Q: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::g_mount_unmount_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = unmount_trampoline::<Q>;
        unsafe {
            ffi::g_mount_unmount(self.to_glib_none().0, flags.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn unmount_future(&self, flags: MountUnmountFlags) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.unmount(
                 flags,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn unmount_with_operation<'a, 'b, P: Into<Option<&'a MountOperation>>, Q: Into<Option<&'b Cancellable>>, R: FnOnce(Result<(), Error>) + Send + 'static>(&self, flags: MountUnmountFlags, mount_operation: P, cancellable: Q, callback: R) {
        let mount_operation = mount_operation.into();
        let mount_operation = mount_operation.to_glib_none();
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn unmount_with_operation_trampoline<R: FnOnce(Result<(), Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let _ = ffi::g_mount_unmount_with_operation_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = unmount_with_operation_trampoline::<R>;
        unsafe {
            ffi::g_mount_unmount_with_operation(self.to_glib_none().0, flags.to_glib(), mount_operation.0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn unmount_with_operation_future<'a, P: Into<Option<&'a MountOperation>>>(&self, flags: MountUnmountFlags, mount_operation: P) -> Box_<futures_core::Future<Item = (Self, ()), Error = (Self, Error)>> {
        use GioFuture;
        use send_cell::SendCell;

        let mount_operation = mount_operation.into();
        let mount_operation = mount_operation.map(ToOwned::to_owned);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = SendCell::new(send);
            let obj_clone = SendCell::new(obj.clone());
            obj.unmount_with_operation(
                 flags,
                 mount_operation.as_ref().map(::std::borrow::Borrow::borrow),
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }

    fn unshadow(&self) {
        unsafe {
            ffi::g_mount_unshadow(self.to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_pre_unmount<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "pre-unmount",
                transmute(pre_unmount_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_unmounted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "unmounted",
                transmute(unmounted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GMount, f: glib_ffi::gpointer)
where P: IsA<Mount> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Mount::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn pre_unmount_trampoline<P>(this: *mut ffi::GMount, f: glib_ffi::gpointer)
where P: IsA<Mount> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Mount::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn unmounted_trampoline<P>(this: *mut ffi::GMount, f: glib_ffi::gpointer)
where P: IsA<Mount> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Mount::from_glib_borrow(this).downcast_unchecked())
}
