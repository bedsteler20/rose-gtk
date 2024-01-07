use gtk::{gio, glib};

pub mod ext;
mod imp;
pub mod subclass;

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends adw::Application, gtk::Application, gio::Application;
}
