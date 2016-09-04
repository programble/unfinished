//! Type aliases.

use typenum::{U2, U10};

use super::Fix;

// Unsigned integer aliases.

/// Fixed-point `u8`.
pub type FixU8<B, E> = Fix<u8, B, E>;

/// Fixed-point `u16`.
pub type FixU16<B, E> = Fix<u16, B, E>;

/// Fixed-point `u32`.
pub type FixU32<B, E> = Fix<u32, B, E>;

/// Fixed-point `u64`.
pub type FixU64<B, E> = Fix<u64, B, E>;

/// Fixed-point `usize`.
pub type FixUsize<B, E> = Fix<usize, B, E>;

// Signed integer aliases.

/// Fixed-point `i8`.
pub type FixI8<B, E> = Fix<i8, B, E>;

/// Fixed-point `i16`.
pub type FixI16<B, E> = Fix<i16, B, E>;

/// Fixed-point `i32`.
pub type FixI32<B, E> = Fix<i32, B, E>;

/// Fixed-point `i64`.
pub type FixI64<B, E> = Fix<i64, B, E>;

/// Fixed-point `isize`.
pub type FixIsize<B, E> = Fix<isize, B, E>;

// Unsigned integer binary aliases.

/// Fixed-point `u8`, scaled in binary.
pub type FixU8B<E> = FixU8<U2, E>;

/// Fixed-point `u16`, scaled in binary.
pub type FixU16B<E> = FixU16<U2, E>;

/// Fixed-point `u32`, scaled in binary.
pub type FixU32B<E> = FixU32<U2, E>;

/// Fixed-point `u64`, scaled in binary.
pub type FixU64B<E> = FixU64<U2, E>;

/// Fixed-point `usize`, scaled in binary.
pub type FixUsizeB<E> = FixUsize<U2, E>;

// Signed integer binary aliases.

/// Fixed-point `i8`, scaled in binary.
pub type FixI8B<E> = FixI8<U2, E>;

/// Fixed-point `i16`, scaled in binary.
pub type FixI16B<E> = FixI16<U2, E>;

/// Fixed-point `i32`, scaled in binary.
pub type FixI32B<E> = FixI32<U2, E>;

/// Fixed-point `i64`, scaled in binary.
pub type FixI64B<E> = FixI64<U2, E>;

/// Fixed-point `isize`, scaled in binary.
pub type FixIsizeB<E> = FixIsize<U2, E>;

// Unsigned integer decimal aliases.

/// Fixed-point `u8`, scaled in decimal.
pub type FixU8D<E> = FixU8<U10, E>;

/// Fixed-point `u16`, scaled in decimal.
pub type FixU16D<E> = FixU16<U10, E>;

/// Fixed-point `u32`, scaled in decimal.
pub type FixU32D<E> = FixU32<U10, E>;

/// Fixed-point `u64`, scaled in decimal.
pub type FixU64D<E> = FixU64<U10, E>;

/// Fixed-point `usize`, scaled in decimal.
pub type FixUsizeD<E> = FixUsize<U10, E>;

// Signed integer decimal aliases.

/// Fixed-point `i8`, scaled in decimal.
pub type FixI8D<E> = FixI8<U10, E>;

/// Fixed-point `i16`, scaled in decimal.
pub type FixI16D<E> = FixI16<U10, E>;

/// Fixed-point `i32`, scaled in decimal.
pub type FixI32D<E> = FixI32<U10, E>;

/// Fixed-point `i64`, scaled in decimal.
pub type FixI64D<E> = FixI64<U10, E>;

/// Fixed-point `isize`, scaled in decimal.
pub type FixIsizeD<E> = FixIsize<U10, E>;
