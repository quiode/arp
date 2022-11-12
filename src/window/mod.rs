mod imp;

use glib::Object;
use gtk::glib::variant::ObjectPath;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    gio::{self, Settings},
    glib,
};

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

        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

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
}
