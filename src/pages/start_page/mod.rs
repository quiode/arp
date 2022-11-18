mod imp;

use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct StartPage(ObjectSubclass<imp::StartPage>)
    @extends gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl StartPage {
    pub fn new() -> Self {
        Object::new(&[])
    }
}

impl Default for StartPage {
    fn default() -> Self {
        Self::new()
    }
}
