use gtk::glib::{clone, closure_local, ParamSpec, ParamSpecString};
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, Box, Button, Label, ListBox};
use gtk::{CompositeTemplate, Entry, LinkButton};
use once_cell::sync::Lazy;

use crate::list_item::ListItem;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/list.ui")]
pub struct List {
    #[template_child]
    label: TemplateChild<Label>,
    #[template_child]
    link_button: TemplateChild<LinkButton>,
    #[template_child]
    entry: TemplateChild<Entry>,
    #[template_child]
    list_box: TemplateChild<ListBox>,
}

#[gtk::template_callbacks]
impl List {
    #[template_callback]
    fn handle_button_clicked(&self, _button: &Button) {
        let text = self.entry.text().to_string();

        if text.is_empty() {
            return;
        }

        let new_item = ListItem::new();
        new_item.set_property("label", text);
        new_item.connect_closure(
            "delete",
            false,
            closure_local!(@weak-allow-none self as list => move |list_item: ListItem, _value: String| {
                if let Some(list) = list {
                    list.list_box.remove(&list_item);
                };
            }),
        );
        self.list_box.append(&new_item);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for List {
    const NAME: &'static str = "List";
    type Type = super::List;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for List {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("title").build(),
                ParamSpecString::builder("link").build(),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "title" => {
                let text = value.get().expect("Value needs to be of type `String`!");
                self.label.set_text(text);
                self.entry.set_placeholder_text(Some(text));
            }
            "link" => self
                .link_button
                .set_uri(value.get().expect("Value needs to be of type `String`!")),
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "title" => self.label.text().to_value(),
            "link" => self.link_button.uri().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for List {}

impl BoxImpl for List {}
