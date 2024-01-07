use super::Route;
use crate::prelude::*;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio;
use gtk::glib::{self, VariantTy};
use std::cell::{Cell, RefCell};

mod imp {

    use crate::Application;

    use super::*;

    #[derive(Default, Debug, glib::Properties)]
    #[properties(wrapper_type = super::Router)]
    pub struct Router {
        #[property(set, get)]
        pub view: RefCell<adw::NavigationView>,
        #[property(get, set)]
        pub can_go_back: Cell<bool>,
        #[property(set, get)]
        pub main_pages: RefCell<adw::ViewStack>,
        pub action_group: gio::SimpleActionGroup,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Router {
        const NAME: &'static str = "RoseRouter";
        type Type = super::Router;
        type ParentType = adw::Bin;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Router {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj()
                .insert_action_group("router", Some(&self.action_group));

            let back_action = gio::SimpleAction::new("back", None);
            let obj = self.obj().week_ref();
            back_action.connect_activate(move |_, _| {
                let this = obj.upgrade().unwrap();
                this.view().pop();
            });
            self.action_group.add_action(&back_action);

            let nav_view = adw::NavigationView::new();
            let obj = self.obj().week_ref();
            // When the nav view changes pages we need to update can_go_back
            nav_view.connect_visible_page_notify(move |nav_view| {
                let this = obj.upgrade().unwrap();
                let page = nav_view.visible_page().unwrap();
                this.set_can_go_back(nav_view.previous_page(&page).is_some());
            });

            // gtk gets angy when the view stack dose not have a parent
            // so we set the parent to the router evan tho it wont
            // actually be displayed
            let view_stack = adw::ViewStack::new();
            let obj = self.obj().week_ref();
            view_stack.connect_visible_child_notify(move |view_stack| {
                println!("view stack changed");
                let this = obj.upgrade().unwrap();
                let page = view_stack.visible_child_name().unwrap();
                unsafe {
                    this.visit_unsafe(&page, None);
                }
            });

            if let Some(app) = Application::try_find() {
                if !app.has_dependency::<super::Router>() {
                    app.add_dependency(&self.obj().clone());
                }
            }
        }
    }

    impl WidgetImpl for Router {}
    impl BinImpl for Router {}
}

glib::wrapper! {
    pub struct Router(ObjectSubclass<imp::Router>) @extends gtk::Widget, adw::Bin;
}

impl Router {
    /// Adds a route to the router. This will add a simple action to the
    /// routers action group with the name `router.visit.<route>` where `<route>` is
    /// the route name. When the action is activated the router will call
    /// `R::build_page` with the parameter from the action. If the route is
    /// top level the router will replace the current page with the new page
    /// otherwise it will push the new page onto the stack.
    pub fn add_route<R>(&self)
    where
        R: Route + 'static,
    {
        let variant: &VariantTy = &R::Parameter::static_variant_type().into_owned();
        let variant = if variant.is_void() {
            None
        } else {
            Some(variant)
        };

        let action = gio::SimpleAction::new(&format!("visit.{}", R::route()), variant);
        let imp = self.imp();
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
    pub fn add_main_route<R>(&self, title: &str, icon: &str)
    where
        R: Route + 'static,
    {
        self.imp().main_pages.borrow().add_titled_with_icon(
            &adw::Bin::new(),
            Some(R::route()),
            title,
            icon,
        );
        self.add_route::<R>();
    }

    /// Navigates to a route by its type. this will activate the action with
    /// the name `router.visit.<route>` where `<route>` is the route name.
    pub fn visit<R>(&self, parameter: Option<R::Parameter>)
    where
        R: Route + 'static,
    {
        self.imp().action_group.activate_action(
            &format!("visit.{}", R::route()),
            parameter.map(|parameter| parameter.to_variant()).as_ref(),
        );
    }

    // Navigates to a route. This will activate the action with the name `router.visit.<route>`
    // where `<route>` is the route name. This is unsafe because the route name is not checked
    // at compile time nor is the parameter type checked. This is useful for navigating to routes
    // that are not known at compile time.
    pub unsafe fn visit_unsafe(&self, route: &str, parameter: Option<glib::Variant>) {
        self.imp()
            .action_group
            .activate_action(route, parameter.as_ref());
    }

    /// Navigates back. This will activate the action with the name `back`.
    pub fn back(&self) {
        self.imp().action_group.activate_action("back", None);
    }
}
