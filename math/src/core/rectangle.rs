use std::{fmt::{Debug, Display}, ops::*};
use crate::*;

/// A `N` dimension rectangle
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Rectangle<C>
{
    pub pos  : C,
    pub size : C,
}

impl<C : HaveZero> HaveZero for Rectangle<C> 
{
    const ZERO : Self = Self { pos : C::ZERO, size:C::ZERO };
}
impl<C : HaveZero + HaveOne> HaveOne for Rectangle<C> 
{
    const ONE : Self = Self { pos : C::ZERO, size:C::ONE };
}

impl<C>  Rectangle<C> 
{
    pub fn new(pos : C, size : C) -> Self { Self { pos, size } }
    pub fn new_zero_pos(size : C) -> Self where C: HaveZero { Self { pos : C::ZERO, size } }
}

impl<C : CoordinateNumber>  Rectangle<C> where <C as CoordinateUnit>::Precision : NumberArithmetic 
{
    /// Put a area of a given size inside the current rectangle, and center it
    /// 
    /// This handle the scaling and the offset of the sub rectangle
    pub fn put_inside(&self, size_inside : C, center : C) -> Rectangle<C>
    {
        let area = size_inside * C::splat((self.size/size_inside).min_element());
        let unused_area = self.size - area;
        Rectangle::new(self.pos+unused_area * center, area)
    }

    pub fn new_centered(pos_middle : impl Into<C>, size : impl Into<C>) -> Self 
    { 
        let p = pos_middle.into();
        let s = size.into();
        Self::new(p-s/ (C::ONE + C::ONE), s) 
    }
    pub fn new_center(bottom_left : impl Into<C>, size : impl Into<C>, center : impl Into<C>) -> Self
    {
        let s = size.into();
        let c = center.into();
        let p = bottom_left.into() - c * s;
        Self::new(p, s)
    }

    pub fn bottom_left (&self) -> C { self.pos }
    pub fn bottom_right(&self) -> C { self.pos + self.size * C::X }
    pub fn top_right   (&self) -> C { self.pos + self.size }
    pub fn top_left    (&self) -> C { self.pos + self.size * C::Y }

    pub fn middle(&self) -> C { self.pos + self.size / (C::ONE + C::ONE) }
    pub fn middle_top   (&self) -> C { self.pos + self.size.with_x(self.size.x() / (C::Precision::ONE + C::Precision::ONE)) }
    pub fn middle_bot   (&self) -> C { self.pos + self.size.with_x(self.size.x() / (C::Precision::ONE + C::Precision::ONE)).with_y(zero()) }
    pub fn middle_right (&self) -> C { self.pos + self.size.with_y(self.size.y() / (C::Precision::ONE + C::Precision::ONE)) }
    pub fn middle_left   (&self) -> C { self.pos + self.size.with_y(self.size.y() / (C::Precision::ONE + C::Precision::ONE)).with_x(zero()) }

    pub fn area(&self) -> C::Precision { self.size.area()}

    pub fn left_value  (&self) -> C::Precision { self.pos.x() }
    pub fn right_value (&self) -> C::Precision { self.pos.x() + self.size.x() }
    pub fn bot_value   (&self) -> C::Precision { self.pos.y() }
    pub fn top_value   (&self) -> C::Precision { self.pos.y() + self.size.y() }

    pub fn width(&self)  -> C::Precision { self.size.x() }
    pub fn height(&self) -> C::Precision { self.size.y() }

    pub fn pos(&self) -> C { self.pos }
    pub fn set_pos(&mut self, pos : impl Into<C>) -> &mut Self { self.pos = pos.into(); self }
    pub fn with_pos(mut self, pos : impl Into<C>) -> Self { self.pos = pos.into(); self }

    pub fn size(&self) -> C { self.size }
    pub fn set_size(&mut self, size : impl Into<C>) -> &mut Self { self.size = size.into(); self }
    pub fn with_size(mut self, size : impl Into<C>) -> Self { self.size = size.into(); self }


    pub fn is_inside(&self, coordinate : C) -> bool where C::Precision : Number
    {
        self.pos.cmp_all(coordinate, |a, c| c >= a)
        && 
        (self.pos + self.size).cmp_all(coordinate, |a, c| c <= a)
    }

    /* 
    pub fn collide(&self, other : &Self) -> bool where C : PartialOrd<C>
    {
        self.pos
    }*/
}

/* 
impl<C : Coordinate + FloatingNumber> Rectangle<C>
{
    pub fn diag(&self) -> real { self.size().length() }
}
*/

impl<T : Scalar> Into<(C2<T>, C2<T>)> for Rectangle<C2<T>> { fn into(self) -> (C2<T>, C2<T>) { (self.pos, self.size)}}


impl<T : Display> Display for Rectangle<C2<T>> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "rect({}, {})", self.pos, self.size) }}
impl<T : Debug>   Debug   for Rectangle<C2<T>> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "rect({:?}, {:?})", self.pos, self.size) }}

