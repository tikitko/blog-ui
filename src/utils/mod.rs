pub mod app_content_context;
pub mod app_meta_context;
pub mod external;
#[cfg(feature = "client")]
pub mod get;
mod head;
pub mod logged_user_context;
pub mod not_empty;
pub mod use_load;

pub use app_content_context::*;
pub use app_meta_context::*;
pub use external::*;
#[cfg(feature = "client")]
pub use get::*;
pub use logged_user_context::*;
pub use not_empty::*;
pub use use_load::*;

#[cfg(not(feature = "client"))]
pub trait RequestableItem<P> {}

#[cfg(not(feature = "client"))]
impl<T, P> RequestableItem<P> for T {}
