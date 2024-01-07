use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};
use std::cell::RefCell;

mod imp {

    use super::*;

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
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends adw::Application, gtk::Application, gio::Application;
}

pub trait RoseApplicationExt:
    glib::object::IsClass
    + IsA<glib::Object>
    + IsA<adw::Application>
    + IsA<gtk::Application>
    + IsA<gio::Application>
    + IsA<Application>
{
    /// Try's to find the default application and downcasts it to `Self`
    fn try_find() -> Option<Self> {
        gio::Application::default().and_then(|app| app.downcast::<Self>().ok())
    }

    /// Find the default application and downcast it to `Self`
    fn find() -> Self {
        Self::try_find().expect("Failed to find application")
    }

    /// Add a dependency to the application
    fn add_dependency<T>(&self, dependency: &T)
    where
        T: IsA<glib::Object>,
    {
        if self.has_dependency::<T>() {
            panic!(
                "The dependency of type {} is already added",
                T::static_type().name()
            );
        }
        let this = imp::Application::from_obj(self.upcast_ref());
        this.dependencies
            .borrow_mut()
            .push(dependency.clone().upcast());
    }

    /// Get a dependency from the application
    fn get_dependency<T>(&self) -> T
    where
        T: IsA<glib::Object>,
    {
        let this = imp::Application::from_obj(self.upcast_ref());
        let dependencies = this.dependencies.borrow();
        dependencies
            .iter()
            .find(|d| d.is::<T>())
            .expect("Failed to find dependency")
            .clone()
            .downcast()
            .expect("Failed to downcast dependency")
    }

    /// Checks if the application has a dependency
    fn has_dependency<T>(&self) -> bool
    where
        T: IsA<glib::Object>,
    {
        let this = imp::Application::from_obj(self.upcast_ref());
        let dependencies = this.dependencies.borrow();
        dependencies.iter().any(|d| d.is::<T>())
    }
}

// Boilerplate to make subclassing work

impl<T> RoseApplicationExt for T where
    T: glib::object::IsClass
        + IsA<glib::Object>
        + IsA<adw::Application>
        + IsA<gtk::Application>
        + IsA<gio::Application>
        + IsA<Application>
{
}

pub trait RoseApplicationImpl: ObjectImpl + AdwApplicationImpl + 'static {}

unsafe impl<T: RoseApplicationImpl> IsSubclassable<T> for Application {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class.upcast_ref_mut());
    }
}
