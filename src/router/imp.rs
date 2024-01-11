use crate::prelude::*;
use crate::router::ext::RoseRouterExt;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio;
use gtk::glib::{self};
use std::cell::{Cell, RefCell};

use crate::Application;

#[derive(Default, glib::Properties)]
#[properties(wrapper_type = super::Router)]
pub struct Router {
    #[property(set, get)]
    pub view: RefCell<adw::NavigationView>,
    #[property(get, set)]
    pub can_go_back: Cell<bool>,
    #[property(set, get)]
    pub view_switcher_pages: RefCell<adw::ViewStack>,
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

        let obj = self.obj().week_ref();
        // When the nav view changes pages we need to update can_go_back
        self.view.borrow().set_pop_on_escape(false);
        self.view
            .borrow()
            .connect_visible_page_notify(move |nav_view| {
                let this = obj.upgrade().unwrap();
                let page = nav_view.visible_page().unwrap();
                this.set_can_go_back(nav_view.previous_page(&page).is_some());
            });

        // gtk gets angy when the view stack dose not have a parent
        // so we set the parent to the router evan tho it wont
        // actually be displayed
        let obj = self.obj().week_ref();
        self.view_switcher_pages
            .borrow()
            .connect_visible_child_notify(move |view_stack| {
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

        // let back_controller = gtk::GestureClick::new();
        // back_controller.set_button(8);
        // let view = self.view.borrow().clone();
        // back_controller.connect_pressed(move |gesture, n_press, _, _| {
        //     println!("pressed: {}", n_press);
        //     let f = view.root().unwrap().focus().unwrap();
        //     println!("focus: {:?}", f.state_flags());
        //     f.unset_state_flags(gtk::StateFlags::FOCUSED);
        //     f.unset_state_flags(gtk::StateFlags::FOCUS_WITHIN);
        //     f.unset_state_flags(gtk::StateFlags::ACTIVE);
        //     back_action.activate(None);
        //     gesture.set_state(gtk::EventSequenceState::Claimed);
        // });
        // let view = self.view.borrow().clone();
        // for controller in view.observe_controllers().iter::<glib::Object>() {
        //     let controller = match controller {
        //         Ok(controller) => controller,
        //         _ => continue,
        //     };
        //     if controller.type_() == gtk::GestureClick::static_type() {
        //         let controller = controller.downcast_ref::<gtk::GestureClick>().unwrap();
        //         view.remove_controller(controller);
        //     }
        // }
        // view.add_controller(back_controller);
    }
}

impl WidgetImpl for Router {}
impl BinImpl for Router {}
