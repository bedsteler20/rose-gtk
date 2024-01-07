use super::subclass::RoseApplicationImpl;
use adw::subclass::prelude::*;
use gtk::glib;
use std::cell::RefCell;

#[derive(Default, Debug)]
pub struct Application {
    pub dependencies: RefCell<Vec<glib::Object>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "RoseApplication";
    type Type = super::Application;
    type ParentType = adw::Application;
}

impl ObjectImpl for Application {
    fn constructed(&self) {
        self.parent_constructed();
    }
}
impl ApplicationImpl for Application {}
impl GtkApplicationImpl for Application {}
impl AdwApplicationImpl for Application {}
impl RoseApplicationImpl for Application {}
