mod imp;

use gtk::glib::{ self, Object };

glib::wrapper! {
    pub struct TextEditor(ObjectSubclass<imp::TextEditor>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl TextEditor {
    pub fn new() -> Self {
        Object::new(&[("language", &"sh".to_string())])
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}