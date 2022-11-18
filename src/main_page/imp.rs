use std::cell::RefCell;
use std::collections::HashMap;

use adw::traits::MessageDialogExt;
use adw::{ MessageDialog, ToastOverlay, Toast };
use gtk::gio::{ Settings, SimpleAction, SimpleActionGroup };
use gtk::glib::variant::ObjectPath;
use gtk::glib::{ self, clone, Variant, VariantTy };
use gtk::subclass::prelude::*;
use gtk::{ CompositeTemplate, Expander, ComboBoxText };
use gtk::{ prelude::*, Window };
use once_cell::unsync::OnceCell;

use crate::entry::Entry;
use crate::list::List;
use crate::package_manager::{ Repository, RepositoryError };
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
    source_file: TemplateChild<Entry>,
    #[template_child]
    sources: TemplateChild<List>,
    #[template_child]
    noextract: TemplateChild<List>,
    #[template_child]
    pgpkeys: TemplateChild<List>,
    #[template_child]
    md5: TemplateChild<List>,
    #[template_child]
    md5_key: TemplateChild<Entry>,
    #[template_child]
    package_type: TemplateChild<ComboBoxText>,
    #[template_child]
    toast_overlay: TemplateChild<ToastOverlay>,
    #[template_child]
    maintainer_expander: TemplateChild<Expander>,
    #[template_child]
    name_expander: TemplateChild<Expander>,
    #[template_child]
    version_expander: TemplateChild<Expander>,
    #[template_child]
    generic_expander: TemplateChild<Expander>,
    #[template_child]
    depend_expander: TemplateChild<Expander>,
    #[template_child]
    pkgrel_expander: TemplateChild<Expander>,
    #[template_child]
    others_expander: TemplateChild<Expander>,
    #[template_child]
    sources_expander: TemplateChild<Expander>,
    #[template_child]
    integrity_expander: TemplateChild<Expander>,
    #[template_child]
    scripts_expander: TemplateChild<Expander>,
}

impl MainPage {
    // gets the values from the repository and applies them to the widgets
    fn populate_widgets(&self) {
        let data = &self.repository.borrow().data;

        self.maintainer_name.set_property(
            "content",
            data.username.clone().or(Some("".to_string())).unwrap()
        );
        self.maintainer_email.set_property(
            "content",
            data.email.clone().or(Some("".to_string())).unwrap()
        );
        self.package_name.set_property(
            "content",
            data.name.clone().or(Some("".to_string())).unwrap()
        );
        self.package_version.set_property(
            "content",
            data.version.clone().or(Some("".to_string())).unwrap()
        );
        self.release_number.set_property(
            "content",
            data.rel.clone().or(Some("".to_string())).unwrap()
        );
        self.epoch.set_property("content", data.epoch.clone().or(Some("".to_string())).unwrap());
        self.description.set_property(
            "content",
            data.desc.clone().or(Some("".to_string())).unwrap()
        );
        self.architectures.set_property("data", data.arch.clone().to_variant());
        self.url.set_property("content", data.url.clone().or(Some("".to_string())).unwrap());
        self.license.set_property("data", data.license.clone().to_variant());
        self.groups.set_property("data", data.groups.clone().to_variant());
        self.dependencies.set_property("data", data.depends.clone().to_variant());
        self.makedependencies.set_property("data", data.makedepends.clone().to_variant());
        self.checkdependencies.set_property("data", data.checkdepends.clone().to_variant());
        self.optdependencies.set_property("data", data.optdepends.clone().to_variant());
        self.provides.set_property("data", data.provides.clone().to_variant());
        self.conflicts.set_property("data", data.conflicts.clone().to_variant());
        self.replaces.set_property("data", data.replaces.clone().to_variant());
        self.backup.set_property("data", data.backup.clone().to_variant());
        self.options.set_property("data", data.options.clone().to_variant());
        self.install.set_property(
            "content",
            data.install.clone().or(Some("".to_string())).unwrap()
        );
        self.changelog.set_property(
            "content",
            data.changelog.clone().or(Some("".to_string())).unwrap()
        );
        self.sources.set_property("data", data.source.clone().to_variant());
        self.pgpkeys.set_property("data", data.pgpkeys.clone().to_variant());
        self.md5.set_property("data", data.md5sums.clone().to_variant());
        self.package_type.set_active(Some(data.package_type as u32));
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
        data.arch = self.architectures
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.url = Some(self.url.property("content"));
        data.license = self.license
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.groups = self.groups
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.depends = self.dependencies
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.makedepends = self.makedependencies
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.checkdepends = self.checkdependencies
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.optdepends = self.optdependencies
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.provides = self.provides
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.conflicts = self.conflicts
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.replaces = self.replaces
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.backup = self.backup
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.options = self.options
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.install = Some(self.install.property("content"));
        data.changelog = Some(self.changelog.property("content"));
        data.source = self.sources
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.noextract = self.noextract
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.pgpkeys = self.pgpkeys
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        data.md5sums = self.md5
            .property::<Variant>("data")
            .get()
            .expect("Value needs to be of type `Vec<String>`!");
        if let Some(id) = self.package_type.active() {
            if let Some(package_type) = num::FromPrimitive::from_u32(id) {
                data.package_type = package_type;
            }
        }
    }

