use adw::subclass::prelude::*;
use gtk::glib;

pub trait RoseRouterImpl: ObjectImpl + BinImpl + 'static {}

unsafe impl<T: RoseRouterImpl> IsSubclassable<T> for super::Router {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class.upcast_ref_mut());
    }
}