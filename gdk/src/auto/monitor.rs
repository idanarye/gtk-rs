// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Display;
use crate::Rectangle;
#[cfg(any(feature = "v3_22", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
use crate::SubpixelLayout;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Monitor(Object<ffi::GdkMonitor, ffi::GdkMonitorClass>);

    match fn {
        get_type => || ffi::gdk_monitor_get_type(),
    }
}

impl Monitor {
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_display(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_geometry(&self) -> Rectangle {
        unsafe {
            let mut geometry = Rectangle::uninitialized();
            ffi::gdk_monitor_get_geometry(self.to_glib_none().0, geometry.to_glib_none_mut().0);
            geometry
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_height_mm(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_height_mm(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_manufacturer(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_manufacturer(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_model(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_monitor_get_model(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_refresh_rate(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_refresh_rate(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_scale_factor(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_scale_factor(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_subpixel_layout(&self) -> SubpixelLayout {
        unsafe { from_glib(ffi::gdk_monitor_get_subpixel_layout(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_width_mm(&self) -> i32 {
        unsafe { ffi::gdk_monitor_get_width_mm(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn get_workarea(&self) -> Rectangle {
        unsafe {
            let mut workarea = Rectangle::uninitialized();
            ffi::gdk_monitor_get_workarea(self.to_glib_none().0, workarea.to_glib_none_mut().0);
            workarea
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn is_primary(&self) -> bool {
        unsafe { from_glib(ffi::gdk_monitor_is_primary(self.to_glib_none().0)) }
    }

    pub fn get_property_display(&self) -> Option<Display> {
        unsafe {
            let mut value = Value::from_type(<Display as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"display\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `display` getter")
        }
    }

    pub fn get_property_geometry(&self) -> Option<Rectangle> {
        unsafe {
            let mut value = Value::from_type(<Rectangle as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"geometry\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `geometry` getter")
        }
    }

    pub fn get_property_height_mm(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"height-mm\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `height-mm` getter")
                .unwrap()
        }
    }

    pub fn get_property_refresh_rate(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"refresh-rate\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `refresh-rate` getter")
                .unwrap()
        }
    }

    pub fn get_property_scale_factor(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"scale-factor\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `scale-factor` getter")
                .unwrap()
        }
    }

    pub fn get_property_width_mm(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"width-mm\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `width-mm` getter")
                .unwrap()
        }
    }

    pub fn get_property_workarea(&self) -> Option<Rectangle> {
        unsafe {
            let mut value = Value::from_type(<Rectangle as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"workarea\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `workarea` getter")
        }
    }

    pub fn connect_invalidate<F: Fn(&Monitor) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn invalidate_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"invalidate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    invalidate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_geometry_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_geometry_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::geometry\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_geometry_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_height_mm_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_mm_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height-mm\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_height_mm_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn connect_property_manufacturer_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_manufacturer_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::manufacturer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_manufacturer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn connect_property_model_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_refresh_rate_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_refresh_rate_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::refresh-rate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_refresh_rate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_scale_factor_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_factor_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scale-factor\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scale_factor_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn connect_property_subpixel_layout_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_subpixel_layout_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subpixel-layout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subpixel_layout_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_width_mm_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_mm_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width-mm\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_width_mm_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_workarea_notify<F: Fn(&Monitor) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_workarea_trampoline<F: Fn(&Monitor) + 'static>(
            this: *mut ffi::GdkMonitor,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::workarea\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_workarea_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monitor")
    }
}
