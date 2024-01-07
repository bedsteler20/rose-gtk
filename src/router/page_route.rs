use gtk::glib::{self, IsA};

pub trait PageRoute
where
    Self: glib::ObjectExt + 'static,
    Self::Parameter: glib::FromVariant + glib::ToVariant,
{
    /// The type of the parameter that will be passed to the page
    /// this should be a tuple of the parameters that will be passed
    /// to the routes build function. The parameters need to be able
    /// to be serialized and deserialized as a Gio Variant
    type Parameter;

    /// The route that this page will be displayed at
    /// this should be a static string formatted with
    /// periods to separate the parts of the route
    /// instead of slashes
    fn route() -> &'static str;

    /// When this is true the route will be displayed in a sidebar/navbar
    /// instead of in the main view stack. This is useful for routes that
    /// are always displayed like the home page
    fn is_top_level() -> bool {
        false
    }

    /// When this is true the route will be built when it is registered
    /// by the router instead of when it is navigated to. This is useful
    /// for routes that are always displayed like the home page
    fn is_static() -> bool {
        false
    }

    /// This function should build the Widget that will be
    /// displayed when the route is navigated to
    fn build(parameter: Option<Self::Parameter>) -> impl IsA<gtk::Widget>;

    /// This function should build the navigation page that will be
    /// added to the view stack when the route is navigated to.
    /// By default this will just build a navigation page with the
    /// title set to the route and the child set to the result of
    /// `Self::build` but this can be overridden if you need to
    /// customize the navigation page in some way
    fn build_page(parameter: Option<Self::Parameter>) -> adw::NavigationPage {
        adw::NavigationPage::builder()
            .title(Self::route())
            .child(&Self::build(parameter))
            .build()
    }
}
