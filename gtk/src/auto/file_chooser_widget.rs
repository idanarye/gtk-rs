// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::BaselinePosition;
use crate::Box;
use crate::Buildable;
use crate::Container;
use crate::FileChooser;
use crate::FileChooserAction;
use crate::FileFilter;
use crate::Orientable;
use crate::Orientation;
use crate::ResizeMode;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct FileChooserWidget(Object<ffi::GtkFileChooserWidget, ffi::GtkFileChooserWidgetClass>) @extends Box, Container, Widget, @implements Buildable, Orientable, FileChooser;

    match fn {
        get_type => || ffi::gtk_file_chooser_widget_get_type(),
    }
}

impl FileChooserWidget {
    #[doc(alias = "gtk_file_chooser_widget_new")]
    pub fn new(action: FileChooserAction) -> FileChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_file_chooser_widget_new(action.to_glib())).unsafe_cast()
        }
    }
}

#[derive(Clone, Default)]
pub struct FileChooserWidgetBuilder {
    search_mode: Option<bool>,
    baseline_position: Option<BaselinePosition>,
    homogeneous: Option<bool>,
    spacing: Option<i32>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
    action: Option<FileChooserAction>,
    create_folders: Option<bool>,
    do_overwrite_confirmation: Option<bool>,
    extra_widget: Option<Widget>,
    filter: Option<FileFilter>,
    local_only: Option<bool>,
    preview_widget: Option<Widget>,
    preview_widget_active: Option<bool>,
    select_multiple: Option<bool>,
    show_hidden: Option<bool>,
    use_preview_label: Option<bool>,
}

