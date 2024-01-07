mod application;
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
pub use router::page_route::PageRoute;
pub use router::Router;
pub use router::error::DisplayableError;

pub mod prelude {
    pub use crate::application::ext::RoseApplicationExt;
    pub use crate::utils::variant::IsVariantTyVoidExt;
    pub use crate::utils::weak_ref::AsWeakRefExt;
}

pub mod subclass {
    pub use crate::application::subclass as application;
    pub use crate::router::subclass as router;

    pub mod prelude {
        pub use crate::subclass::application::*;
        pub use crate::subclass::router::*;
    }
}
