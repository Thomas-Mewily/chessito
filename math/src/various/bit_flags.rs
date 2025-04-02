use std::ops::*;
use crate::*;

pub trait BitFlags<T : Into<Self>> : UnsignedInteger + Into<Self>
{
    // Explicit (No into)

    fn with_explicit_mask         (self, mask : Self, set : bool) -> Self { if set { self.with_explicit_mask_set_one(mask) } else { self.with_explicit_mask_set_zero(mask) } }
    fn with_explicit_mask_set_one (self, mask : Self) -> Self { self | mask }
    fn with_explicit_mask_set_zero(self, mask : Self) -> Self { self & !mask }
    fn with_explicit_mask_toggle  (self, mask : Self) -> Self { self ^ mask }

    fn set_explicit_mask          (&mut self, mask : Self, set : bool) -> &mut Self { *self = self.with_explicit_mask(mask, set); self }
    fn explicit_mask_set_one      (&mut self, mask : Self) -> &mut Self { *self = self.with_explicit_mask_set_one(mask); self }
    fn explicit_mask_set_zero     (&mut self, mask : Self) -> &mut Self { *self = self.with_explicit_mask_set_zero(mask); self }
    fn explicit_mask_toggle       (&mut self, mask : Self) -> &mut Self { *self = self.with_explicit_mask_toggle(mask); self }

    fn explicit_mask_all_true     (self, mask : Self) -> bool { (self & mask) == mask }
    fn explicit_mask_any_one      (self, mask : Self) -> bool { (self & mask).is_non_zero() }
    fn explicit_mask_any_zero     (self, mask : Self) -> bool { !self.explicit_mask_all_true(mask) }


    fn with_explicit_flag         (self, bit_idx : Self, set : bool) -> Self { self.with_explicit_mask(Self::ONE << bit_idx,set) } 
    fn with_explicit_flag_add     (self, bit_idx : Self) -> Self { self.with_explicit_mask_set_one(Self::ONE << bit_idx) }
    fn with_explicit_flag_remove  (self, bit_idx : Self) -> Self { self.with_explicit_mask_set_zero(Self::ONE << bit_idx) }
    fn with_explicit_flag_toggle  (self, bit_idx : Self) -> Self { self.with_explicit_mask_toggle(Self::ONE << bit_idx) }

    fn set_explicit_flag          (&mut self, bit_idx : Self, set : bool) -> &mut Self { self.set_explicit_mask(Self::ONE << bit_idx,set); self } 
    fn explicit_flag_add          (&mut self, bit_idx : Self) -> &mut Self { self.explicit_mask_set_one(Self::ONE << bit_idx); self }
    fn explicit_flag_remove       (&mut self, bit_idx : Self) -> &mut Self { self.explicit_mask_set_zero(Self::ONE << bit_idx); self }
    fn explicit_flag_toggle       (&mut self, bit_idx : Self) -> &mut Self { self.explicit_mask_toggle(Self::ONE << bit_idx); self }

    fn explicit_flag_have         (self, bit_idx : Self) -> bool { self.explicit_mask_any_one(Self::ONE << bit_idx) }


    /// Implicit (with Into)

    fn with_mask         (self, mask : T, set : bool) -> Self { self.with_explicit_mask(mask.into(), set) }
    fn with_mask_set_one (self, mask : T) -> Self { self.with_explicit_mask_set_one(mask.into()) }
    fn with_mask_set_zero(self, mask : T) -> Self { self.with_explicit_mask_set_zero(mask.into()) }
    fn with_mask_toggle  (self, mask : T) -> Self { self.with_explicit_mask_toggle(mask.into()) }

    fn set_mask          (&mut self, mask : T, set : bool) -> &mut Self { self.set_explicit_mask(mask.into(), set); self }
    fn mask_set_one      (&mut self, mask : T) -> &mut Self { self.explicit_mask_set_one(mask.into()); self }
    fn mask_set_zero     (&mut self, mask : T) -> &mut Self { self.explicit_mask_set_zero(mask.into()); self }
    fn mask_toggle       (&mut self, mask : T) -> &mut Self { self.explicit_mask_toggle(mask.into()); self }

    fn mask_all_true     (self, mask : T) -> bool { self.explicit_mask_all_true(mask.into()) }
    fn mask_any_one      (self, mask : T) -> bool { self.explicit_mask_any_one(mask.into()) }
    fn mask_any_zero     (self, mask : T) -> bool { self.explicit_mask_any_zero(mask.into()) }


    fn with_flag         (self, bit_idx : T, set : bool) -> Self { self.with_explicit_flag(bit_idx.into(), set) } 
    fn with_flag_add     (self, bit_idx : T) -> Self { self.with_explicit_flag_add(bit_idx.into()) }
    fn with_flag_remove  (self, bit_idx : T) -> Self { self.with_explicit_flag_remove(bit_idx.into()) }
    fn with_flag_toggle  (self, bit_idx : T) -> Self { self.with_explicit_flag_toggle(bit_idx.into()) }

    fn set_flag          (&mut self, bit_idx : T, set : bool) { self.set_explicit_flag(bit_idx.into(), set); } 
    fn flag_add          (&mut self, bit_idx : T) { self.explicit_flag_add(bit_idx.into()); }
    fn flag_remove       (&mut self, bit_idx : T) { self.explicit_flag_remove(bit_idx.into()); }
    fn flag_toggle       (&mut self, bit_idx : T) { self.explicit_flag_toggle(bit_idx.into()); }

