mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct MainPage(ObjectSubclass<imp::MainPage>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl MainPage {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

impl Default for MainPage {
    fn default() -> Self {
        Self::new()
    }
}
