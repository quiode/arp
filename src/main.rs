mod start_page;
mod window;
use gtk::gio;
use gtk::gio::Settings;
use gtk::prelude::*;

const APP_ID: &str = "com.github.quiode.arp";

fn main() {
    // Register and include resources
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &adw::Application) {
    // Create new window and present it
    let window = window::Window::new(app);

    window.show();
}
