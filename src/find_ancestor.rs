use gtk::{
    glib::{Cast, IsA},
    prelude::WidgetExt,
};

pub trait FindAncestor {
    fn find_ancestor<T>(&self) -> Option<T>
    where
        T: IsA<gtk::Widget>;
}

impl FindAncestor for gtk::Widget {
    /// Find the first ancestor of type `T` in the widget hierarchy
    /// this is just a convenience wrapper around `WidgetExt::ancestor`
    /// that downcasts the result to `T` and returns `None` if the
    /// ancestor is not found or the downcast fails.
    fn find_ancestor<T>(&self) -> Option<T>
    where
        T: IsA<gtk::Widget>,
    {
        self.ancestor(T::static_type())
            .and_then(|w| w.downcast().ok())
    }
}
