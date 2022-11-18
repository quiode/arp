mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct ListItem(ObjectSubclass<imp::ListItem>)
    @extends gtk::ListBoxRow, gtk::Widget,
    @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl ListItem {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

impl Default for ListItem {
    fn default() -> Self {
        Self::new()
    }
}