//impl<T> Into<(C2<T>, C2<T>)> for Rectangle2<C2<T>> { fn into(self) -> (C2<T>, C2<T>) { (self.pos, self.size)}}
impl<T> From<(C2<T>, C2<T>)> for Rectangle<C2<T>> { fn from(rect: (C2<T>, C2<T>)) -> Rectangle<C2<T>> { Self::new(rect.0, rect.1) }}

impl<T> Into<[C2<T>; 2]> for Rectangle<C2<T>> { fn into(self) -> [C2<T>; 2] { [self.pos, self.size]}}
impl<T : Copy> From<[C2<T>; 2]> for Rectangle<C2<T>> { fn from(rect: [C2<T>; 2]) -> Rectangle<C2<T>> { Self::new(rect[0], rect[1]) }}

impl<T> Into<[T; 4]> for Rectangle<C2<T>> { fn into(self) -> [T; 4] { [self.pos.x, self.pos.y, self.size.x, self.size.y] }}
impl<T : Copy> From<[T; 4]> for Rectangle<C2<T>> { fn from(rect: [T; 4]) -> Rectangle<C2<T>> { Self::new(C2::<T>::new(rect[0], rect[1]), C2::<T>::new(rect[2], rect[3])) }}

impl<T> Into<(T,T,T,T)> for Rectangle<C2<T>> { fn into(self) -> (T,T,T,T) { (self.pos.x, self.pos.y, self.size.x, self.size.y) }}
impl<T> From<(T,T,T,T)> for Rectangle<C2<T>> { fn from(rect: (T,T,T,T)) -> Rectangle<C2<T>> { Self::new(C2::<T>::new(rect.0, rect.1), C2::<T>::new(rect.2, rect.3)) }}



impl<C, R : Scalar> Add<R> for Rectangle<C> where C : Add<R, Output=C> { type Output=Rectangle<C>; fn add(self, rhs: R) -> Self::Output { Self::new(self.pos.add(rhs), self.size.add(rhs)) }}
impl<C, R : Scalar> AddAssign<R> for Rectangle<C> where C : AddAssign<R> { fn add_assign(&mut self, rhs: R) { self.pos.add_assign(rhs); self.size.add_assign(rhs); }}

impl<C, R : Scalar> Sub<R> for Rectangle<C> where C : Sub<R, Output=C> { type Output=Rectangle<C>; fn sub(self, rhs: R) -> Self::Output { Self::new(self.pos.sub(rhs), self.size.sub(rhs)) }}
impl<C, R : Scalar> SubAssign<R> for Rectangle<C> where C : SubAssign<R> { fn sub_assign(&mut self, rhs: R) { self.pos.sub_assign(rhs); self.size.sub_assign(rhs); }}

impl<C, R : Scalar> Mul<R> for Rectangle<C> where C : Mul<R, Output=C> { type Output=Rectangle<C>; fn mul(self, rhs: R) -> Self::Output { Self::new(self.pos.mul(rhs), self.size.mul(rhs)) }}
impl<C, R : Scalar> MulAssign<R> for Rectangle<C> where C : MulAssign<R> { fn mul_assign(&mut self, rhs: R) { self.pos.mul_assign(rhs); self.size.mul_assign(rhs); }}

impl<C, R : Scalar> Div<R> for Rectangle<C> where C : Div<R, Output=C> { type Output=Rectangle<C>; fn div(self, rhs: R) -> Self::Output { Self::new(self.pos.div(rhs), self.size.div(rhs)) }}
impl<C, R : Scalar> DivAssign<R> for Rectangle<C> where C : DivAssign<R> { fn div_assign(&mut self, rhs: R) { self.pos.div_assign(rhs); self.size.div_assign(rhs); }}

impl<C, R : Scalar> Rem<R> for Rectangle<C> where C : Rem<R, Output=C> { type Output=Rectangle<C>; fn rem(self, rhs: R) -> Self::Output { Self::new(self.pos.rem(rhs), self.size.rem(rhs)) }}
impl<C, R : Scalar> RemAssign<R> for Rectangle<C> where C : RemAssign<R> { fn rem_assign(&mut self, rhs: R) { self.pos.rem_assign(rhs); self.size.rem_assign(rhs); }}


impl<C> Add<Rectangle<C>> for Rectangle<C> where C : Add<C,Output=C> { type Output=Rectangle<C>; fn add(self, rhs: Rectangle<C>) -> Self::Output { Self::new(self.pos.add(rhs.pos), self.size.add(rhs.size)) }}
impl<C> AddAssign<Rectangle<C>> for Rectangle<C> where C : AddAssign<C> { fn add_assign(&mut self, rhs: Rectangle<C>) { self.pos.add_assign(rhs.pos); self.size.add_assign(rhs.size); }}

