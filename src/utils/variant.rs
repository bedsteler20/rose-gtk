use gtk::glib;

pub trait IsVariantTyVoidExt {
    fn is_void(&self) -> bool;
}

impl IsVariantTyVoidExt for glib::VariantTy {
    /// Checks if the type is void. When rusts `()` is serialized as a variant
    /// is is serialized as a tuple with 0 items. This function checks if the
    /// type is a tuple with 0 items and returns true if it is and false if it
    /// is not
    fn is_void(&self) -> bool {
        if self.is_tuple() {
            return self.n_items() == 0;
        } else {
            return true;
        }
    }
}
