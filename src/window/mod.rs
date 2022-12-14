mod imp;

use adw::traits::MessageDialogExt;
use glib::Object;
use gtk::gio::SimpleAction;
use gtk::glib::clone;
use gtk::glib::variant::ObjectPath;
use gtk::subclass::prelude::*;
use gtk::{ gio::{ self, Settings }, glib };
use gtk::{ prelude::*, FileChooserAction, FileChooserDialog, ResponseType };

use crate::APP_ID;
use crate::package_manager::Repository;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        // Create new window
        Object::new(&[("application", app)])
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);

        // every time the path changes, check that the path is still valid, else display the placeholder page
        // WARNING: This could lead to probles if the path just changes and the whole application reloads, maybe not
        settings.connect_changed(
            Some("project-path"),
            clone!(@weak self as window => move |_settings,_key|{
                window.set_stack();
                }
            )
        );

        // read the value to the signal handler gets registered correctly
        let _path = settings.get::<Option<ObjectPath>>("project-path");

        // save settings
        self.imp()
            .settings.set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn setup_actions(&self) {
        // file-dialog action
        let action_file_dialog = SimpleAction::new(
            "file-dialog",
            Some(&bool::static_variant_type())
        );

        action_file_dialog.connect_activate(
            clone!(@weak self as window => move |_action, parameter| {
                window.project_location_dialog(parameter.expect("No parameter provided in file dialog action!").get::<bool>().expect("This value needs to be of type `bool`!"));
            })
        );

        self.add_action(&action_file_dialog);

        // about-window action
        let action_about_window = SimpleAction::new("about", None);

        action_about_window.connect_activate(clone!(@weak self as window => move |_action, _parameter| {
            // create about dialog
            let about_window = adw::AboutWindow::new();
            about_window.set_application_icon("arp-logo");
            about_window.set_application_name("ARP - AUR Uploader");
            about_window.set_developer_name("Dominik Schwaiger <mail@dominik-schwaiger.ch>");
            about_window.set_version("0.0.1");
            about_window.set_issue_url("https://github.com/quiode/arp/issues/new/choose");
            about_window.set_license_type(gtk::License::MitX11);
            about_window.set_comments(" GUI Application that let's the user upload packages to the AUR ");
            about_window.set_resizable(false);
            about_window.add_link("GitHub", "https://github.com/quiode/arp");

            about_window.show();
        }));

        self.add_action(&action_about_window);
    }

    fn settings(&self) -> &Settings {
        self.imp().settings.get().expect("`settings` should be set in `setup_settings`.")
    }

    // displays the no_project page if no project is selected, else displays the main page
    fn set_stack(&self) {
        let settings = self.settings();
        let stack = &self.imp().stack;

        // display placeholder page if no path is given, else display main page
        let path = settings.get::<Option<ObjectPath>>("project-path");

        match path {
            Some(_) => {
                stack.set_visible_child_name("main");
            }
            None => stack.set_visible_child_name("no-project"),
        }
    }

    fn project_location_dialog(&self, create_folder: bool) {
        let title: Option<&str>;
        let accept: &str;

        if create_folder {
            title = Some("Select project location");
            accept = "Select";
        } else {
            title = Some("Open a project");
            accept = "Open";
        }

        let file_dialog = FileChooserDialog::new(
            title,
            Some(self),
            FileChooserAction::SelectFolder,
            &[
                ("Cancel", ResponseType::Cancel),
                (accept, ResponseType::Accept),
            ]
        );

        file_dialog.connect_response(
            clone!(@weak self as window => move |file_chooser, response| {
                    if response == ResponseType::Accept {
                        if let Some(file) = file_chooser.file() {
                            // check that directory is empty
                            if create_folder {
                            if file.path().expect("Error while converting file to pathbuf!").read_dir().expect("Error while reading directory!").next().is_none() {
                                let path = file.path().unwrap();
                                let path = path.to_str().expect("Couldn't convert path to string!");
                                if let Ok(repo) = Repository::new(path){
                                    repo.save_data();
                                };
                                
                                window.settings().set("project-path", &Some(ObjectPath::try_from(path).expect("Path is not valid!"))).expect("Couldn't save path!");
                        } 
                        else {
                            window.error_dialog("Folder is not empty!");
                        }}
                    else {
                                window.settings().set("project-path", &Some(ObjectPath::try_from(file.path().unwrap().to_str().expect("Couldn't convert path to string!")).expect("Path is not valid!"))).expect("Couldn't save path!");
                        }} 
                        else {
                            window.error_dialog("Couldn't open Folder!");
                        }
                    }

                window.set_visible(true);
                file_chooser.destroy();
            })
        );

        file_dialog.present();
        self.set_visible(false);
    }

    // shows error dialog with text
    fn error_dialog(&self, text: &str) {
        let dialog = adw::MessageDialog::new(Some(self), Some("An Error has occured!"), Some(text));

        dialog.add_response("ok", "Understood");
        dialog.set_response_appearance("ok", adw::ResponseAppearance::Destructive);

        dialog.connect_response(None, move |dialog, _id| {
            dialog.destroy();
        });

        dialog.present();
    }
}