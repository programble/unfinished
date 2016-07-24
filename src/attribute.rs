use hyper::LanguageTag;
use mime::Mime;

/// Target attributes.
///
/// See [section 5.4][s5.4].
///
/// [s5.4]: https://tools.ietf.org/html/rfc5988#section-5.4
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribute {
    /// A hint indicating what the language of the result of dereferencing the link should be.
    Hreflang(LanguageTag),

    /// Used to indicate intended destination medium or media for style information.
    Media(String), // TODO

    /// Used to label the destination of a link such that it can be used as a human-readable
    /// identifier.
    Title(String),

    // TODO: title*

    /// A hint indicating what the media type of the result of dereferencing the link should be.
    Type(Mime),

    /// Link extension link parameter.
    Extension(String, String),
}
