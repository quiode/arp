use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/main_page.ui")]
pub struct MainPage {}

#[glib::object_subclass]
impl ObjectSubclass for MainPage {
    const NAME: &'static str = "MainPage";
    type Type = super::MainPage;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainPage {}

impl WidgetImpl for MainPage {}

impl BoxImpl for MainPage {}
