//! Domain name system.

mod error;
mod label;
mod name;

pub use self::error::Error;
pub use self::label::Label;
pub use self::name::Name;

/// Resource record query types.
///
/// Superset of [`Type`](trait.Type.html).
pub trait QType {
    /// Returns the type code value.
    ///
    /// Wants to be an associated constant.
    fn value() -> u16;
}

/// Resource record types.
///
/// Subset of [`QType`](trait.QType.html).
pub trait Type: QType { }

/// Resource record query classes.
///
/// Superset of [`Class`](trait.Class.html).
pub trait QClass {
    /// Returns the class code value.
    ///
    /// Wants to be an associated constant.
    fn value() -> u16;
}

/// Resource record classes.
///
/// Subset of [`QClass`](trait.QClass.html).
pub trait Class: QClass { }
