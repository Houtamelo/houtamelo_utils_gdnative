use std::ops::*;
use std::cmp::*;
use saturate_into::*;

pub(crate) mod saturate_into;

#[macro_use] pub(crate) mod main;
#[macro_use] pub(crate) mod from;
#[macro_use] pub(crate) mod eq;
#[macro_use] pub(crate) mod ord;
#[macro_use] pub(crate) mod add;
#[macro_use] pub(crate) mod sub;
#[macro_use] pub(crate) mod mul;
#[macro_use] pub(crate) mod div;
#[macro_use] pub(crate) mod rem;

#[derive(Debug, Clone, Copy)]
pub struct BoundUSize<const MIN: usize, const MAX: usize> {
	inner_value: usize
}

#[derive(Debug, Clone, Copy)]
pub struct BoundU32<const MIN: u32, const MAX: u32> {
	inner_value: u32
}

#[derive(Debug, Clone, Copy)]
pub struct BoundU64<const MIN: u64, const MAX: u64> {
	inner_value: u64
}

#[derive(Debug, Clone, Copy)]
pub struct BoundISize<const MIN: isize, const MAX: isize> {
	inner_value: isize
}

#[derive(Debug, Clone, Copy)]
pub struct BoundI32<const MIN: i32, const MAX: i32> {
	inner_value: i32
}

#[derive(Debug, Clone, Copy)]
pub struct BoundI64<const MIN: i64, const MAX: i64> {
	inner_value: i64
}

bound_main!(BoundUSize<MIN, MAX>, usize);
bound_from!(BoundUSize<MIN, MAX>, usize);
bound_add!(BoundUSize<MIN, MAX>, usize);
bound_sub!(BoundUSize<MIN, MAX>, usize);
bound_mul!(BoundUSize<MIN, MAX>, usize);
bound_div!(BoundUSize<MIN, MAX>, usize);
bound_rem!(BoundUSize<MIN, MAX>, usize);
bound_eq!(BoundUSize<MIN, MAX>, usize);
bound_ord!(BoundUSize<MIN, MAX>, usize);

bound_main!(BoundU32<MIN, MAX>, u32);
bound_from!(BoundU32<MIN, MAX>, u32);
bound_add!(BoundU32<MIN, MAX>, u32);
bound_sub!(BoundU32<MIN, MAX>, u32);
bound_mul!(BoundU32<MIN, MAX>, u32);
bound_div!(BoundU32<MIN, MAX>, u32);
bound_rem!(BoundU32<MIN, MAX>, u32);
bound_eq!(BoundU32<MIN, MAX>, u32);
bound_ord!(BoundU32<MIN, MAX>, u32);

bound_main!(BoundU64<MIN, MAX>, u64);
bound_from!(BoundU64<MIN, MAX>, u64);
bound_add!(BoundU64<MIN, MAX>, u64);
bound_sub!(BoundU64<MIN, MAX>, u64);
bound_mul!(BoundU64<MIN, MAX>, u64);
bound_div!(BoundU64<MIN, MAX>, u64);
bound_rem!(BoundU64<MIN, MAX>, u64);
bound_eq!(BoundU64<MIN, MAX>, u64);
bound_ord!(BoundU64<MIN, MAX>, u64);

bound_main!(BoundISize<MIN, MAX>, isize);
bound_from!(BoundISize<MIN, MAX>, isize);
bound_add!(BoundISize<MIN, MAX>, isize);
bound_sub!(BoundISize<MIN, MAX>, isize);
bound_mul!(BoundISize<MIN, MAX>, isize);
bound_div!(BoundISize<MIN, MAX>, isize);
bound_rem!(BoundISize<MIN, MAX>, isize);
bound_eq!(BoundISize<MIN, MAX>, isize);
bound_ord!(BoundISize<MIN, MAX>, isize);

bound_main!(BoundI32<MIN, MAX>, i32);
bound_from!(BoundI32<MIN, MAX>, i32);
bound_add!(BoundI32<MIN, MAX>, i32);
bound_sub!(BoundI32<MIN, MAX>, i32);
bound_mul!(BoundI32<MIN, MAX>, i32);
bound_div!(BoundI32<MIN, MAX>, i32);
bound_rem!(BoundI32<MIN, MAX>, i32);
bound_eq!(BoundI32<MIN, MAX>, i32);
bound_ord!(BoundI32<MIN, MAX>, i32);

bound_main!(BoundI64<MIN, MAX>, i64);
bound_from!(BoundI64<MIN, MAX>, i64);
bound_add!(BoundI64<MIN, MAX>, i64);
bound_sub!(BoundI64<MIN, MAX>, i64);
bound_mul!(BoundI64<MIN, MAX>, i64);
bound_div!(BoundI64<MIN, MAX>, i64);
bound_rem!(BoundI64<MIN, MAX>, i64);
bound_eq!(BoundI64<MIN, MAX>, i64);
bound_ord!(BoundI64<MIN, MAX>, i64);