    fn flag_have         (self, bit_idx : T) -> bool { self.explicit_flag_have(bit_idx.into()) }
}

impl<U:UnsignedInteger, T : Into<U>> BitFlags<T> for U { }

/* 
impl<Precision : UnsignedInteger> HaveZero for BitFlags<Precision>
{ const ZERO : Self = Self(zero()); }

/* 
impl<Precision : UnsignedInteger> HaveOne for BitFlags<Precision>
{ const ONE : Self  = !Self::ZERO; }*/

#[derive(PartialEq, Eq, Debug, Default)]
pub struct BitFlags<Precision : UnsignedInteger>(Precision);

impl<Precision : UnsignedInteger> Clone for BitFlags<Precision>
{ fn clone(&self) -> Self { Self(self.0.clone()) }}

impl<Precision : UnsignedInteger> Copy for BitFlags<Precision> {}

impl<Precision : UnsignedInteger> BitFlags<Precision>
{
    pub fn iter_bit_idx() -> impl Iterator<Item = usize> { 0..std::mem::size_of::<Precision>() }
    /* 
    pub fn iter_one(self) -> impl Iterator<Item = Precision> 
    {
        Self::iter_bit_idx().filter_map(|e| if self.have(1 << e) { Some(1 << val) } else { None })
    }*/
    
    pub fn count_one (self) -> u32 { self.0.count_ones() }
    pub fn count_zero(self) -> u32 { self.0.count_zeros() }

    #[inline] pub fn is_exactly_value(self, mask : Self) -> bool { (self.0 & mask.0) == mask.0 }
    #[inline] pub fn any_one         (self, mask : Self) -> bool { (self.0 & mask.0).is_non_zero() }
    #[inline] pub fn set_one         (self, mask : Self) -> Self { Self(self.0 | mask.0) }
    #[inline] pub fn set_zero        (self, mask : Self) -> Self { Self(self.0 & !mask.0) }
    #[inline] pub fn toggle_mask     (self, mask : Self) -> Self { Self(self.0 ^ mask.0) }
    #[inline] pub fn with_mask       (self, mask : Self, set : bool) -> Self { if set { self.set_one(mask) } else { self.set_zero(mask) } }

    #[inline] pub fn have      (self, bit_idx : Self) -> bool { self.any_one(Self(Precision::ONE << bit_idx.0)) }
    #[inline] pub fn add       (self, bit_idx : Self) -> Self { self.set_one(Self(Precision::ONE << bit_idx.0)) }
    #[inline] pub fn remove    (self, bit_idx : Self) -> Self { self.set_zero(Self(Precision::ONE << bit_idx.0)) }
    #[inline] pub fn toggle    (self, bit_idx : Self) -> Self { self.toggle_mask(Self(Precision::ONE << bit_idx.0)) }
    #[inline] pub fn with      (self, bit_idx : Self, set : bool) -> Self { if set { self.add(bit_idx) } else { self.remove(bit_idx) } } 
}

impl<Precision : UnsignedInteger> BitAnd for BitFlags<Precision> { type Output=Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0.bitand(rhs.0)) }}
impl<Precision : UnsignedInteger> BitAndAssign for BitFlags<Precision> { fn bitand_assign(&mut self, rhs: Self) { self.0.bitand_assign(rhs.0) }}

impl<Precision : UnsignedInteger> BitOr for BitFlags<Precision> { type Output=Self; fn bitor(self, rhs: Self) -> Self::Output { Self(self.0.bitor(rhs.0)) }}
impl<Precision : UnsignedInteger> BitOrAssign for BitFlags<Precision> { fn bitor_assign(&mut self, rhs: Self) { self.0.bitor_assign(rhs.0) }}

impl<Precision : UnsignedInteger> BitXor for BitFlags<Precision> { type Output=Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0.bitxor(rhs.0)) }}
impl<Precision : UnsignedInteger> BitXorAssign for BitFlags<Precision> { fn bitxor_assign(&mut self, rhs: Self) { self.0.bitxor_assign(rhs.0) }}

impl<Precision : UnsignedInteger, T> Shl<T> for BitFlags<Precision> where Precision : Shl<T,Output=Precision>  { type Output=Self; fn shl(self, rhs: T) -> Self::Output { Self(self.0.shl(rhs)) }}
impl<Precision : UnsignedInteger, T> ShlAssign<T> for BitFlags<Precision> where Precision : ShlAssign<T>  { fn shl_assign(&mut self, rhs: T) { self.0.shl_assign(rhs); }}

impl<Precision : UnsignedInteger, T> Shr<T> for BitFlags<Precision> where Precision : Shr<T,Output=Precision>  { type Output=Self; fn shr(self, rhs: T) -> Self::Output { Self(self.0.shr(rhs)) }}
impl<Precision : UnsignedInteger, T> ShrAssign<T> for BitFlags<Precision> where Precision : ShrAssign<T>  { fn shr_assign(&mut self, rhs: T) { self.0.shr_assign(rhs); }}

impl<Precision : UnsignedInteger> Not for BitFlags<Precision>{ type Output=Self; fn not(self) -> Self::Output { Self(!self.0) }}
*/