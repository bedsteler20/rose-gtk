use crate::prelude::*;
use crate::router::ext::RoseRouterExt;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio;
use gtk::glib::{self};
use std::cell::{Cell, RefCell};

use crate::Application;


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
