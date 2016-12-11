//! Collection of higher-order functions.

#![warn(
    missing_copy_implementations,
    missing_docs,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
)]

#![feature(
    fn_traits,
    unboxed_closures,
)]

/// Returns its argument.
///
/// # Examples
///
/// ```
/// # use pointfree::id;
/// assert_eq!(1, id(1));
/// ```
pub fn id<A>(a: A) -> A {
    a
}

mod always;
mod compose;
mod ignore;
mod partial;

pub use always::{Always, always, always1, always2, always3};
pub use compose::{Compose1, Compose2, Compose3, compose1, compose2, compose3};
pub use ignore::{Ignore1, Ignore2, Ignore3, ignore1, ignore2, ignore3};
pub use partial::{Partial1, Partial2, partial1, partial2};
