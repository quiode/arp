use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    gio::Settings, glib, prelude::*, subclass::prelude::*, Button, CompositeTemplate,
    FileChooserAction, FileChooserNative, Stack,
};
use once_cell::sync::OnceCell;

use crate::start_page::StartPage;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/window.ui")]
pub struct Window {
    pub settings: OnceCell<Settings>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MainWindow";

    type Type = super::Window;

    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        // register child templates
        StartPage::ensure_type();

        klass.bind_template();
    }

    fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        // setup
        let obj = self.obj();

        obj.setup_settings();
        obj.setup_actions();
        obj.set_stack();
    }
}

impl WidgetImpl for Window {
    fn show(&self) {
        self.parent_show();
    }
}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
