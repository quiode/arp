mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct Entry(ObjectSubclass<imp::Entry>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Entry {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new()
    }
}
