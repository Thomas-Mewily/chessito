use std::{fmt::{Debug, Display}, ops::*};
use crate::*;


macro_rules! impl_zst_arithmetic
{
    ($primitive_name: ty) => 
    { 
        impl Display for $primitive_name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0{}", stringify!($primitive_name)) }}
        impl Debug for $primitive_name { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0{}", stringify!($primitive_name)) }}

        impl Neg for $primitive_name { type Output=Self; fn neg(self) -> Self::Output { Self }}
        
        impl Add<Self> for $primitive_name { type Output = Self; fn add(self, _: Self) -> Self::Output { Self }}
        impl Sub<Self> for $primitive_name { type Output = Self; fn sub(self, _: Self) -> Self::Output { Self }}
        impl Mul<Self> for $primitive_name { type Output = Self; fn mul(self, _: Self) -> Self::Output { Self }}
        impl Div<Self> for $primitive_name { type Output = Self; fn div(self, _: Self) -> Self::Output { Self }}
        impl Rem<Self> for $primitive_name { type Output = Self; fn rem(self, _: Self) -> Self::Output { Self }}
        impl AddAssign<Self> for $primitive_name { fn add_assign(&mut self, _: Self) { }}
        impl SubAssign<Self> for $primitive_name { fn sub_assign(&mut self, _: Self) { }}
        impl MulAssign<Self> for $primitive_name { fn mul_assign(&mut self, _: Self) { }}
        impl DivAssign<Self> for $primitive_name { fn div_assign(&mut self, _: Self) { }}
        impl RemAssign<Self> for $primitive_name { fn rem_assign(&mut self, _: Self) { }}

        impl $primitive_name
        {
            pub const MIN : Self = Self;
            pub const MAX : Self = Self;
        }
    };
}
map_on_zero_scalar!(impl_zst_arithmetic);
/* 
impl_zst_arithmetic!(f0);
impl_zst_arithmetic!(u0);
impl_zst_arithmetic!(i0);
*/

pub trait Signed : Neg
{
    /// The unsigned equivalence
    type UnsignedEquivalent : Copy + Unsigned;
    fn to_unsigned(self) -> Self::UnsignedEquivalent;
}
macro_rules! impl_signed
{
    ($primitive_name: ty, $unsigned_primitive_name: ty) => 
    { 
        impl Signed for $primitive_name  { type UnsignedEquivalent=$unsigned_primitive_name; #[inline] fn to_unsigned(self) -> Self::UnsignedEquivalent  {self as Self::UnsignedEquivalent }}
    };
}
impl Signed for i0  { type UnsignedEquivalent=u0; #[inline] fn to_unsigned(self) -> Self::UnsignedEquivalent  { Self::UnsignedEquivalent{}  }}
impl_signed!(i8 , u8);
impl_signed!(i16, u16);
impl_signed!(i32, u32);
impl_signed!(i64, u64);
impl_signed!(isize, usize);
impl_signed!(f32, f32);
impl_signed!(f64, f64);

pub trait Unsigned
{
    /// The signed equivalence
    type SignedEquivalent : Copy + Signed;
    fn to_signed(self) -> Self::SignedEquivalent;
}
macro_rules! impl_unsigned
{
    ($primitive_name: ty, $signed_primitive_name: ty) => 
    { 
        impl Unsigned for $primitive_name { type SignedEquivalent=$signed_primitive_name; #[inline] fn to_signed(self) -> Self::SignedEquivalent { self as Self::SignedEquivalent  }}
    };
}
impl Unsigned for u0  { type SignedEquivalent=i0; #[inline] fn to_signed(self) -> Self::SignedEquivalent { Self::SignedEquivalent{} }}
impl_unsigned!(u8 , i8);
impl_unsigned!(u16, i16);
impl_unsigned!(u32, i32);
impl_unsigned!(u64, i64);
impl_unsigned!(usize, isize);
impl_unsigned!(f32, f32);
impl_unsigned!(f64, f64);


pub trait Lerpable : Mul<real,Output=Self> + Add<Self,Output=Self> + Copy
{
    /// Not restricted between [0..1]
    fn lerp_unrestricted(self, dest : Self, coef : Coef) -> Self { self * (1. - coef) + dest * coef  }

    /// Restricted between [0..1]
    fn lerp(self, dest : Self, coef : Coef) -> Self { self.lerp_unrestricted(dest, coef.min(1.).max(0.)) }

    /// Restricted between [0..1]
    fn ease(self, dest : Self, coef : Coef, easing : Easing) -> Self { self.lerp(dest, easing.apply(coef)) }

    /// Not restricted between [0..1]
    fn ease_unrestricted(self, dest : Self, coef : Coef, easing : Easing) -> Self { self.lerp_unrestricted(dest, easing.apply(coef)) }
}
impl<T : Mul<real,Output=T> + Add<T,Output=T> + Copy> Lerpable for T {}


pub trait Percent  { fn percent(self) -> Coef; }
macro_rules! impl_percent{
    ($primitive_name: ty) => 
    { impl Percent for $primitive_name { #[inline] fn percent(self) -> real { (self.to_real()) / 100. }} };
}
map_on_scalar!(impl_percent);

//#[repr(i8)]
#[derive(Debug, Copy, Clone)]
pub enum IndexRangeOrdering { Less = -1, InRange = 0, Greater = 1 }

pub trait IndexRangeCmp where Self : Scalar
{ 
    fn cmp_idx_in_range<Range>(&self, range : Range) -> IndexRangeOrdering 
        where Range: RangeBounds<Self>,
    {
        match range.start_bound()
        {
            Bound::Included(r) => if self <  r { return IndexRangeOrdering::Less; },
            Bound::Excluded(r) => if self <= r { return IndexRangeOrdering::Less; },
            Bound::Unbounded => {},
        }
        match range.end_bound()
        {
            Bound::Included(r) => if self >  r { return IndexRangeOrdering::Greater; },
            Bound::Excluded(r) => if self >= r { return IndexRangeOrdering::Greater; },
            Bound::Unbounded => {},
        }
        
        IndexRangeOrdering::InRange
    }
}


macro_rules! impl_percent{
    ($primitive_name: ty) => 
    { impl IndexRangeCmp for $primitive_name {} };
}
map_on_scalar!(impl_percent);


pub trait Absolute { fn absolute(self) -> Self; }
macro_rules! impl_abs{
    ($primitive_name: ty) => 
    { impl Absolute for $primitive_name { #[inline] fn absolute(self) -> Self { Self::abs(self) }} };
}
map_on_non_zero_floating!(impl_abs);
map_on_non_zero_signed!(impl_abs);
macro_rules! impl_abs_is_itself {
    ($primitive_name: ty) => 
    { impl Absolute for $primitive_name { #[inline] fn absolute(self) -> Self { self }} };
}
map_on_non_zero_unsigned!(impl_abs_is_itself);
map_on_zero_scalar!(impl_abs_is_itself);


pub trait ToUint   { fn to_uint(self) -> uint; }
macro_rules! impl_to_real {
    ($primitive_name: ty) => 
    { impl ToUint for $primitive_name { #[inline] fn to_uint(self) -> uint { self as uint }} };
}
map_on_non_zero_scalar!(impl_to_real);
macro_rules! impl_to_real_for_zst {
    ($primitive_name: ty) => 
    { impl ToUint for $primitive_name { #[inline] fn to_uint(self) -> uint { 0 as uint }} };
}
map_on_zero_scalar!(impl_to_real_for_zst);
impl ToUint for bool { fn to_uint(self) -> uint { if self { 1 } else { 0 } }}



pub trait ToInt   { fn to_int(self) -> int; }
macro_rules! impl_to_int_for_int {
    ($primitive_name: ty) => 
    { impl ToInt for $primitive_name { #[inline] fn to_int(self) -> int { self as int }} };
}
map_on_non_zero_integer!(impl_to_int_for_int);
macro_rules! impl_to_int_for_float {
    ($primitive_name: ty) => 
    { impl ToInt for $primitive_name { #[inline] fn to_int(self) -> int { if self >= 0. {self as int} else { self as int - 1} }} };
}
map_on_non_zero_floating!(impl_to_int_for_float);
macro_rules! impl_to_int_for_zst {
    ($primitive_name: ty) => 
    { impl ToInt for $primitive_name { #[inline] fn to_int(self) -> int { 0 as int }} };
}
map_on_zero_scalar!(impl_to_int_for_zst);
impl ToInt for bool { fn to_int(self) -> int { if self { 1 } else { 0 } }}



/// Convert the number to real one
pub trait ToReal   { fn to_real(self) -> real; }
macro_rules! impl_to_real {
    ($primitive_name: ty) => 
    { impl ToReal for $primitive_name { #[inline] fn to_real(self) -> real { self as real }} };
}
map_on_non_zero_scalar!(impl_to_real);
macro_rules! impl_to_real_for_zst {
    ($primitive_name: ty) => 
    { impl ToReal for $primitive_name { #[inline] fn to_real(self) -> real { 0 as real }} };
}
map_on_zero_scalar!(impl_to_real_for_zst);
impl ToReal for bool { fn to_real(self) -> real { if self { 1.0 } else { 0. } }}


/// Should saturate if the value is not representable
pub trait FromReal   { fn from_real(val : real) -> Self; }
macro_rules! impl_from_real_for_integer {
    ($primitive_name: ty) => 
    { 
        impl FromReal for $primitive_name 
        { 
            ///Todo : Use the trait `std::convert::FloatToInt` when stabilized
            #[inline] fn from_real(val : real) -> Self 
            { 
                if val >= Self::MAX_VAL as real { return Self::MAX_VAL; }
                if val <= Self::MIN_VAL as real { return Self::MIN_VAL; }
                val as Self
            }
        } 
    }
}
map_on_non_zero_integer!(impl_from_real_for_integer);
macro_rules! impl_from_real_for_float {
    ($primitive_name: ty) => 
    { 
        impl FromReal for $primitive_name 
        { #[inline] fn from_real(val : real) -> Self { val as Self }}
    }
}
map_on_non_zero_floating!(impl_from_real_for_float);
macro_rules! impl_from_real_for_zst {
    ($primitive_name: ty) => 
    { impl FromReal for $primitive_name { #[inline] fn from_real(_val : real) -> Self { Self } } }
}
map_on_zero_scalar!(impl_from_real_for_zst);
impl FromReal for bool { fn from_real(val : real) -> Self { val >= 0.5 }}

/// Define the `0` representation
pub trait HaveZero 
{ 
    const ZERO : Self;
    fn is_zero(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::ZERO }
    fn is_non_zero(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_zero() }
}
macro_rules! impl_have_zero {
    ($primitive_name: ty) => 
    { impl HaveZero for $primitive_name { const ZERO : Self = 0 as Self; } };
}
map_on_non_zero_scalar!(impl_have_zero);
macro_rules! impl_have_zero_for_zst {
    ($primitive_name: ty) => 
    { impl HaveZero for $primitive_name { const ZERO : Self = Self; } };
}
map_on_zero_scalar!(impl_have_zero_for_zst);
impl HaveZero for bool { const ZERO : Self = false; }
pub const fn zero<T : HaveZero>() -> T { T::ZERO }

/// Define the half `0.5` representation
pub trait HaveHalf  
{ 
    const HALF : Self; 
    fn is_half(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::HALF }
    fn is_non_half(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_half() }
}
macro_rules! impl_have_half {
    ($primitive_name: ty) => 
    { impl HaveHalf for $primitive_name { const HALF : Self = 0.5 as Self; } };
}
map_on_non_zero_scalar!(impl_have_half);
macro_rules! impl_have_half_for_zst {
    ($primitive_name: ty) => 
    { impl HaveHalf for $primitive_name { const HALF : Self = Self; } };
}
map_on_zero_scalar!(impl_have_half_for_zst);
pub const fn half<T : HaveHalf>() -> T { T::HALF }

/// Define the smallest reasonable increment. `1` for integer, 1/60. for float (can change).
pub trait SmallestIncrement  { const SMALL_INC  : Self; }
macro_rules! impl_smallest_inc_integer {
    ($primitive_name: ty) => 
    { impl SmallestIncrement for $primitive_name { const SMALL_INC : Self = Self::ONE; } };
}
map_on_integer!(impl_smallest_inc_integer);
macro_rules! impl_smallest_inc_float {
    ($primitive_name: ty) => 
    { impl SmallestIncrement for $primitive_name { const SMALL_INC : Self = Self::ZERO; } };
}
map_on_floating!(impl_smallest_inc_float);

/// Define the `1` representation for the number
pub trait HaveOne  
{ 
    const ONE  : Self; 
    fn is_one(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::ONE }
    fn is_non_one(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_one() }
}
macro_rules! impl_have_one {
    ($primitive_name: ty) => 
    { impl HaveOne for $primitive_name { const ONE : Self = 1 as Self; } };
}
map_on_non_zero_scalar!(impl_have_one);
macro_rules! impl_have_one_for_zst {
    ($primitive_name: ty) => 
    { impl HaveOne for $primitive_name { const ONE : Self = Self; } };
}
map_on_zero_scalar!(impl_have_one_for_zst);
impl HaveOne for bool { const ONE  : Self = true; }
pub const fn one<T : HaveOne>() -> T { T::ONE }


/* 
/// Define the `2` representation for the number
pub trait HaveTwo  
{ 
    const TWO  : Self; 
    fn is_two(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::TWO }
    fn is_non_two(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_two() }
}
impl<T : HaveOne + Add<Self, Output = Self>> HaveTwo for T 
{
    // cannot call non-const operator in constants calls in constants are limited to constant functions, tuple structs and tuple variants
    const TWO  : Self = Self::ONE + Self::ONE;
}
*/

/// Define the `-1` representation for the number
pub trait HaveMinusOne  
{ 
    const MINUS_ONE  : Self;
    fn is_minus_one(self) -> bool where Self : PartialEq<Self> + Copy { self == Self::MINUS_ONE }
    fn is_non_minus_one(&self) -> bool where Self : PartialEq<Self> + Copy { !self.is_minus_one() }
}
macro_rules! impl_have_one {
    ($primitive_name: ty) => 
    { impl HaveMinusOne for $primitive_name { const MINUS_ONE : Self = -1 as Self; } };
}
map_on_non_zero_floating!(impl_have_one);
map_on_non_zero_signed!(impl_have_one);
impl HaveMinusOne for i0 { const MINUS_ONE  : Self = Self; }
impl HaveMinusOne for f0 { const MINUS_ONE  : Self = Self; }
pub const fn minus_one<T : HaveMinusOne>() -> T { T::MINUS_ONE }


/// The main floating point supported by the engine (`f32` for the moment)
pub trait RealNumber : FloatingNumber + UnitArithmetic<real> {}
impl RealNumber for real{}

/// For floating point only : `f32`, `f64` currently
pub trait FloatingNumber : Scalar + HaveHalf 
{ 
    fn sqrt(self) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;

    fn from_usize(val : usize) -> Self;

    /// between [0., 1.]
    fn normalize(self) -> Self { self.min_partial(Self::ONE).max_partial(Self::ZERO) }

    // add more if needed
}
macro_rules! impl_floating_number {
    ($primitive_name: ty) => 
    { 
        impl FloatingNumber for $primitive_name 
        { 
            fn sqrt(self) -> Self { self.sqrt() }
            fn cos(self) -> Self { self.cos() }
            fn sin(self) -> Self { self.sin() }
            fn from_usize(val : usize) -> Self { val as Self}
        }
    };
}
map_on_non_zero_floating!(impl_floating_number);
impl FloatingNumber for f0 
{ 
    fn sqrt(self) -> Self { self }
    fn cos(self) -> Self { self }
    fn sin(self) -> Self { self }
    fn from_usize(_ : usize) -> Self { Self }
}

/// For unsigned integer : `u8`, `u16`, `u32`, `u64`, `usize`
pub trait UnsignedInteger : 
    Eq + Ord + Scalar + 
    Shl<Self, Output=Self> + ShlAssign<Self> +
    Shr<Self, Output=Self>  + ShlAssign<Self> +
    BitOr<Self, Output=Self> + BitOrAssign<Self> +
    BitAnd<Self, Output=Self> + BitAndAssign<Self> + 
    BitXor<Self, Output=Self> + BitXorAssign<Self> +
    Not<Output = Self>
{
    fn count_ones(self) -> u32;
    fn count_zeros(self) -> u32;
}
macro_rules! impl_unsigned_integer {
    ($primitive_name: ty) => 
    { 
        impl UnsignedInteger for $primitive_name 
        {
            fn count_ones(self) -> u32 { Self::count_ones(self) }
            fn count_zeros(self) -> u32 { Self::count_zeros(self) }
        }
    };
}
map_on_non_zero_unsigned!(impl_unsigned_integer);
impl UnsignedInteger for u0
{
    fn count_ones(self) -> u32 { 0 }
    fn count_zeros(self) -> u32 { 0 }
}
impl Shl for u0 { type Output=Self; fn shl(self, _rhs: Self) -> Self::Output { Self }}
impl ShlAssign for u0 { fn shl_assign(&mut self, _rhs: Self) {}}

impl Shr for u0 { type Output=Self; fn shr(self, _rhs: Self) -> Self::Output { Self }}
impl ShrAssign for u0 { fn shr_assign(&mut self, _rhs: Self) {}}

impl BitAnd for u0 { type Output=Self; fn bitand(self, _rhs: Self) -> Self::Output { Self }}
impl BitAndAssign for u0 { fn bitand_assign(&mut self, _rhs: Self) {}}

impl BitOr for u0 { type Output=Self; fn bitor(self, _rhs: Self) -> Self::Output { Self }}
impl BitOrAssign for u0 { fn bitor_assign(&mut self, _rhs: Self) {}}

impl BitXor for u0 { type Output=Self; fn bitxor(self, _rhs: Self) -> Self::Output { Self }}
impl BitXorAssign for u0 { fn bitxor_assign(&mut self, _rhs: Self) {}}

impl Not for u0 { type Output=Self; fn not(self) -> Self::Output { Self }}


pub trait UnsignedIntegerOrFloatingNumber : Scalar {}
macro_rules! impl_unsigned_integer_or_float {
    ($primitive_name: ty) => 
    { 
        impl UnsignedIntegerOrFloatingNumber for $primitive_name {}
    };
}
map_on_unsigned!(impl_unsigned_integer_or_float);
map_on_floating!(impl_unsigned_integer_or_float);

pub trait SignedIntegerOrFloatingNumber : Scalar {}
macro_rules! impl_unsigned_integer_or_float {
    ($primitive_name: ty) => 
    { 
        impl SignedIntegerOrFloatingNumber for $primitive_name {}
    };
}
map_on_signed!(impl_unsigned_integer_or_float);
map_on_floating!(impl_unsigned_integer_or_float);

/// For signed integer : `i8`, `i16`, `i32`, `i64`, `isize`
pub trait SignedInteger : Eq + Ord + Scalar {}
macro_rules! impl_signed_integer {
    ($primitive_name: ty) => 
    { impl SignedInteger for $primitive_name {} };
}
map_on_signed!(impl_signed_integer);


pub trait HaveMinMaxValue { const MIN_VAL : Self; const MAX_VAL : Self; }
macro_rules! impl_have_min_max_value {
    ($primitive_name: ty) => 
    { impl HaveMinMaxValue for $primitive_name 
        { 
            const MIN_VAL : Self = Self::MIN;
            const MAX_VAL : Self = Self::MAX;
        }    
    };
}
map_on_scalar!(impl_have_min_max_value);


/// For floating the range is : `[0., 1.]`. For integers the range is : `[0, MAX]`
pub trait HaveDefaultRange : HaveMinMaxValue + ToReal + FromReal + UnitArithmetic<Self>
{ 
    const MIN_RANGE_VAL : Self;
    const MAX_RANGE_VAL : Self;
    const RANGE : Self;

    #[inline] fn to_coef(self) -> Coef { (self - Self::MIN_RANGE_VAL).to_real() / Self::RANGE.to_real()  }
    /// based on the HaveDefaultRange trait
    #[inline] fn from_coef(c : Coef) -> Self { Self::from_real(Self::RANGE.to_real() * c + Self::MIN_RANGE_VAL.to_real()) }
}
macro_rules! impl_have_min_max_range_value_signed {
    ($primitive_name: ty) => 
    { impl HaveDefaultRange for $primitive_name 
        { 
            const MIN_RANGE_VAL : Self = Self::ZERO;
            const MAX_RANGE_VAL : Self = Self::MAX_VAL;
            const RANGE         : Self = Self::MAX_RANGE_VAL - Self::ZERO;
        }
    };
}
map_on_non_zero_signed!(impl_have_min_max_range_value_signed);
impl HaveDefaultRange for i0 { const MIN_RANGE_VAL : Self = Self; const MAX_RANGE_VAL : Self = Self; const RANGE : Self = Self; }
macro_rules! impl_have_min_max_range_value_unsigned {
    ($primitive_name: ty) => 
    { impl HaveDefaultRange for $primitive_name 
        { 
            const MIN_RANGE_VAL : Self = Self::ZERO;
            const MAX_RANGE_VAL : Self = Self::MAX_VAL;
            const RANGE         : Self = Self::MAX_RANGE_VAL - Self::MIN_RANGE_VAL;
        }
    };
}
map_on_non_zero_unsigned!(impl_have_min_max_range_value_unsigned);
impl HaveDefaultRange for u0 { const MIN_RANGE_VAL : Self = Self; const MAX_RANGE_VAL : Self = Self; const RANGE : Self = Self; }
macro_rules! impl_have_min_max_range_value_float {
    ($primitive_name: ty) => 
    { impl HaveDefaultRange for $primitive_name 
        { 
            const MIN_RANGE_VAL : Self = Self::ZERO;
            const MAX_RANGE_VAL : Self = Self::ONE;
            const RANGE         : Self = Self::MAX_RANGE_VAL - Self::MIN_RANGE_VAL;
        }
    };
}
map_on_non_zero_floating!(impl_have_min_max_range_value_float);
impl HaveDefaultRange for f0 { const MIN_RANGE_VAL : Self = Self; const MAX_RANGE_VAL : Self = Self; const RANGE : Self = Self; }


/// For signed and unsigned integer : (`u8`, `u16`, `u32`, `u64`, `usize`) + (`i8`, `i16`, `i32`, `i64`, `isize`)
pub trait Integer : Eq + Ord + Scalar {}
macro_rules! impl_integer {
    ($primitive_name: ty) => 
    { impl Integer for $primitive_name {} };
}
map_on_integer!(impl_integer);


pub trait NumberArithmetic<T=Self> :
    UnitArithmetic<Self> + 
    Mul<T,Output=Self> + Div<T,Output=Self> + Rem<T,Output=Self> +
    MulAssign<T> + DivAssign<T> + RemAssign<T> +
{}

/// For any mesurable things, like distance in (meter, km...), time (s, ms, mins, hours...)
pub trait UnitArithmetic<T=Self> :
    Copy + Clone +
    Add<T,Output=Self> + Sub<T,Output=Self> + 
    AddAssign<T> + SubAssign<T> +
    HaveZero + HaveOne + Absolute + 
    PartialEq +
    Debug
{}
macro_rules! impl_basic_arithmetic {
    ($primitive_name: ty) => 
    { 
        impl UnitArithmetic<Self> for $primitive_name {} 
        impl NumberArithmetic<Self> for $primitive_name {} 
    };
}
map_on_scalar!(impl_basic_arithmetic);


pub trait NumberCompare : PartialEq + PartialOrd + PartialOrdComparison {}
macro_rules! impl_nb_cmp {
    ($primitive_name: ty) => { impl NumberCompare for $primitive_name {} };
}
map_on_scalar!(impl_nb_cmp);

pub trait Unit : UnitArithmetic<Self> + NumberCompare + SmallestIncrement { }
macro_rules! impl_unit {
    ($primitive_name: ty) => { impl Unit for $primitive_name {} };
}
map_on_scalar!(impl_unit);

pub trait Number : Unit + NumberArithmetic<Self> {}
macro_rules! impl_number {
    ($primitive_name: ty) => { impl Number for $primitive_name {} };
}
map_on_scalar!(impl_number);

/// For signed unsigned integer + floating
pub trait Scalar : Number + ToReal + FromReal + ToInt + ToUint + HaveMinMaxValue + HaveDefaultRange
{ }
macro_rules! impl_scalar {
    ($primitive_name: ty) => 
    { impl Scalar for $primitive_name {} };
}
map_on_scalar!(impl_scalar);

/// For floating / signed integer : `0-x`, `x*-1`
/// 
/// For boolean : `!b`
pub trait Reversable { fn rev(self) -> Self; }
macro_rules! impl_rev_nb {
    ($primitive_name: ty) => 
    { impl Reversable for $primitive_name { fn rev(self) -> Self { self * Self::MINUS_ONE}} };
}
map_on_floating!(impl_rev_nb);
map_on_signed!(impl_rev_nb);
impl Reversable for bool { fn rev(self) -> Self { !self }}


/* 
pub trait ScalableByReal : Mul<real,Output=Self> {}
macro_rules! impl_scalable_by_real {
    ($primitive_name: ty) => 
    { impl ScalableByReal for $primitive_name {} };
}
impl_scalable_by_real!(real);
*/


pub trait PartialOrdComparison : PartialOrd
{
    #[inline] fn max_partial(self, other: Self) -> Self where Self: Sized { if self >= other { self } else { other } }
    #[inline] fn min_partial(self, other: Self) -> Self where Self: Sized { if self <= other { self } else { other } }

    #[inline] fn clamp_partial(self, min: Self, max: Self) -> Self where Self: Sized
    {
        // copied frm rust std
        assert!(min <= max);
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}
macro_rules! impl_partial_cmp {
    ($primitive_name: ty) => 
    { impl PartialOrdComparison for $primitive_name {} };
}
map_on_scalar!(impl_partial_cmp);

