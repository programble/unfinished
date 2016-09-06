//! Type aliases.

use Fix;

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

/// Binary-scaled type aliases.
pub mod binary {
    use typenum::U2;

    /// Fixed-point `u8`.
    pub type FixU8<E> = super::FixU8<U2, E>;

    /// Fixed-point `u16`.
    pub type FixU16<E> = super::FixU16<U2, E>;

    /// Fixed-point `u32`.
    pub type FixU32<E> = super::FixU32<U2, E>;

    /// Fixed-point `u64`.
    pub type FixU64<E> = super::FixU64<U2, E>;

    /// Fixed-point `usize`.
    pub type FixUsize<E> = super::FixUsize<U2, E>;

    /// Fixed-point `i8`.
    pub type FixI8<E> = super::FixI8<U2, E>;

    /// Fixed-point `i16`.
    pub type FixI16<E> = super::FixI16<U2, E>;

    /// Fixed-point `i32`.
    pub type FixI32<E> = super::FixI32<U2, E>;

    /// Fixed-point `i64`.
    pub type FixI64<E> = super::FixI64<U2, E>;

    /// Fixed-point `isize`.
    pub type FixIsize<E> = super::FixIsize<U2, E>;
}

/// Decimal-scaled type aliases.
pub mod decimal {
    use typenum::U10;

    /// Fixed-point `u8`.
    pub type FixU8<E> = super::FixU8<U10, E>;

    /// Fixed-point `u16`.
    pub type FixU16<E> = super::FixU16<U10, E>;

    /// Fixed-point `u32`.
    pub type FixU32<E> = super::FixU32<U10, E>;

    /// Fixed-point `u64`.
    pub type FixU64<E> = super::FixU64<U10, E>;

    /// Fixed-point `usize`.
    pub type FixUsize<E> = super::FixUsize<U10, E>;

    /// Fixed-point `i8`.
    pub type FixI8<E> = super::FixI8<U10, E>;

    /// Fixed-point `i16`.
    pub type FixI16<E> = super::FixI16<U10, E>;

    /// Fixed-point `i32`.
    pub type FixI32<E> = super::FixI32<U10, E>;

    /// Fixed-point `i64`.
    pub type FixI64<E> = super::FixI64<U10, E>;

    /// Fixed-point `isize`.
    pub type FixIsize<E> = super::FixIsize<U10, E>;
}

/// SI prefixes.
pub mod si {
    use typenum::{N1, N2, N3, N6, N9, N12, N15, N18, N21, N24};
    use typenum::{P1, P2, P3, P6, P9, P12, P15, P18, P21, P24};
    use typenum::{U10, Z0};

    use Fix;

    /// 10^-24
    pub type Yocto<I> = Fix<I, U10, N24>;

    /// 10^-21
    pub type Zepto<I> = Fix<I, U10, N21>;

    /// 10^-18
    pub type Atto<I> = Fix<I, U10, N18>;

    /// 10^-15
    pub type Femto<I> = Fix<I, U10, N15>;

    /// 10^-12
    pub type Pico<I> = Fix<I, U10, N12>;

    /// 10^-9
    pub type Nano<I> = Fix<I, U10, N9>;

    /// 10^-6
    pub type Micro<I> = Fix<I, U10, N6>;

    /// 10^-3
    pub type Milli<I> = Fix<I, U10, N3>;

    /// 10^-2
    pub type Centi<I> = Fix<I, U10, N2>;

    /// 10^-1
    pub type Deci<I> = Fix<I, U10, N1>;

    /// 10^0
    pub type Unit<I> = Fix<I, U10, Z0>;

    /// 10^1
    pub type Deca<I> = Fix<I, U10, P1>;

    /// 10^2
    pub type Hecto<I> = Fix<I, U10, P2>;

    /// 10^3
    pub type Kilo<I> = Fix<I, U10, P3>;

    /// 10^6
    pub type Mega<I> = Fix<I, U10, P6>;

    /// 10^9
    pub type Giga<I> = Fix<I, U10, P9>;

    /// 10^12
    pub type Tera<I> = Fix<I, U10, P12>;

    /// 10^15
    pub type Peta<I> = Fix<I, U10, P15>;

    /// 10^18
    pub type Exa<I> = Fix<I, U10, P18>;

    /// 10^21
    pub type Zeta<I> = Fix<I, U10, P21>;

    /// 10^24
    pub type Yotta<I> = Fix<I, U10, P24>;
}

/// IEC prefixes.
pub mod iec {
    use typenum::{P10, P20, P30, P40, P50, P60, P70, P80, U2, Z0};

    use Fix;

    /// 2^0
    pub type Unit<I> = Fix<I, U2, Z0>;

    /// 2^10
    pub type Kibi<I> = Fix<I, U2, P10>;

    /// 2^20
    pub type Mebi<I> = Fix<I, U2, P20>;

    /// 2^30
    pub type Gibi<I> = Fix<I, U2, P30>;

    /// 2^40
    pub type Tebi<I> = Fix<I, U2, P40>;

    /// 2^50
    pub type Pebi<I> = Fix<I, U2, P50>;

    /// 2^60
    pub type Exbi<I> = Fix<I, U2, P60>;

    /// 2^70
    pub type Zebi<I> = Fix<I, U2, P70>;

    /// 2^80
    pub type Yobi<I> = Fix<I, U2, P80>;
}
