// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Adjustment(Object<ffi::GtkAdjustment, ffi::GtkAdjustmentClass>);

    match fn {
        get_type => || ffi::gtk_adjustment_get_type(),
    }
}

impl Adjustment {
    pub fn new(
        value: f64,
        lower: f64,
        upper: f64,
        step_increment: f64,
        page_increment: f64,
        page_size: f64,
    ) -> Adjustment {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_adjustment_new(
                value,
                lower,
                upper,
                step_increment,
                page_increment,
                page_size,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct AdjustmentBuilder {
    lower: Option<f64>,
    page_increment: Option<f64>,
    page_size: Option<f64>,
    step_increment: Option<f64>,
    upper: Option<f64>,
    value: Option<f64>,
}

impl AdjustmentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Adjustment {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref lower) = self.lower {
            properties.push(("lower", lower));
        }
        if let Some(ref page_increment) = self.page_increment {
            properties.push(("page-increment", page_increment));
        }
        if let Some(ref page_size) = self.page_size {
            properties.push(("page-size", page_size));
        }
        if let Some(ref step_increment) = self.step_increment {
            properties.push(("step-increment", step_increment));
        }
        if let Some(ref upper) = self.upper {
            properties.push(("upper", upper));
        }
        if let Some(ref value) = self.value {
            properties.push(("value", value));
        }
        let ret = glib::Object::new(Adjustment::static_type(), &properties)
            .expect("object new")
            .downcast::<Adjustment>()
            .expect("downcast");
        ret
    }

    pub fn lower(mut self, lower: f64) -> Self {
        self.lower = Some(lower);
        self
    }

    pub fn page_increment(mut self, page_increment: f64) -> Self {
        self.page_increment = Some(page_increment);
        self
    }

    pub fn page_size(mut self, page_size: f64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn step_increment(mut self, step_increment: f64) -> Self {
        self.step_increment = Some(step_increment);
        self
    }

    pub fn upper(mut self, upper: f64) -> Self {
        self.upper = Some(upper);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
}

pub const NONE_ADJUSTMENT: Option<&Adjustment> = None;

pub trait AdjustmentExt: 'static {
    #[cfg_attr(feature = "v3_18", deprecated)]
    fn changed(&self);

    fn clamp_page(&self, lower: f64, upper: f64);

    fn configure(
        &self,
        value: f64,
        lower: f64,
        upper: f64,
        step_increment: f64,
        page_increment: f64,
        page_size: f64,
    );

    fn get_lower(&self) -> f64;

    fn get_minimum_increment(&self) -> f64;

    fn get_page_increment(&self) -> f64;

    fn get_page_size(&self) -> f64;

    fn get_step_increment(&self) -> f64;

    fn get_upper(&self) -> f64;

    fn get_value(&self) -> f64;

    fn set_lower(&self, lower: f64);

    fn set_page_increment(&self, page_increment: f64);

    fn set_page_size(&self, page_size: f64);

    fn set_step_increment(&self, step_increment: f64);

    fn set_upper(&self, upper: f64);

    fn set_value(&self, value: f64);

    #[cfg_attr(feature = "v3_18", deprecated)]
    fn value_changed(&self);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Adjustment>> AdjustmentExt for O {
    fn changed(&self) {
        unsafe {
            ffi::gtk_adjustment_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn clamp_page(&self, lower: f64, upper: f64) {
        unsafe {
            ffi::gtk_adjustment_clamp_page(self.as_ref().to_glib_none().0, lower, upper);
        }
    }

    fn configure(
        &self,
        value: f64,
        lower: f64,
        upper: f64,
        step_increment: f64,
        page_increment: f64,
        page_size: f64,
    ) {
        unsafe {
            ffi::gtk_adjustment_configure(
                self.as_ref().to_glib_none().0,
                value,
                lower,
                upper,
                step_increment,
                page_increment,
                page_size,
            );
        }
    }

    fn get_lower(&self) -> f64 {
        unsafe { ffi::gtk_adjustment_get_lower(self.as_ref().to_glib_none().0) }
    }

    fn get_minimum_increment(&self) -> f64 {
        unsafe { ffi::gtk_adjustment_get_minimum_increment(self.as_ref().to_glib_none().0) }
    }

    fn get_page_increment(&self) -> f64 {
        unsafe { ffi::gtk_adjustment_get_page_increment(self.as_ref().to_glib_none().0) }
    }

    fn get_page_size(&self) -> f64 {
        unsafe { ffi::gtk_adjustment_get_page_size(self.as_ref().to_glib_none().0) }
    }

    fn get_step_increment(&self) -> f64 {
        unsafe { ffi::gtk_adjustment_get_step_increment(self.as_ref().to_glib_none().0) }
    }

    fn get_upper(&self) -> f64 {
        unsafe { ffi::gtk_adjustment_get_upper(self.as_ref().to_glib_none().0) }
    }

    fn get_value(&self) -> f64 {
        unsafe { ffi::gtk_adjustment_get_value(self.as_ref().to_glib_none().0) }
    }

    fn set_lower(&self, lower: f64) {
        unsafe {
            ffi::gtk_adjustment_set_lower(self.as_ref().to_glib_none().0, lower);
        }
    }

    fn set_page_increment(&self, page_increment: f64) {
        unsafe {
            ffi::gtk_adjustment_set_page_increment(self.as_ref().to_glib_none().0, page_increment);
        }
    }

    fn set_page_size(&self, page_size: f64) {
        unsafe {
            ffi::gtk_adjustment_set_page_size(self.as_ref().to_glib_none().0, page_size);
        }
    }

    fn set_step_increment(&self, step_increment: f64) {
        unsafe {
            ffi::gtk_adjustment_set_step_increment(self.as_ref().to_glib_none().0, step_increment);
        }
    }

    fn set_upper(&self, upper: f64) {
        unsafe {
            ffi::gtk_adjustment_set_upper(self.as_ref().to_glib_none().0, upper);
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_adjustment_set_value(self.as_ref().to_glib_none().0, value);
        }
    }

    fn value_changed(&self) {
        unsafe {
            ffi::gtk_adjustment_value_changed(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAdjustment,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Adjustment>,
        {
            let f: &F = &*(f as *const F);
            f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_value_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn value_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAdjustment,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Adjustment>,
        {
            let f: &F = &*(f as *const F);
            f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"value-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    value_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_lower_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lower_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAdjustment,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Adjustment>,
        {
            let f: &F = &*(f as *const F);
            f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::lower\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_lower_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_page_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_increment_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAdjustment,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Adjustment>,
        {
            let f: &F = &*(f as *const F);
            f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::page-increment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_page_increment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_page_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_page_size_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAdjustment,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Adjustment>,
        {
            let f: &F = &*(f as *const F);
            f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::page-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_page_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_step_increment_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_step_increment_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAdjustment,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Adjustment>,
        {
            let f: &F = &*(f as *const F);
            f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::step-increment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_step_increment_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_upper_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_upper_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAdjustment,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Adjustment>,
        {
            let f: &F = &*(f as *const F);
            f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::upper\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_upper_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkAdjustment,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Adjustment>,
        {
            let f: &F = &*(f as *const F);
            f(&Adjustment::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Adjustment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Adjustment")
    }
}
