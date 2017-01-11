//! Domain name system.

mod error;
mod label;
mod name;

pub use self::error::Error;
pub use self::label::Label;
pub use self::name::Name;
