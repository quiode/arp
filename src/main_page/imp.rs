use std::cell::RefCell;

use gtk::gio::Settings;
use gtk::glib::variant::ObjectPath;
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use once_cell::unsync::OnceCell;

use crate::package_manager::Repository;
use crate::APP_ID;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/main_page.ui")]
pub struct MainPage {
    repository: RefCell<Repository>,
    settings: OnceCell<Settings>,
}

#[glib::object_subclass]
impl ObjectSubclass for MainPage {
    const NAME: &'static str = "MainPage";
    type Type = super::MainPage;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MainPage {
    fn constructed(&self) {
        let settings = Settings::new(APP_ID);

        let path = settings.get::<Option<ObjectPath>>("project-path");

        if let Some(path) = path {
            // TODO: good error handling
            self.repository
                .try_borrow_mut()
                .unwrap()
                .load_path(path.as_str());
        }

        settings.connect_changed(
            Some("project-path"),
            clone!(@weak self as main_page => move |settings, key| {
                let path: Option<ObjectPath> = settings.get(key);
                if let Some(path) = path {
                    main_page.repository
                        .try_borrow_mut()
                        .unwrap()
                        .load_path(path.as_str());
                }
            }),
        );

        self.settings
            .set(settings)
            .expect("Settings should only be set once!");
    }

    fn dispose(&self) {
        if let Ok(repo) = self.repository.try_borrow() {
            repo.save_data().ok();
        }
    }
}

impl WidgetImpl for MainPage {}

impl BoxImpl for MainPage {}
