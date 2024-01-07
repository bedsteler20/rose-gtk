use adw::subclass::prelude::*;
use gtk::glib;

pub trait RoseApplicationImpl: ObjectImpl + AdwApplicationImpl + 'static {}

unsafe impl<T: RoseApplicationImpl> IsSubclassable<T> for super::Application {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class.upcast_ref_mut());
    }
}
