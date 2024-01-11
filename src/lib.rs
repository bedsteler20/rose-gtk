mod application;
mod functions;
mod future;
mod router;
mod utils;
pub use future::channels::Channel;
pub use future::channels::Receiver;
pub use future::channels::Sender;
pub use future::spawn::spawn_async;
pub use future::spawn::spawn_background_thread;
pub use future::spawn::RUNTIME;

pub use application::Application;
pub use router::error::DisplayableError;
pub use router::page_route::PageRoute;
pub use router::Router;

pub use functions::add_dependency;
pub use functions::back;
pub use functions::get_dependency;
pub use functions::init;
pub use functions::show_error_page;
pub use functions::visit;

pub mod prelude {
    pub use crate::application::ext::RoseApplicationExt;
    pub use crate::router::ext::RoseRouterExt;
    pub use crate::utils::variant::IsVariantTyVoidExt;
    pub use crate::utils::weak_ref::AsWeakRefExt;
    pub use crate::utils::widget::RoseWidgetExt;
}

pub mod subclass {
    pub use crate::application::subclass as application;
    pub use crate::router::subclass as router;

    pub mod prelude {
        pub use crate::subclass::application::*;
        pub use crate::subclass::router::*;
    }
}
