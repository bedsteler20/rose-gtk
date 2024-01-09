use glib::subclass::prelude::*;
use glib::IsA;
use gtk::glib::{Cast, VariantTy};
use gtk::prelude::*;
use gtk::{gio, glib};

use crate::prelude::IsVariantTyVoidExt;
use crate::{DisplayableError, PageRoute};

pub trait RoseRouterExt:
    glib::object::IsClass + IsA<super::Router> + IsA<glib::Object> + IsA<gtk::Widget> + IsA<adw::Bin>
{
    /// Adds a route to the router. This will add a simple action to the
    /// routers action group with the name `router.visit.<route>` where `<route>` is
    /// the route name. When the action is activated the router will call
    /// `R::build_page` with the parameter from the action. If the route is
    /// top level the router will replace the current page with the new page
    /// otherwise it will push the new page onto the stack.
    fn add_route<R>(&self)
    where
        R: PageRoute + 'static,
    {
        let variant: &VariantTy = &R::Parameter::static_variant_type().into_owned();
        let variant = if variant.is_void() {
            None
        } else {
            Some(variant)
        };

        let action = gio::SimpleAction::new(&format!("visit.{}", R::route()), variant);
        let imp = super::imp::Router::from_obj(self.upcast_ref());
        let nav_view = imp.view.borrow().clone();
        let static_page = if R::is_static() {
            Some(R::build_page(None))
        } else {
            None
        };

        action.connect_activate(move |_, parameter| {
            let page = if let Some(static_page) = static_page.clone() {
                static_page
            } else {
                let variant = parameter.and_then(|variant| variant.get::<R::Parameter>());
                R::build_page(variant)
            };

            if R::is_top_level() {
                nav_view.replace(&[page]);
            } else {
                nav_view.push(&page);
            }
        });

        imp.action_group.add_action(&action);
    }

    /// Adds a route to the router and adds a page to the main pages view
    /// stack. This is useful for adding pages to a view switcher.
    fn add_main_route<R>(&self, title: &str, icon: &str)
    where
        R: PageRoute + 'static,
    {
        let imp = super::imp::Router::from_obj(self.upcast_ref());

        imp.view_switcher_pages.borrow().add_titled_with_icon(
            &adw::Bin::new(),
            Some(R::route()),
            title,
            icon,
        );
        self.add_route::<R>();
    }

    fn show_error<E>(&self, error: &E)
    where
        E: DisplayableError,
    {
        let imp = super::imp::Router::from_obj(self.upcast_ref());

        let page = adw::StatusPage::builder()
            .title(&error.tile())
            .description(&error.body())
            .build();

        let page = adw::NavigationPage::builder().child(&page).build();
        let view = imp.view.borrow().clone();
        let current_page = match view.visible_page() {
            Some(page) => page,
            None => return,
        };
        let is_first = view.previous_page(&current_page).is_none();
        if is_first {
            view.replace(&[page]);
        } else {
            view.pop();
            view.push(&page);
        }
    }

    /// Navigates to a route by its type. this will activate the action with
    /// the name `router.visit.<route>` where `<route>` is the route name.
    fn visit<R>(&self, parameter: Option<R::Parameter>)
    where
        R: PageRoute + 'static,
    {
        let imp = super::imp::Router::from_obj(self.upcast_ref());

        imp.action_group.activate_action(
            &format!("visit.{}", R::route()),
            parameter.map(|parameter| parameter.to_variant()).as_ref(),
        );
    }

    // Navigates to a route. This will activate the action with the name `router.visit.<route>`
    // where `<route>` is the route name. This is unsafe because the route name is not checked
    // at compile time nor is the parameter type checked. This is useful for navigating to routes
    // that are not known at compile time.
    unsafe fn visit_unsafe(&self, route: &str, parameter: Option<glib::Variant>) {
        let imp = super::imp::Router::from_obj(self.upcast_ref());

        imp.action_group.activate_action(route, parameter.as_ref());
    }

    /// Navigates back. This will activate the action with the name `back`.
    fn back(&self) {
        let imp = super::imp::Router::from_obj(self.upcast_ref());

        imp.action_group.activate_action("back", None);
    }
}

impl<T> RoseRouterExt for T where
    T: glib::object::IsClass
        + IsA<super::Router>
        + IsA<glib::Object>
        + IsA<gtk::Widget>
        + IsA<adw::Bin>
{
}
