mod entry;
mod list;
mod list_item;
mod main_page;
mod package_manager;
mod file_picker;
mod start_page;
mod window;
use gtk::gio::resources_register_include;
use gtk::prelude::*;

const APP_ID: &str = "com.github.quiode.arp";

fn main() {
    // Register and include resources
    resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // shortcuts
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);
    app.set_accels_for_action("repo.save", &["<Ctrl>S"]);
    app.set_accels_for_action("repo.publish", &["<Ctrl>P"]);

    // Run the application
    app.run();
}

fn build_ui(app: &adw::Application) {
    // Create new window and present it
    let window = window::Window::new(app);

    window.show();
}