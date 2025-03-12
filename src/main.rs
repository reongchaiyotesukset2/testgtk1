use gtk::{glib};
mod application;
mod widgets;
mod models;

use application::Application;

fn main() -> glib::ExitCode {
    gtk::init().expect("failed to init gtk");

        glib::set_application_name("Window.guillaume_gomez.demo");
       
     Application::run()
}
