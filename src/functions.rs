use gtk::glib::{IsA, self, StaticTypeExt};

use crate::{PageRoute, Application, prelude::RoseApplicationExt, Router, router::ext::RoseRouterExt, DisplayableError};

pub fn visit<T: PageRoute>(parameter: Option<T::Parameter>) {
    Application::find().get_dependency::<Router>().visit::<T>(parameter);
}

pub fn back() {
    Application::find().get_dependency::<Router>().back();
}

pub fn get_dependency<T: IsA<glib::Object>>() -> T {
    Application::find().get_dependency::<T>()
}

pub fn add_dependency<T: IsA<glib::Object>>(dependency: &T) {
    Application::find().add_dependency::<T>(dependency);
}

pub fn show_error_page<T: DisplayableError>(error: T) {
    Application::find().get_dependency::<Router>().show_error(&error);
}

pub fn init() {
    Application::ensure_type();
    Router::ensure_type();
}