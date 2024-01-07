mod application;
mod find_ancestor;
mod route;
mod router;
mod variant;
mod week_ref;

mod channels;

pub mod prelude {
    pub use crate::application::RoseApplicationExt;
    pub use crate::find_ancestor::FindAncestor;
    pub use crate::variant::IsVariantTyVoid;
    pub use crate::week_ref::AsWeakRef;
}

pub mod subclass {
    pub mod application {
        pub use crate::application::RoseApplicationImpl;
    }

    pub mod prelude {
        pub use crate::subclass::application::*;
    }
}

use prelude::RoseApplicationExt;

pub use crate::application::Application;
pub use crate::channels::channel;
pub use crate::channels::Receiver;
pub use crate::channels::Sender;
pub use crate::route::Route;
pub use crate::router::Router;

/// Visit a route by type
pub fn visit<R>(parameter: Option<R::Parameter>)
where
    R: Route + 'static,
{
    Application::find()
        .get_dependency::<Router>()
        .visit::<R>(parameter);
}

/// Visit a route by name
pub unsafe fn visit_unsafe(route: &str, parameter: Option<gtk::glib::Variant>) {
    Application::find()
        .get_dependency::<Router>()
        .visit_unsafe(route, parameter);
}

/// Navigate back to the previous page
pub fn back() {
    Application::find().get_dependency::<Router>().back();
}


/// Gets a dependency from the default application
pub fn inject<T>() -> T
where
    T: gtk::glib::IsA<gtk::glib::Object> + gtk::glib::StaticType + 'static,
{
    Application::find().get_dependency::<T>()
}

/// Initialize the library and register all types
/// this should be called in the main function
pub fn init() {
    use gtk::glib::StaticTypeExt;
    router::Router::ensure_type();
    application::Application::ensure_type();
}
