use gtk::glib::{self, WeakRef};

pub trait AsWeakRefExt<T>
where
    T: glib::ObjectExt + 'static,
{
    /// Get a weak reference to this object
    fn week_ref(&self) -> WeakRef<T>;
}

impl<T> AsWeakRefExt<T> for T
where
    T: glib::ObjectExt + 'static,
{
    /// Create a weak reference to this object. This is useful for creating
    /// closures that need to be passed a object.
    ///
    /// # Example
    /// ```
    /// let label = gtk::Label::new(Some("Hello"));
    /// let button = gtk::Button::new();
    /// button.connect_clicked({
    ///     let label = label.week_ref();
    ///     move |_| {
    ///        label.upgrade().unwrap().set_text("Hello from button");
    ///     }
    /// });
    fn week_ref(&self) -> WeakRef<T> {
        let week_ref = WeakRef::new();
        week_ref.set(Some(self));
        return week_ref;
    }
}
