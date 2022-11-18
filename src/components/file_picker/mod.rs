mod imp;

use gtk::glib::{ self, Object };

glib::wrapper! {
    pub struct FilePicker(ObjectSubclass<imp::FilePicker>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl FilePicker {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

impl Default for FilePicker {
    fn default() -> Self {
        Self::new()
    }
}