use gtk::glib::{self, WeakRef};

pub trait AsWeakRef<T>
where
    T: glib::ObjectExt + 'static,
{
    /// Get a weak reference to this object
    fn week_ref(&self) -> WeakRef<T>;
}

impl<T> AsWeakRef<T> for T
where
    T: glib::ObjectExt + 'static,
{
    fn week_ref(&self) -> WeakRef<T> {
        let week_ref = WeakRef::new();
        week_ref.set(Some(self));
        return week_ref;
    }
}
