//! Link header (de)serialization.
//!
//! Implementation of [RFC 5988](https://tools.ietf.org/html/rfc5988).

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
)]

extern crate url;

mod relation;

use url::Url;

pub use relation::Relation;

/// A typed connection between two resources.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Link {
    /// Context URL.
    pub context_url: Url,

    /// Link relation type.
    pub relation: Relation,

    /// Target URL.
    pub target_url: Url,

    /// Target attributes.
    pub target_attrs: (), // TODO
}
