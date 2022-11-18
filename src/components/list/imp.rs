use gtk::glib::{
    closure_local,
    ParamSpec,
    ParamSpecString,
    ParamSpecVariant,
    Variant,
    VariantType,
};
use gtk::subclass::prelude::*;
use gtk::{ glib, prelude::*, Box, Button, Label, ListBox, ListBoxRow };
use gtk::{ CompositeTemplate, Entry, LinkButton };
use once_cell::sync::Lazy;

use super::list_item::ListItem;

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

impl List {
    // gets the values from all the children and converts them to strings
    fn get_values(&self) -> Vec<String> {
        let children = self.get_children();

        children
            .into_iter()
            .map(|child| child.property::<String>("label"))
            .collect()
    }

    // deletes all children and creates new one for each value
    fn set_values(&self, values: Vec<String>) {
        // clear all children
        self.clear();
        for value in values {
            let new_item = self.create_list_item(&value);
            self.list_box.append(&new_item);
        }
    }

    // gets all list box items
    fn get_children(&self) -> Vec<ListBoxRow> {
        let mut i = 0;
        let mut values = vec![];
        while let Some(child) = self.list_box.row_at_index(i) {
            values.push(child);
            i += 1;
        }

        values
    }

    // removes all list box items
    fn clear(&self) {
        let children = self.get_children();
        for child in children {
            self.list_box.remove(&child);
        }
    }

    fn create_list_item(&self, label: &str) -> ListItem {
        let new_item = ListItem::new();
        new_item.set_property("label", label);
        new_item.connect_closure(
            "delete",
            false,
            closure_local!(@weak-allow-none self as list => move |list_item: ListItem, _value: String| {
                if let Some(list) = list {
                    list.list_box.remove(&list_item);
                };
            })
        );

        new_item
    }
}

#[gtk::template_callbacks]
impl List {
    #[template_callback]
    fn handle_button_clicked(&self, _button: &Button) {
        let text = self.entry.text().to_string();

        if text.is_empty() {
            return;
        }

        let new_item = self.create_list_item(&text);
        self.entry.set_text("");
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
                ParamSpecVariant::builder("data", &VariantType::from_string("as").unwrap()).build()
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
            "link" =>
                self.link_button.set_uri(value.get().expect("Value needs to be of type `String`!")),
            "data" =>
                self.set_values(
                    value
                        .get::<Variant>()
                        .expect("Value needs to be of type `Variant<Vec<String>>`!")
                        .get::<Vec<String>>()
                        .expect("Value needs to be of type `Vec<String>`")
                ),
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "title" => self.label.text().to_value(),
            "link" => self.link_button.uri().to_value(),
            "data" => self.get_values().to_variant().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for List {}

impl BoxImpl for List {}