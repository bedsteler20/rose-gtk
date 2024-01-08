use gtk::glib::{IsA, self};

use crate::{PageRoute, Application, prelude::RoseApplicationExt, Router, router::ext::RoseRouterExt};

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
