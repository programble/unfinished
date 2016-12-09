//! Collection of higher-order functions.

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
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
mod ignore;

pub use always::{Always, always};
pub use ignore::{Ignore1, Ignore2, Ignore3, ignore1, ignore2, ignore3};