impl<C> Sub<Rectangle<C>> for Rectangle<C> where C : Sub<C,Output=C> { type Output=Rectangle<C>; fn sub(self, rhs: Rectangle<C>) -> Self::Output { Self::new(self.pos.sub(rhs.pos), self.size.sub(rhs.size)) }}
impl<C> SubAssign<Rectangle<C>> for Rectangle<C> where C : SubAssign<C> { fn sub_assign(&mut self, rhs: Rectangle<C>) { self.pos.sub_assign(rhs.pos); self.size.sub_assign(rhs.size); }}

impl<C> Mul<Rectangle<C>> for Rectangle<C> where C : Mul<C,Output=C> { type Output=Rectangle<C>; fn mul(self, rhs: Rectangle<C>) -> Self::Output { Self::new(self.pos.mul(rhs.pos), self.size.mul(rhs.size)) }}
impl<C> MulAssign<Rectangle<C>> for Rectangle<C> where C : MulAssign<C> { fn mul_assign(&mut self, rhs: Rectangle<C>) { self.pos.mul_assign(rhs.pos); self.size.mul_assign(rhs.size); }}

impl<C> Div<Rectangle<C>> for Rectangle<C> where C : Div<C,Output=C> { type Output=Rectangle<C>; fn div(self, rhs: Rectangle<C>) -> Self::Output { Self::new(self.pos.div(rhs.pos), self.size.div(rhs.size)) }}
impl<C> DivAssign<Rectangle<C>> for Rectangle<C> where C : DivAssign<C> { fn div_assign(&mut self, rhs: Rectangle<C>) { self.pos.div_assign(rhs.pos); self.size.div_assign(rhs.size); }}

impl<C> Rem<Rectangle<C>> for Rectangle<C> where C : Rem<C,Output=C> { type Output=Rectangle<C>; fn rem(self, rhs: Rectangle<C>) -> Self::Output { Self::new(self.pos.rem(rhs.pos), self.size.rem(rhs.size)) }}
impl<C> RemAssign<Rectangle<C>> for Rectangle<C> where C : RemAssign<C> { fn rem_assign(&mut self, rhs: Rectangle<C>) { self.pos.rem_assign(rhs.pos); self.size.rem_assign(rhs.size); }}



impl<C> Rectangle<C>
{
    pub fn move_by(&mut self, delta : C) -> &mut Self where C : AddAssign<C> 
    { 
        self.pos += delta;
        self
    }
    
    /*
    fn add_margin_with_center(&mut self, margin : C, coef : Vec2Coef) where C : Coordinate + Mul<Vec2Coef, Output = C> //, C::Precision : Mul<real, Output = C::Precision>
    { 
        self.move_by(margin * coef);
        self.size -= margin;
    }*/

    pub fn add_margin_top(&mut self, margin_top : C::Precision) -> &mut Self where C : CoordinateUnit
    { 
        self.size.move_neg_y(margin_top);
        self
    }

    
    pub fn add_margin_bot(&mut self, margin_bot : C::Precision) -> &mut Self where C : CoordinateUnit
    {
        self.add_margin_top(margin_bot);
        self.pos.move_y(margin_bot);
        self
    }

    pub fn add_margin_right(&mut self, margin_right : C::Precision) -> &mut Self where C : CoordinateUnit
    {
        self.size.move_neg_x(margin_right);
        self
    }

    pub fn add_margin_left(&mut self, margin_left : C::Precision) -> &mut Self  where C : CoordinateUnit
    {
        self.add_margin_right(margin_left);
        self.pos.move_x(margin_left);
        self
    }

    pub fn add_margin_left_and_right(&mut self, margin_right_and_also_left : C::Precision) -> &mut Self where C : CoordinateUnit
    {
        self.add_margin_left(margin_right_and_also_left).add_margin_right(margin_right_and_also_left)
    }

    pub fn add_margin_top_and_bot(&mut self, margin_top_and_also_bot : C::Precision) -> &mut Self where C : CoordinateUnit
    {
        self.add_margin_top(margin_top_and_also_bot).add_margin_bot(margin_top_and_also_bot)
    }

    pub fn add_margin(&mut self, margin_each_side : C) -> &mut Self where C : CoordinateUnit
    {
        self.add_margin_left_and_right(margin_each_side.x()).add_margin_top_and_bot(margin_each_side.y())
    }


    pub fn glue_bot(&mut self, size_bot : C::Precision) -> &mut Self where C : CoordinateUnit
    { self.add_margin_top(self.size.y() - size_bot); self }

    pub fn glue_top(&mut self, size_top : C::Precision) -> &mut Self where C : CoordinateUnit
    { self.add_margin_bot(self.size.y() - size_top); self }

    pub fn glue_right(&mut self, size_right : C::Precision) -> &mut Self where C : CoordinateUnit
    { self.add_margin_left(self.size.x() - size_right); self }

    pub fn glue_left(&mut self, size_left : C::Precision) -> &mut Self where C : CoordinateUnit
    { self.add_margin_right(self.size.x() - size_left); self }

}