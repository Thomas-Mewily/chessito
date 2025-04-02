use std::ops::*;
use crate::*;

// Vector related
pub trait CoordinateFloatingNumber : Sized + CoordinateNumber + NumberArithmetic<Self> 
    where 
    <Self as CoordinateUnit>::Precision : FloatingNumber
{
    fn length(self) -> Self::Precision { self.length_squared().sqrt() }
    fn normalized(self) -> Self 
    { 
        if self.have_length() 
        { 
            self / Self::splat(self.length())
        } 
        else 
        { 
            Self::ZERO
        }
    }
    
    fn normalize(&mut self) { *self = self.normalized(); }
    fn set_length(&mut self, length : Self::Precision) { *self = self.normalized() * Self::splat(length);  }
    fn with_length(mut self, length : Self::Precision) -> Self { self.set_length(length); self }
}


pub trait CoordinateInteger : Sized + CoordinateUnit
    where 
    <Self as CoordinateUnit>::Precision : Integer
{
    fn iter_area(self) -> impl Iterator<Item = Self>;
}

pub trait CoordinateNumber where Self : CoordinateUnit + NumberArithmetic, <Self as CoordinateUnit>::Precision : NumberArithmetic
{
    fn length_squared(self) -> Self::Precision;
    fn area(self) -> Self::Precision;

    fn min_element(self) -> Self::Precision;
    fn max_element(self) -> Self::Precision;
}

pub trait CoordinateUnit where Self : UnitArithmetic<Self>
{
    type Precision : UnitArithmetic<Self::Precision> + PartialEq;

    type UPoint : CoordinateUnit;
    type IPoint : CoordinateUnit;
    type Vec    : CoordinateUnit + Mul<real,Output=Self::Vec>;

    fn new_2d(x : Self::Precision, y : Self::Precision) -> Self;
    fn new_3d(x : Self::Precision, y : Self::Precision, z : Self::Precision) -> Self;
    fn new_4d(x : Self::Precision, y : Self::Precision, z : Self::Precision, w : Self::Precision) -> Self;

    fn splat(val : Self::Precision) -> Self;

    /// `x + y + z ...`
    fn sum_axis(self) -> Self::Precision;

    fn length_manhattan(self) -> Self::Precision;

    fn x(self) -> Self::Precision; 
    fn y(self) -> Self::Precision;
    fn z(self) -> Self::Precision;
    fn w(self) -> Self::Precision;

    fn set_x(&mut self, x : Self::Precision) -> &mut Self; 
    fn set_y(&mut self, y : Self::Precision) -> &mut Self; 
    fn set_z(&mut self, z : Self::Precision) -> &mut Self; 
    fn set_w(&mut self, w : Self::Precision) -> &mut Self;

    fn move_x(&mut self, delta_x : Self::Precision) -> &mut Self { self.set_x(self.x() + delta_x) }
    fn move_y(&mut self, delta_y : Self::Precision) -> &mut Self { self.set_y(self.y() + delta_y) }
    fn move_z(&mut self, delta_z : Self::Precision) -> &mut Self { self.set_z(self.z() + delta_z) }
    fn move_w(&mut self, delta_w : Self::Precision) -> &mut Self { self.set_w(self.w() + delta_w) } 

    fn move_neg_x(&mut self, delta_x : Self::Precision) -> &mut Self { self.set_x(self.x() - delta_x) }
    fn move_neg_y(&mut self, delta_y : Self::Precision) -> &mut Self { self.set_y(self.y() - delta_y) }
    fn move_neg_z(&mut self, delta_z : Self::Precision) -> &mut Self { self.set_z(self.z() - delta_z) }
    fn move_neg_w(&mut self, delta_w : Self::Precision) -> &mut Self { self.set_w(self.w() - delta_w) } 

    fn with_x(self, x : Self::Precision) -> Self;
    fn with_y(self, y : Self::Precision) -> Self;
    fn with_z(self, z : Self::Precision) -> Self;
    fn with_w(self, w : Self::Precision) -> Self;

    const X : Self;
    const Y : Self;
    const Z : Self;
    const W : Self;

    fn up(self) -> Self { self + Self::Y }
    fn right(self) -> Self { self + Self::X }

    fn left(self) -> Self where Self::Precision : Signed;
    fn down(self) -> Self where Self::Precision : Signed;

    fn have_length(self) -> bool 
    { 
        self.x().is_non_zero() || 
        self.y().is_non_zero() || 
        self.z().is_non_zero() || 
        self.w().is_non_zero()
    }

    fn any<F>(self, any_fn : F) -> bool where F : Fn(Self::Precision) -> bool;
    /// true for all
    fn all<F>(self, all_fn : F) -> bool where F : Fn(Self::Precision) -> bool { !self.any(|a| !all_fn(a))}
    
    /// Compare each component with another self
    fn cmp_any<F>(self, other : Self, any_fn : F) -> bool where F : Fn(Self::Precision, Self::Precision) -> bool;
    /// True for all.
    /// 
    /// Compare each component with another self
    fn cmp_all<F>(self, other : Self, any_fn : F) -> bool where F : Fn(Self::Precision, Self::Precision) -> bool { !self.cmp_any(other, |a, b| !any_fn(a, b))}
}

/* 
pub trait CoordinateRelativeExtension
{
    fn left(self) -> Self;
    fn down(self) -> Self;
}*/