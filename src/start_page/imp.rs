use gtk::subclass::prelude::*;
use gtk::{glib, Button, FileChooserNative};
use gtk::{prelude::*, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/start_page.ui")]
pub struct StartPage {}

#[glib::object_subclass]
impl ObjectSubclass for StartPage {
    const NAME: &'static str = "StartPage";
    type Type = super::StartPage;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for StartPage {}

impl WidgetImpl for StartPage {}

impl BoxImpl for StartPage {}
