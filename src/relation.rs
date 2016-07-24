use std::str::FromStr;

use url::{ParseError, Url};

/// Link relation types.
///
/// See [section 4][s4] and [section 6.2][s6.2].
///
/// [s4]: https://tools.ietf.org/html/rfc5988#section-4
/// [s6.2]: https://tools.ietf.org/html/rfc5988#section-6.2
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Relation {
    /// Designates a substitute for the link's context.
    Alternate,

    /// Refers to an appendix.
    Appendix,

    /// Refers to a bookmark or entry point.
    Bookmark,

    /// Refers to a chapter in a collection of resources.
    Chapter,

    /// Refers to a table of contents.
    Contents,

    /// Refers to a copyright statement that applies to the link's context.
    Copyright,

    /// Refers to a resource containing the most recent item(s) in a collection of resources.
    Current,

    /// Refers to a resource providing information about the link's context.
    Describedby,

    /// Refers to a resource that can be used to edit the link's context.
    Edit,

    /// Refers to a resource that can be used to edit media associated with the link's context.
    EditMedia,

    /// Identifies a related resource that is potentially large and might require special handling.
    Enclosure,

    /// An IRI that refers to the furthest preceding resource in a series of resources.
    First,

    /// Refers to a glossary of terms.
    Glossary,

    /// Refers to a resource offering help (more information, links to other sources information,
    /// etc.)
    Help,

    /// Refers to a hub that enables registration for notification of updates to the context.
    Hub,

    /// Refers to an index.
    Index,

    /// An IRI that refers to the furthest following resource in a series of resources.
    Last,

    /// Points to a resource containing the latest (e.g., current) version of the context.
    LatestVersion,

    /// Refers to a license associated with the link's context.
    License,

    /// Refers to the next resource in a ordered series of resources.
    Next,

    /// Refers to the immediately following archive resource.
    NextArchive,

    /// Indicates a resource where payment is accepted.
    Payment,

    /// Refers to the previous resource in an ordered series of resources. Synonym for "previous".
    Prev,

    /// Points to a resource containing the predecessor version in the version history.
    PredecessorVersion,

    /// Refers to the previous resource in an ordered series of resources.  Synonym for "prev".
    Previous,

    /// Refers to the immediately preceding archive resource.
    PrevArchive,

    /// Identifies a related resource.
    Related,

    /// Identifies a resource that is a reply to the context of the link.
    Replies,

    /// Refers to a section in a collection of resources.
    Section,

    /// Conveys an identifier for the link's context.
    Self_,

    /// Indicates a URI that can be used to retrieve a service document.
    Service,

    /// Refers to the first resource in a collection of resources.
    Start,

    /// Refers to an external style sheet.
    Stylesheet,

    /// Refers to a resource serving as a subsection in a collection of resources.
    Subsection,

    /// Points to a resource containing the successor version in the version history.
    SuccessorVersion,

    /// Refers to a parent document in a hierarchy of documents.
    Up,

    /// Points to a resource containing the version history for the context.
    VersionHistory,

    /// Identifies a resource that is the source of the information in the link's context.
    Via,

    /// Points to a working copy for this resource.
    WorkingCopy,

    /// Points to the versioned resource from which this working copy was obtained.
    WorkingCopyOf,

    /// Extension relation type.
    Extension(Url),
}

impl FromStr for Relation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        match s.to_lowercase().as_str() {
            "alternate" => Ok(Relation::Alternate),
            "appendix" => Ok(Relation::Appendix),
            "bookmark" => Ok(Relation::Bookmark),
            "chapter" => Ok(Relation::Chapter),
            "contents" => Ok(Relation::Contents),
            "copyright" => Ok(Relation::Copyright),
            "current" => Ok(Relation::Current),
            "describedby" => Ok(Relation::Describedby),
            "edit" => Ok(Relation::Edit),
            "edit-media" => Ok(Relation::EditMedia),
            "enclosure" => Ok(Relation::Enclosure),
            "first" => Ok(Relation::First),
            "glossary" => Ok(Relation::Glossary),
            "help" => Ok(Relation::Help),
            "hub" => Ok(Relation::Hub),
            "index" => Ok(Relation::Index),
            "last" => Ok(Relation::Last),
            "latest-version" => Ok(Relation::LatestVersion),
            "license" => Ok(Relation::License),
            "next" => Ok(Relation::Next),
            "next-archive" => Ok(Relation::NextArchive),
            "payment" => Ok(Relation::Payment),
            "prev" => Ok(Relation::Prev),
            "predecessor-version" => Ok(Relation::PredecessorVersion),
            "previous" => Ok(Relation::Previous),
            "prev-archive" => Ok(Relation::PrevArchive),
            "related" => Ok(Relation::Related),
            "replies" => Ok(Relation::Replies),
            "section" => Ok(Relation::Section),
            "self" => Ok(Relation::Self_),
            "service" => Ok(Relation::Service),
            "start" => Ok(Relation::Start),
            "stylesheet" => Ok(Relation::Stylesheet),
            "subsection" => Ok(Relation::Subsection),
            "successor-version" => Ok(Relation::SuccessorVersion),
            "up" => Ok(Relation::Up),
            "version-history" => Ok(Relation::VersionHistory),
            "via" => Ok(Relation::Via),
            "working-copy" => Ok(Relation::WorkingCopy),
            "working-copy-of" => Ok(Relation::WorkingCopyOf),
            extension => Url::parse(extension).map(Relation::Extension),
        }
    }
}
