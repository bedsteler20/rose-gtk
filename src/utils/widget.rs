use gtk::{
    glib::{Cast, IsA},
    prelude::WidgetExt,
};

use crate::prelude::RoseApplicationExt;

trait RoseWidgetExt: IsA<gtk::Widget> {
    /// Try's to find the current widget's ancestor of type `T` and downcasts it to `T`
    fn try_find_ancestor<T: IsA<gtk::Widget>>(&self) -> Option<T> {
        self.ancestor(T::static_type())
            .and_then(|ancestor| ancestor.downcast::<T>().ok())
    }

    /// Find the current widget's ancestor of type `T` and downcasts it to `T`
    fn find_ancestor<T: IsA<gtk::Widget>>(&self) -> T {
        self.try_find_ancestor().expect("Failed to find ancestor")
    }

    /// Try's to find the current widget's ancestor of type `T` and downcasts it to `T`
    /// If the ancestor is not found, it will try to find the application and get the dependency
    /// of type `T`. If the dependency is not found, it will panic.
    /// This is useful for widgets that haven't been added to the widget tree yet.
    fn find_ancestor_or_inject<T: IsA<gtk::Widget> + IsA<gtk::glib::Object>>(&self) -> T {
        self.try_find_ancestor::<T>()
            .or_else(|| {
                crate::Application::try_find().and_then(|app| app.try_get_dependency::<T>())
            })
            .expect(&format!(
                "Failed to find ancestor or inject {}",
                T::static_type().name()
            ))
    }

    /// Show a toast notification with the given message, timeout and priority
    /// This will try to find the current widget's ancestor of type `adw::ToastOverlay`
    /// and add the toast to it.
    /// If the ancestor is not found, it will try to find the application and get the dependency
    /// of type `adw::ToastOverlay`. If the dependency is not found, it will panic.
    /// 
    /// # Example
    /// ```
    /// let overlay = adw::ToastOverlay::new();
    /// let button = gtk::Button::new();
    /// button.connect_clicked(|this| {
    ///     this.show_tost("Hello", 5000, adw::ToastPriority::Normal);
    /// });
    /// overlay.set_child(&button);
    /// ```
    fn show_tost(&self, message: &str, timeout: u32, priority: adw::ToastPriority) {
        self.find_ancestor::<adw::ToastOverlay>().add_toast(
            adw::Toast::builder()
                .title(message)
                .timeout(timeout)
                .priority(priority)
                .build(),
        );
    }
}
