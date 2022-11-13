mod imp;

use adw::traits::MessageDialogExt;
use glib::Object;
use gtk::gio::SimpleAction;
use gtk::glib::clone;
use gtk::glib::variant::ObjectPath;
use gtk::subclass::prelude::*;
use gtk::{
    gio::{self, Settings},
    glib,
};
use gtk::{prelude::*, FileChooserAction, FileChooserDialog, ResponseType};

use crate::APP_ID;

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
            ),
        );

        // read the value to the signal handler gets registered correctly
        let path = settings.get::<Option<ObjectPath>>("project-path");

        // save settings
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn setup_actions(&self) {
        let action_file_dialog =
            SimpleAction::new("file-dialog", Some(&bool::static_variant_type()));

        action_file_dialog.connect_activate(
            clone!(@weak self as window => move |_action, parameter| {
                window.project_location_dialog(parameter.expect("No parameter provided in file dialog action!").get::<bool>().expect("This value needs to be of type `bool`!"));
            }),
        );

        self.add_action(&action_file_dialog);
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
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

        if create_folder {
            title = Some("Create a project");
        } else {
            title = Some("Open a project");
        }

        let file_dialog = FileChooserDialog::new(
            title,
            Some(self),
            FileChooserAction::SelectFolder,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Open", ResponseType::Accept),
            ],
        );

        file_dialog.connect_response(
            clone!(@weak self as window => move |file_chooser, response| {
                // TODO: Create Error messages and don't just close the file dialog!
                    if response == ResponseType::Accept {
                        if let Some(file) = file_chooser.file() {
                            // check that directory is empty
                            if file.path().expect("Error while converting file to pathbuf!").read_dir().expect("Error while reading directory!").next().is_none(){
                            println!("{}", file.path().unwrap().to_string_lossy());
                        }
                        };
                    }

                window.set_visible(true);
                file_chooser.destroy();
            }),
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