impl FileChooserWidgetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> FileChooserWidget {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref search_mode) = self.search_mode {
            properties.push(("search-mode", search_mode));
        }
        if let Some(ref baseline_position) = self.baseline_position {
            properties.push(("baseline-position", baseline_position));
        }
        if let Some(ref homogeneous) = self.homogeneous {
            properties.push(("homogeneous", homogeneous));
        }
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        if let Some(ref action) = self.action {
            properties.push(("action", action));
        }
        if let Some(ref create_folders) = self.create_folders {
            properties.push(("create-folders", create_folders));
        }
        if let Some(ref do_overwrite_confirmation) = self.do_overwrite_confirmation {
            properties.push(("do-overwrite-confirmation", do_overwrite_confirmation));
        }
        if let Some(ref extra_widget) = self.extra_widget {
            properties.push(("extra-widget", extra_widget));
        }
        if let Some(ref filter) = self.filter {
            properties.push(("filter", filter));
        }
        if let Some(ref local_only) = self.local_only {
            properties.push(("local-only", local_only));
        }
        if let Some(ref preview_widget) = self.preview_widget {
            properties.push(("preview-widget", preview_widget));
        }
        if let Some(ref preview_widget_active) = self.preview_widget_active {
            properties.push(("preview-widget-active", preview_widget_active));
        }
        if let Some(ref select_multiple) = self.select_multiple {
            properties.push(("select-multiple", select_multiple));
        }
        if let Some(ref show_hidden) = self.show_hidden {
            properties.push(("show-hidden", show_hidden));
        }
        if let Some(ref use_preview_label) = self.use_preview_label {
            properties.push(("use-preview-label", use_preview_label));
        }
        let ret = glib::Object::new::<FileChooserWidget>(&properties).expect("object new");
        ret
    }

    pub fn search_mode(mut self, search_mode: bool) -> Self {
        self.search_mode = Some(search_mode);
        self
    }

    pub fn baseline_position(mut self, baseline_position: BaselinePosition) -> Self {
        self.baseline_position = Some(baseline_position);
        self
    }

    pub fn homogeneous(mut self, homogeneous: bool) -> Self {
        self.homogeneous = Some(homogeneous);
        self
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn action(mut self, action: FileChooserAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn create_folders(mut self, create_folders: bool) -> Self {
        self.create_folders = Some(create_folders);
        self
    }

    pub fn do_overwrite_confirmation(mut self, do_overwrite_confirmation: bool) -> Self {
        self.do_overwrite_confirmation = Some(do_overwrite_confirmation);
        self
    }

    pub fn extra_widget<P: IsA<Widget>>(mut self, extra_widget: &P) -> Self {
        self.extra_widget = Some(extra_widget.clone().upcast());
        self
    }

    pub fn filter(mut self, filter: &FileFilter) -> Self {
        self.filter = Some(filter.clone());
        self
    }

    pub fn local_only(mut self, local_only: bool) -> Self {
        self.local_only = Some(local_only);
        self
    }

    pub fn preview_widget<P: IsA<Widget>>(mut self, preview_widget: &P) -> Self {
        self.preview_widget = Some(preview_widget.clone().upcast());
        self
    }

    pub fn preview_widget_active(mut self, preview_widget_active: bool) -> Self {
        self.preview_widget_active = Some(preview_widget_active);
        self
    }

    pub fn select_multiple(mut self, select_multiple: bool) -> Self {
        self.select_multiple = Some(select_multiple);
        self
    }

    pub fn show_hidden(mut self, show_hidden: bool) -> Self {
        self.show_hidden = Some(show_hidden);
        self
    }

    pub fn use_preview_label(mut self, use_preview_label: bool) -> Self {
        self.use_preview_label = Some(use_preview_label);
        self
    }
}

pub const NONE_FILE_CHOOSER_WIDGET: Option<&FileChooserWidget> = None;

pub trait FileChooserWidgetExt: 'static {
    fn get_property_search_mode(&self) -> bool;

    fn set_property_search_mode(&self, search_mode: bool);

    fn get_property_subtitle(&self) -> Option<glib::GString>;

    fn connect_desktop_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_desktop_folder(&self);

    fn connect_down_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_down_folder(&self);

    fn connect_home_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_home_folder(&self);

    fn connect_location_popup<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_location_popup(&self, path: &str);

    fn connect_location_popup_on_paste<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_location_popup_on_paste(&self);

    fn connect_location_toggle_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_location_toggle_popup(&self);

    fn connect_places_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_places_shortcut(&self);

    fn connect_quick_bookmark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_quick_bookmark(&self, bookmark_index: i32);

    fn connect_recent_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_recent_shortcut(&self);

    fn connect_search_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_search_shortcut(&self);

    fn connect_show_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_show_hidden(&self);

    fn connect_up_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_up_folder(&self);

    fn connect_property_search_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileChooserWidget>> FileChooserWidgetExt for O {
    fn get_property_search_mode(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"search-mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `search-mode` getter")
                .unwrap()
        }
    }

    fn set_property_search_mode(&self, search_mode: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"search-mode\0".as_ptr() as *const _,
                glib::Value::from(&search_mode).to_glib_none().0,
            );
        }
    }

    fn get_property_subtitle(&self) -> Option<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<glib::GString as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"subtitle\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `subtitle` getter")
        }
    }

    fn connect_desktop_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn desktop_folder_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"desktop-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    desktop_folder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_desktop_folder(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("desktop-folder", &[])
                .unwrap()
        };
    }

    fn connect_down_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn down_folder_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"down-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    down_folder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_down_folder(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("down-folder", &[])
                .unwrap()
        };
    }

    fn connect_home_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn home_folder_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"home-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    home_folder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_home_folder(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("home-folder", &[])
                .unwrap()
        };
    }

    fn connect_location_popup<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn location_popup_trampoline<P, F: Fn(&P, &str) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(
                &FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(path),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    location_popup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_location_popup(&self, path: &str) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("location-popup", &[&path])
                .unwrap()
        };
    }

    fn connect_location_popup_on_paste<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn location_popup_on_paste_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-popup-on-paste\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    location_popup_on_paste_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_location_popup_on_paste(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("location-popup-on-paste", &[])
                .unwrap()
        };
    }

    fn connect_location_toggle_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn location_toggle_popup_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"location-toggle-popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    location_toggle_popup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_location_toggle_popup(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("location-toggle-popup", &[])
                .unwrap()
        };
    }

    fn connect_places_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn places_shortcut_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"places-shortcut\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    places_shortcut_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_places_shortcut(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("places-shortcut", &[])
                .unwrap()
        };
    }

    fn connect_quick_bookmark<F: Fn(&Self, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn quick_bookmark_trampoline<P, F: Fn(&P, i32) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            bookmark_index: libc::c_int,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(
                &FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref(),
                bookmark_index,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"quick-bookmark\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    quick_bookmark_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_quick_bookmark(&self, bookmark_index: i32) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("quick-bookmark", &[&bookmark_index])
                .unwrap()
        };
    }

    fn connect_recent_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn recent_shortcut_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"recent-shortcut\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    recent_shortcut_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_recent_shortcut(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("recent-shortcut", &[])
                .unwrap()
        };
    }

    fn connect_search_shortcut<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_shortcut_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"search-shortcut\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    search_shortcut_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_search_shortcut(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("search-shortcut", &[])
                .unwrap()
        };
    }

    fn connect_show_hidden<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn show_hidden_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"show-hidden\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    show_hidden_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_show_hidden(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("show-hidden", &[])
                .unwrap()
        };
    }

    fn connect_up_folder<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn up_folder_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"up-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    up_folder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_up_folder(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("up-folder", &[])
                .unwrap()
        };
    }

    fn connect_property_search_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkFileChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<FileChooserWidget>,
        {
            let f: &F = &*(f as *const F);
            f(&FileChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FileChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileChooserWidget")
    }
}