use gtk::glib::subclass::Signal;
use gtk::glib::{ ParamSpec, ParamSpecString };
use gtk::subclass::prelude::*;
use gtk::{ glib, prelude::*, Button, ListBoxRow };
use gtk::{ CompositeTemplate, Label };
use once_cell::sync::Lazy;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/components/list_item.ui")]
pub struct ListItem {
    #[template_child]
    label: TemplateChild<Label>,
}

#[gtk::template_callbacks]
impl ListItem {
    #[template_callback]
    fn handle_button_clicked(&self, _button: &Button) {
        self.obj().emit_by_name::<()>("delete", &[&self.label.text()]);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for ListItem {
    const NAME: &'static str = "ListItem";
    type Type = super::ListItem;
    type ParentType = ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ListItem {
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("delete").param_types([String::static_type()]).build()]
        });
        SIGNALS.as_ref()
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(||
            vec![ParamSpecString::builder("label").build()]
        );
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "label" => self.label.set_text(value.get().expect("Value has to be of type `&str`!")),
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "label" => self.label.text().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for ListItem {}

impl ListBoxRowImpl for ListItem {}