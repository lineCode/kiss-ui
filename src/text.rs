//! Widgets that can render and process text (labels, text boxes).

use widget_prelude::*;

use std::ffi::CString;
use std::ptr;

/// A static widget that renders text within its parent.
pub struct Label(IUPPtr);

impl Label {
    /// Create a label with some text. 
    pub fn new<S: Into<String>>(text: S) -> Label {
        let c_text = CString::new(text.into()).unwrap();
         unsafe {
            let ptr = ::iup_sys::IupLabel(c_text.as_ptr());
            Self::from_ptr(ptr)
        }
    }

    /// Create a blank label. The text can be set later.
    pub fn new_empty() -> Label {
        unsafe { 
            let ptr = ::iup_sys::IupLabel(ptr::null());
            Self::from_ptr(ptr)       
        }
    }

    /// Update the text of this label.
    ///
    /// ##Panics
    /// If any `WidgetStr` instances from `self.get_text()` are still reachable.
    pub fn set_text(self, text: &str) -> Self {
        self.set_str_attribute(::attrs::TITLE, text);
        self
    }

    /// Get the text of this label.
    pub fn get_text(&self) -> WidgetStr {
        self.get_str_attribute(::attrs::TITLE)
            .expect("This widget should have a text pointer even if it's empty!")
    }
}

impl_widget! { Label, "label" }

impl ::image::ImageContainer for Label {}

/// A widget that renders user-editable text.
pub struct TextBox(IUPPtr);

impl TextBox {
    /// Create a new, empty text box.
    pub fn new() -> TextBox {
        unsafe {
            let ptr = ::iup_sys::IupText(ptr::null());
            Self::from_ptr(ptr)
        }
    }

    /// Set if the text box should accept and render newline characters.
    ///
    /// If `false`, it will only be slightly taller than a line of text in the current font.
    /// If `true`, the total dimensions will be set by `set_visible_columns` and
    /// `set_visible_lines`. Text outside these bounds will be accessible with a scrollbar.
    pub fn set_multiline(self, multiline: bool) -> Self {
        self.set_bool_attribute(::attrs::MULTILINE, multiline);
        self
    }

    /// Set the rendered width of the textbox in columns (character width + padding).
    ///
    /// If the textbox is set as multiline, this will cause additional text beyond the maximum
    /// width to wrap. Otherwise, it can be scrolled only horizontally.
    pub fn set_visible_columns(self, cols: u32) -> Self {
        self.set_int_attribute(::attrs::VISIBLE_COLUMNS, cols as i32);
        self
    }

    /// Set the rendered height of the textbox in lines (character height + padding).
    ///
    /// If the textbox is set as multiline, newline characters will push text following them to the
    /// next visible line. Line counts beyond these bounds will cause a scrollbar to be shown.
    pub fn set_visible_lines(self, lines: u32) ->  Self {
        self.set_int_attribute(::attrs::VISIBLE_LINES, lines as i32);
        self
    }

    /// Set the text of this textbox.
    ///
    /// ##Panics
    /// If any `WidgetStr` instances from `self.get_text()` are still reachable.
    pub fn set_text(self, value: &str) -> Self {
        self.set_str_attribute(::attrs::VALUE, value);
        self
    }

    /// Get the text value of this textbox.
    pub fn get_text(&self) -> WidgetStr {
        self.get_str_attribute(::attrs::VALUE)
            .expect("This string should be present even if it's empty!")
    }    
}

impl_widget! { TextBox, "text" }

impl_on_value_change! { TextBox }