    // sets state of all expanders
    fn set_expanded(&self, expanded: bool){
        self.get_expanders().values().for_each(|expander| expander.set_expanded(expanded));
    }

    // gets all expanders which are expanded
    fn get_expanded(&self) -> Vec<String>{
       let mut expanded = Vec::new(); 
       for (expander_name, expander) in self.get_expanders().iter(){
            if expander.is_expanded(){
                expanded.push(expander_name.to_string());
            }
       }
       
       expanded
    }

    // sets all expanders included in expanded list to expanded, all others not
    fn set_expanded_expanders(&self, expanded: Vec<String>){
        let expanders = self.get_expanders();
        for (name, expander) in expanders{
            if expanded.contains(&name.to_string()){
                expander.set_expanded(true);
            } else{
                expander.set_expanded(false);
            }
        }
    }

    // gets all expanders
    fn get_expanders(&self) -> HashMap<&str, &TemplateChild<Expander>>{
        let mut hasmap = HashMap::new();
        hasmap.insert("maintainer_expander", &self.maintainer_expander);
        hasmap.insert("name_expander", &self.name_expander);
        hasmap.insert("version_expander", &self.version_expander);
        hasmap.insert("generic_expander", &self.generic_expander);
        hasmap.insert("depend_expander", &self.depend_expander);
        hasmap.insert("pkgrel_expander", &self.pkgrel_expander);
        hasmap.insert("others_expander", &self.others_expander);
        hasmap.insert("sources_expander", &self.sources_expander);
        hasmap.insert("integrity_expander", &self.integrity_expander);
        hasmap.insert("scripts_expander", &self.scripts_expander);

        hasmap
    }


    // updates settings with all expanded expanders
    fn update_settings_expanders(&self, settings: &Settings){
        let expanded = self.get_expanded();
        settings.set("opened-expanders", &expanded);
    }

