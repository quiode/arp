use adw::subclass::prelude::AdwApplicationWindowImpl;
use gtk::{glib, prelude::*, subclass::prelude::*, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/window.ui")]
pub struct Window {}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MainWindow";

    type Type = super::Window;

    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
