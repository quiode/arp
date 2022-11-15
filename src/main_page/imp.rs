use std::cell::RefCell;

use adw::traits::MessageDialogExt;
use adw::{ApplicationWindow, MessageDialog};
use gtk::gio::{ActionGroup, Settings, SimpleAction, SimpleActionGroup};
use gtk::glib::variant::ObjectPath;
use gtk::glib::{self, clone, Variant};
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::{prelude::*, Window};
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
        let data = &self.repository.borrow().data;

        self.maintainer_name.set_property(
            "content",
            data.username.clone().or(Some("".to_string())).unwrap(),
        );
        self.maintainer_email.set_property(
            "content",
            data.email.clone().or(Some("".to_string())).unwrap(),
        );
        self.package_name.set_property(
            "content",
            data.name.clone().or(Some("".to_string())).unwrap(),
        );
        self.package_version.set_property(
            "content",
            data.version.clone().or(Some("".to_string())).unwrap(),
        );
        self.release_number.set_property(
            "content",
            data.rel.clone().or(Some("".to_string())).unwrap(),
        );
        self.epoch.set_property(
            "content",
            data.epoch.clone().or(Some("".to_string())).unwrap(),
        );
        self.description.set_property(
            "content",
            data.desc.clone().or(Some("".to_string())).unwrap(),
        );
        self.architectures.set_property("data", data.arch.clone().to_variant());
        self.url.set_property(
            "content",
            data.url.clone().or(Some("".to_string())).unwrap(),
        );
        self.license.set_property("data", data.license.clone().to_variant());
        self.groups.set_property("data", data.groups.clone().to_variant());
        self.dependencies.set_property("data", data.depends.clone().to_variant());
        self.makedependencies
            .set_property("data", data.makedepends.clone().to_variant());
        self.checkdependencies
            .set_property("data", data.checkdepends.clone().to_variant());
        self.optdependencies
            .set_property("data", data.optdepends.clone().to_variant());
        self.provides.set_property("data", data.provides.clone().to_variant());
        self.conflicts.set_property("data", data.conflicts.clone().to_variant());
        self.replaces.set_property("data", data.replaces.clone().to_variant());
        self.backup.set_property("data", data.backup.clone().to_variant());
        self.options.set_property("data", data.options.clone().to_variant());
        self.install.set_property(
            "content",
            data.install.clone().or(Some("".to_string())).unwrap(),
        );
        self.changelog.set_property(
            "content",
            data.changelog.clone().or(Some("".to_string())).unwrap(),
        );
        self.sources.set_property("data", data.source.clone().to_variant());
        self.pgpkeys.set_property("data", data.pgpkeys.clone().to_variant());
        self.md5.set_property("data", data.md5sums.clone().to_variant());

    }

    // saves the state of the widgets to the repository
    fn save_widget_sate(&self) {
        let data = &mut self.repository.borrow_mut().data;

        data.username = Some(self.maintainer_name.property("content"));
        data.email = Some(self.maintainer_email.property("content"));
        data.name = Some(self.package_name.property("content"));
        data.version = Some(self.package_version.property("content"));
        data.rel = Some(self.release_number.property("content"));
        data.epoch = Some(self.epoch.property("content"));
        data.desc = Some(self.description.property("content"));
        data.arch = self.architectures.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.url = Some(self.url.property("content"));
        data.license = self.license.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.groups = self.groups.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.depends = self.dependencies.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.makedepends = self.makedependencies.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.checkdepends = self.checkdependencies.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.optdepends = self.optdependencies.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.provides = self.provides.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.conflicts = self.conflicts.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.replaces = self.replaces.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.backup = self.backup.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.options = self.options.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.install = Some(self.epoch.property("content"));
        data.changelog = Some(self.changelog.property("content"));
        data.source = self.sources.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.noextract = self.noextract.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.pgpkeys = self.pgpkeys.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
        data.md5sums = self.md5.property::<Variant>("data").get().expect("Value needs to be of type `Vec<String>`!");
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

            self.populate_widgets();
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

        // register actions
        let actions = SimpleActionGroup::new();

        let delete_action = SimpleAction::new("delete", None);
        delete_action.connect_activate(clone!(@weak self as main_page => move |_action, _param|{
            println!("delete");
            let dialog = MessageDialog::new(None as Option<&Window>, Some("Are you sure?"), Some(&format!("Are you sure you want to delete this directory: {} and all it's children?", main_page.repository.borrow().get_path())));
            dialog.add_response("cancel", "No");
            dialog.add_response("delete", "Yes, Delete");
            dialog.set_response_appearance("cancel", adw::ResponseAppearance::Suggested);
            dialog.set_response_appearance("delete", adw::ResponseAppearance::Destructive);
            dialog.connect_response(None, clone!(@weak main_page => move |dialog, id|{
                if id == "delete"{
                    main_page.repository.borrow().delete();
                    if let Some(settings) = main_page.settings.get(){
                        settings.set("project-path", &None::<ObjectPath>);
                    }
                } 
                dialog.destroy();
            }));

            dialog.present();
        }));

        let save_action = SimpleAction::new("save", None);

        let publish_action = SimpleAction::new("publish", None);

        actions.add_action(&delete_action);
        actions.add_action(&save_action);
        actions.add_action(&publish_action);
        self.instance().insert_action_group("repo", Some(&actions));
    }

    fn dispose(&self) {
        self.save_widget_sate();
        if let Ok(repo) = self.repository.try_borrow() {
            repo.save_data().ok();
        }
    }
}

impl WidgetImpl for MainPage {}

impl BoxImpl for MainPage {}