    // applies settings expanded to expanders
    fn update_expanders_settings(&self, settings: &Settings){
        let expanded = settings.get::<Vec<String>>("opened-expanders");
        self.set_expanded_expanders(expanded);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for MainPage {
    const NAME: &'static str = "MainPage";
    type Type = super::MainPage;
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
impl MainPage {
    #[template_callback]
    fn handle_type_changed(&self, dropdown: &ComboBoxText) {
        if let Ok(mut repo) = self.repository.try_borrow_mut() {
            if let Some(id) = dropdown.active() {
                if id != (repo.data.package_type as u32) {
                    // package type has changed
                    if let Some(package_type) = num::FromPrimitive::from_u32(id) {
                        repo.data.package_type = package_type;

                        match package_type {
                            crate::package_manager::PackageType::Binary => {
                                self.scripts_expander.hide();
                                self.sources.hide();
                                self.noextract.hide();
                                self.pgpkeys.hide();
                                self.md5.hide();

                                self.source_file.show();
                                self.md5_key.show();
                            }
                            crate::package_manager::PackageType::Make => println!("TODO"),
                            crate::package_manager::PackageType::Cargo => println!("TODO"),
                            crate::package_manager::PackageType::Custom => println!("TODO"),
                        }
                    }
                }
            }
        }
    }
}

impl ObjectImpl for MainPage {
    fn constructed(&self) {
        // load repo on settings change, set expanded state
        let settings = Settings::new(APP_ID);

        // set expanded state
        self.update_expanders_settings(&settings);

        let path = settings.get::<Option<ObjectPath>>("project-path");

        if let Some(path) = path {
            // TODO: good error handling
            if let Err(err) = self.repository.try_borrow_mut().unwrap().load_path(path.as_str()) {
                settings.set("project-path", &None::<ObjectPath>);
                println!("{}", err);
            }

            self.populate_widgets();
        }

        settings.connect_changed(
            Some("project-path"),
            clone!(@weak self as main_page => move |settings, key| {
                // update expanders
                main_page.update_expanders_settings(settings);

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
            })
        );

        self.settings.set(settings).expect("Settings should only be set once!");

        // register actions
        let repo_actions = SimpleActionGroup::new();

        let delete_action = SimpleAction::new("delete", None);
        delete_action.connect_activate(
            clone!(@weak self as main_page => move |_action, _param|{
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
        })
        );

        let save_action = SimpleAction::new("save", None);
        save_action.connect_activate(clone!(@weak self as main_window => move |_action, _param|{
            // save expanders state
            if let Some(settings) = main_window.settings.get(){
                main_window.update_settings_expanders(settings);
                }
            main_window.save_widget_sate();
            let toast = match main_window.repository.borrow().save_data(){
                Ok(_) => Toast::new("Project Saved"),
                Err(_) => Toast::new("Error trying to save project!"),
                };

            toast.set_timeout(1);
            
            main_window.toast_overlay.add_toast(&toast);
        })
        );

        let publish_action = SimpleAction::new("publish", None);
        publish_action.connect_activate(
            clone!(@weak self as main_window => move |_action, _param|{
            // safe window state
            main_window.save_widget_sate();
            if let Err(_) = main_window.repository.borrow().save_data(){
                let toast = Toast::new("Error trying to save project!");
                toast.set_timeout(1);
                main_window.toast_overlay.add_toast(&toast);
                return;
            };

            let toast = match main_window.repository.borrow().publish(){
                Err(RepositoryError::DataNotProvied) => Toast::new("Not all required data set!"),
                Err(_) => Toast::new("Error trying to publish project!"),
                Ok(_) => Toast::new("Project published"),

            };

            main_window.toast_overlay.add_toast(&toast);
        })
        );

        let clear_action = SimpleAction::new("clear", None);
        clear_action.connect_activate(
            clone!(@weak self as main_window => move |_action, _param|{
            let toast: Toast;
            if let Ok(mut repo) = main_window.repository.try_borrow_mut(){
                repo.clear();
                drop(repo);
                main_window.populate_widgets();
                toast = Toast::new("Data cleared!");
                toast.set_timeout(1);
            } else{
                toast = Toast::new("Error while clearing data!");
            }

            main_window.toast_overlay.add_toast(&toast);
        })
        );

        let toggle_expander_action = SimpleAction::new("open_expander", Some(VariantTy::BOOLEAN));
        toggle_expander_action.connect_activate(
            clone!(@weak self as main_window => move |_action, param|{
            if let Some(param) = param{
                if let Some(param) = param.get::<bool>(){
                    main_window.set_expanded(param);
                }
            }
        })
        );

        repo_actions.add_action(&delete_action);
        repo_actions.add_action(&save_action);
        repo_actions.add_action(&publish_action);
        repo_actions.add_action(&clear_action);
        repo_actions.add_action(&toggle_expander_action);
        self.instance().insert_action_group("repo", Some(&repo_actions));
    }

    fn dispose(&self) {
        // save expanders state
        if let Some(settings) = self.settings.get(){
            self.update_settings_expanders(settings);
        }
        self.save_widget_sate();
        if let Ok(repo) = self.repository.try_borrow() {
            repo.save_data().ok();
        }
    }
}

impl WidgetImpl for MainPage {}

impl BoxImpl for MainPage {}