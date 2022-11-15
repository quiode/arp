use std::cell::RefCell;

use gtk::gio::Settings;
use gtk::glib::variant::ObjectPath;
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use once_cell::unsync::OnceCell;

use crate::entry::Entry;
use crate::list::List;
use crate::package_manager::Repository;
use crate::APP_ID;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/main_page.ui")]
pub struct MainPage {
    repository: RefCell<Repository>,
    settings: OnceCell<Settings>,
    #[template_child]
    maintainer_name: TemplateChild<Entry>,
    #[template_child]
    maintainer_email: TemplateChild<Entry>,
    #[template_child]
    package_name: TemplateChild<Entry>,
    #[template_child]
    package_version: TemplateChild<Entry>,
    #[template_child]
    release_number: TemplateChild<Entry>,
    #[template_child]
    epoch: TemplateChild<Entry>,
    #[template_child]
    description: TemplateChild<Entry>,
    #[template_child]
    architectures: TemplateChild<List>,
    #[template_child]
    url: TemplateChild<Entry>,
    #[template_child]
    license: TemplateChild<List>,
    #[template_child]
    groups: TemplateChild<List>,
    #[template_child]
    dependencies: TemplateChild<List>,
    #[template_child]
    makedependencies: TemplateChild<List>,
    #[template_child]
    checkdependencies: TemplateChild<List>,
    #[template_child]
    optdependencies: TemplateChild<List>,
    #[template_child]
    provides: TemplateChild<List>,
    #[template_child]
    conflicts: TemplateChild<List>,
    #[template_child]
    replaces: TemplateChild<List>,
    #[template_child]
    backup: TemplateChild<List>,
    #[template_child]
    options: TemplateChild<List>,
    #[template_child]
    install: TemplateChild<Entry>,
    #[template_child]
    changelog: TemplateChild<Entry>,
    #[template_child]
    sources: TemplateChild<List>,
    #[template_child]
    noextract: TemplateChild<List>,
    #[template_child]
    pgpkeys: TemplateChild<List>,
    #[template_child]
    md5: TemplateChild<List>,
}

impl MainPage {
    // gets the values from the repository and applies them to the widgets
    fn populate_widgets(&self) {
        todo!()
    }

    // saves the state of the widgets to the repository
    fn save_widget_sate(&self) {
        todo!()
    }
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
        // load repo on settings change
        let settings = Settings::new(APP_ID);

        let path = settings.get::<Option<ObjectPath>>("project-path");

        if let Some(path) = path {
            // TODO: good error handling
            if let Err(err) = self
                .repository
                .try_borrow_mut()
                .unwrap()
                .load_path(path.as_str())
            {
                settings.set("project-path", &None::<ObjectPath>);
                println!("{}", err);
            }
        }

        settings.connect_changed(
            Some("project-path"),
            clone!(@weak self as main_page => move |settings, key| {
                let path: Option<ObjectPath> = settings.get(key);
                if let Some(path) = path {
                    // TODO: good error handling
                    if let Err(err) = main_page
                    .repository
                    .try_borrow_mut()
                    .unwrap()
                    .load_path(path.as_str()) {
                        settings.set("project-path", &None::<ObjectPath>);
                        println!("{}",err);
                    }

                    main_page.populate_widgets();
                }
            }),
        );

        self.settings
            .set(settings)
            .expect("Settings should only be set once!");
    }

    fn dispose(&self) {
        if let Ok(repo) = self.repository.try_borrow() {
            self.save_widget_sate();
            repo.save_data().ok();
        }
    }
}

impl WidgetImpl for MainPage {}

impl BoxImpl for MainPage {}
