pub mod list_item;
mod imp;

use gtk::glib::{ self, Object };

glib::wrapper! {
    pub struct List(ObjectSubclass<imp::List>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl List {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}