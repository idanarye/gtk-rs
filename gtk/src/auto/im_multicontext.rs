// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use gtk_sys;
use std::fmt;
use IMContext;
use InputHints;
use InputPurpose;

glib_wrapper! {
    pub struct IMMulticontext(Object<gtk_sys::GtkIMMulticontext, gtk_sys::GtkIMMulticontextClass>) @extends IMContext;

    match fn {
        get_type => || gtk_sys::gtk_im_multicontext_get_type(),
    }
}

impl IMMulticontext {
    pub fn new() -> IMMulticontext {
        assert_initialized_main_thread!();
        unsafe { IMContext::from_glib_full(gtk_sys::gtk_im_multicontext_new()).unsafe_cast() }
    }
}

impl Default for IMMulticontext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct IMMulticontextBuilder {
    input_hints: Option<InputHints>,
    input_purpose: Option<InputPurpose>,
}

impl IMMulticontextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> IMMulticontext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref input_hints) = self.input_hints {
            properties.push(("input-hints", input_hints));
        }
        if let Some(ref input_purpose) = self.input_purpose {
            properties.push(("input-purpose", input_purpose));
        }
        let ret = glib::Object::new(IMMulticontext::static_type(), &properties)
            .expect("object new")
            .downcast::<IMMulticontext>()
            .expect("downcast");
        ret
    }

    pub fn input_hints(mut self, input_hints: InputHints) -> Self {
        self.input_hints = Some(input_hints);
        self
    }

    pub fn input_purpose(mut self, input_purpose: InputPurpose) -> Self {
        self.input_purpose = Some(input_purpose);
        self
    }
}

pub const NONE_IM_MULTICONTEXT: Option<&IMMulticontext> = None;

pub trait IMMulticontextExt: 'static {
    fn get_context_id(&self) -> Option<GString>;

    fn set_context_id(&self, context_id: &str);
}

impl<O: IsA<IMMulticontext>> IMMulticontextExt for O {
    fn get_context_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_im_multicontext_get_context_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_context_id(&self, context_id: &str) {
        unsafe {
            gtk_sys::gtk_im_multicontext_set_context_id(
                self.as_ref().to_glib_none().0,
                context_id.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for IMMulticontext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IMMulticontext")
    }
}