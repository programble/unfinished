//! Gap buffers...

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
)]

/// Gap buffer...
pub struct GapBuffer {
    inner: Vec<u8>,
}

/// Gap buffer string...
pub struct GapString {
    inner: GapBuffer,
}

/// Trait for slicing...
pub trait Slice<R> {
    /// Slice output type...
    type Output;

    /// Do the slice...
    fn slice(&self, range: R) -> Self::Output;
}

/// Trait for mutable slicing...
pub trait SliceMut<R> {
    /// Slice output type...
    type Output;

    /// Do the slice...
    fn slice_mut(&mut self, range: R) -> Self::Output;
}

/// Trait for splicing...
pub trait Splice<R> {
    /// Source data type...
    type Data;

    /// Do the splice...
    fn splice(&mut self, dest: R, src: Self::Data);
}
