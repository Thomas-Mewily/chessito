use std::{fmt::Display, num::Wrapping, ops::*};
use crate::*;

/// NOT APPROPRIATE FOR CRYPTOGRAPHIC USE
/// 
/// A really simple/dumb random for the moment. Will be changed in the futur if needed
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Random
{
    seed: Wrapping<u64>,
}

impl Default for Random { fn default() -> Self { Self::new_unseed() }}

impl Random
{
    pub fn new(seed : u64) -> Self { Self { seed: Wrapping(seed) }}
    pub fn new_unseed() -> Self { Self::new(123456789123456789) }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x << 21;
        x ^= x >> 35;
        x ^= x << 4;
        self.seed = x;
        x.0
    }


    pub fn gen_bool(&mut self) -> bool   {  self.next_u64() % 2 == 0 }

    /// between [0., 1.]
    pub fn gen_coef (&mut self) -> Coef  { self.next_u64().to_real() / u64::MAX.to_real() }
    pub fn gen_angle(&mut self) -> Angle { Angle::from_radian(self.gen_range(0.0..=Angle::FULL_RADIAN)) }

    /// return a normalized vec2 (length = 1) with a random direction
    pub fn gen_vec2_direction(&mut self) -> Vec2 { self.gen_angle().to_vec2_normalized() }
    /// return a vec2 where each axis is in [0.0..1.]
    pub fn gen_vec2_coef(&mut self) -> Vec2 { Vec2::new(self.gen_coef(), self.gen_coef()) }

    pub fn gen_range<T : Scalar + SmallestIncrement, Range : RangeBounds<T>>(&mut self, range: Range) -> T
    {
        let start_included = match range.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + T::SMALL_INC,
            Bound::Unbounded => T::MIN_VAL,
        };
        let end_excluded = match range.end_bound() {
            Bound::Included(&e) => e + T::SMALL_INC,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => T::MAX_VAL,
        };
    
        debug_assert!(start_included < end_excluded, "Random : start must be less than end");
    
        let range_size = end_excluded - start_included;
        let nb = start_included + T::from_real(self.gen_coef() * range_size.to_real());

        debug_assert!(nb >= start_included, "random generated number was too low");
        debug_assert!(nb < end_excluded, "random generated number was too hight");
        nb
    }

    /* 
    pub fn gen_range<T>(&mut self, range: Range<T>) -> T
        where T: Scalar
    {
        let start = range.start;
        let end   = range.end;
        debug_assert!(start < end, "Random : start must be less than end");

        let range_size = end - start;
        let nb = start + T::from_real(self.next_u64().to_real() * range_size.to_real() / u64::MAX.to_real());
        debug_assert!(nb >= start);
        debug_assert!(nb < end);
        nb
    }*/
}
