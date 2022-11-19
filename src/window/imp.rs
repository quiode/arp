use adw::{ prelude::*, subclass::prelude::* };
use gtk::{ gio::Settings, glib, CompositeTemplate, Stack };
use once_cell::sync::OnceCell;

use crate::{
    pages::{ start_page::StartPage, main_page::MainPage },
    components::{
        entry::Entry,
        list::{ List, list_item::ListItem },
        file_picker::FilePicker,
        text_editor::TextEditor,
    },
};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/pages/window.ui")]
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
        MainPage::ensure_type();
        Entry::ensure_type();
        List::ensure_type();
        ListItem::ensure_type();
        FilePicker::ensure_type();
        TextEditor::ensure_type();

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