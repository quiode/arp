use std::cell::{ RefCell };
use std::fs;
use gtk::gio::Settings;
use gtk::glib::variant::ObjectPath;
use gtk::glib::{ ParamSpec, ParamSpecString, clone };
use gtk::subclass::prelude::*;
use gtk::{ CompositeTemplate, Label, Button, FileChooserDialog, ResponseType, Window };
use gtk::{ glib, prelude::*, LinkButton };
use once_cell::sync::{ Lazy, OnceCell };

use crate::APP_ID;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/filepicker.ui")]
pub struct FilePicker {
    #[template_child]
    link_button: TemplateChild<LinkButton>,
    #[template_child]
    file_label: TemplateChild<Label>,
    #[template_child]
    label: TemplateChild<Label>,
    // #[template_child]
    // file_dialog: TemplateChild<FileChooserDialog>,
    file_path: RefCell<String>,
    file_name: RefCell<String>,
    settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for FilePicker {
    const NAME: &'static str = "FilePicker";
    type Type = super::FilePicker;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl FilePicker {
    #[template_callback]
    fn handle_file_open(&self, _button: &Button) {
        let file_dialog = FileChooserDialog::new(
            Some("Chose a file"),
            Option::<&Window>::from(&None),
            gtk::FileChooserAction::Open,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Select", ResponseType::Accept),
            ]
        );

        file_dialog.connect_response(
            clone!(@weak self as file_picker => move |dialog, response|{
                match response {
                    ResponseType::Accept => {
                       if let Some(file) = dialog.file(){
                        // copy file to repo dir
                        if let Some(path) = file_picker.settings.get_or_init(||Settings::new(APP_ID)).get::<Option<ObjectPath>>("project-path"){
                            // TODO: Error handling
                            if let Ok(file_name) = file_picker.file_name.try_borrow(){
                            if file_name.is_empty(){
                                panic!("File Name has to be set! (Dev Error)");
                            }
                            if let Ok(_) = fs::copy(file.path().unwrap(), format!("{}/{}", path.to_string(), file_name)){
                                if let Ok(mut file_path) = file_picker.file_path.try_borrow_mut(){
                                    *file_path = format!("./{}", file_name);
                                    file_picker.file_label.set_text(file_path.as_str());
                                }
                            }
                        }}
                       }
                    },
                    _ => {},                
                }

                dialog.destroy();
            })
        );

        file_dialog.present();
    }
}

impl ObjectImpl for FilePicker {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("text").build(),
                ParamSpecString::builder("link").build(),
                ParamSpecString::builder("filePath").build(),
                ParamSpecString::builder("fileName").build()
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "text" =>
                self.label.set_text(
                    &value.get::<String>().expect("Value needs to be of type `String`!")
                ),
            "filePath" => {
                let value = value.get::<String>().expect("Value needs to be of type `String`!");
                let text = if value.trim().is_empty() { "Select a file" } else { &value };
                self.file_label.set_text(text);

                if let Ok(mut file_path) = self.file_path.try_borrow_mut() {
                    *file_path = value;
                }
            }
            "link" =>
                self.link_button.set_uri(value.get().expect("Value needs to be of type `String`!")),
            "fileName" => {
                if let Ok(mut file_name) = self.file_name.try_borrow_mut() {
                    *file_name = value
                        .get::<String>()
                        .expect("Value needs to be of type `String`!");
                }
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "text" => self.label.text().to_value(),
            "filePath" => {
                if let Ok(file_path) = self.file_path.try_borrow() {
                    file_path.to_value()
                } else {
                    "".to_value()
                }
            }
            "link" => self.link_button.uri().to_value(),
            "fileName" => {
                if let Ok(file_name) = self.file_name.try_borrow() {
                    file_name.to_value()
                } else {
                    "".to_value()
                }
            }
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        // sett settings
        let settings = Settings::new(APP_ID);
    }
}

impl WidgetImpl for FilePicker {}

impl BoxImpl for FilePicker {}