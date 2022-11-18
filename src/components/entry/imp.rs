use adw::traits::PreferencesRowExt;
use adw::EntryRow;
use gtk::glib::{ ParamSpec, ParamSpecString };
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{ glib, prelude::*, LinkButton };
use once_cell::sync::Lazy;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/components/entry.ui")]
pub struct Entry {
    #[template_child]
    link_button: TemplateChild<LinkButton>,
    #[template_child]
    entry_row: TemplateChild<EntryRow>,
}

#[glib::object_subclass]
impl ObjectSubclass for Entry {
    const NAME: &'static str = "Entry";
    type Type = super::Entry;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Entry {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("text").build(),
                ParamSpecString::builder("link").build(),
                ParamSpecString::builder("content").build()
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "text" =>
                self.entry_row.set_title(value.get().expect("Value needs to be of type `String`!")),
            "content" =>
                self.entry_row.set_text(value.get().expect("Value needs to be of type `String`!")),
            "link" =>
                self.link_button.set_uri(value.get().expect("Value needs to be of type `String`!")),
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "text" => self.entry_row.title().to_value(),
            "content" => self.entry_row.text().to_value(),
            "link" => self.link_button.uri().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for Entry {}

impl BoxImpl for Entry {}