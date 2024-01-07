use glib::subclass::prelude::*;
use glib::IsA;
use gtk::glib::Cast;
use gtk::prelude::*;
use gtk::{gio, glib};

pub trait RoseApplicationExt:
    glib::object::IsClass
    + IsA<glib::Object>
    + IsA<adw::Application>
    + IsA<gtk::Application>
    + IsA<gio::Application>
    + IsA<super::Application>
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
        let this = super::imp::Application::from_obj(self.upcast_ref());
        this.dependencies
            .borrow_mut()
            .push(dependency.clone().upcast());
    }

    /// Get a dependency from the application
    fn get_dependency<T>(&self) -> T
    where
        T: IsA<glib::Object>,
    {
        self.try_get_dependency().expect(&format!(
            "Failed to get dependency {}",
            T::static_type().name()
        ))
    }

    /// Try's to get a dependency from the application
    fn try_get_dependency<T: IsA<glib::Object>>(&self) -> Option<T> {
        let this = super::imp::Application::from_obj(self.upcast_ref());
        let dependencies = this.dependencies.borrow();
        dependencies
            .iter()
            .find(|d| d.is::<T>())
            .map(|d| d.clone().downcast().expect("Failed to downcast dependency"))
    }

    /// Checks if the application has a dependency
    fn has_dependency<T>(&self) -> bool
    where
        T: IsA<glib::Object>,
    {
        let this = super::imp::Application::from_obj(self.upcast_ref());
        let dependencies = this.dependencies.borrow();
        dependencies.iter().any(|d| d.is::<T>())
    }
}

impl<T> RoseApplicationExt for T where
    T: glib::object::IsClass
        + IsA<glib::Object>
        + IsA<adw::Application>
        + IsA<gtk::Application>
        + IsA<gio::Application>
        + IsA<super::Application>
{
}
