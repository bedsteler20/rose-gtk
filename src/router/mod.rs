use gtk::glib;

pub mod ext;
pub mod imp;
pub mod subclass;
pub mod page_route;
pub mod error;


glib::wrapper! {
    pub struct Router(ObjectSubclass<imp::Router>) @extends gtk::Widget, adw::Bin;
}