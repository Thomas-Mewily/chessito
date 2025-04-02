use std::{fmt::{Debug, Display}, ops::*};
use crate::*;

type C<T> = C2<T>;

macro_rules! op_with_myself { 
    ($myself : ident, $fn_name : ident, $other : expr) => 
    { Self::new($myself.x.$fn_name($other.x), $myself.y.$fn_name($other.y)) };
}
macro_rules! assign_with_myself {
    ($myself : ident, $fn_name : ident, $other : expr) =>
    { { $myself.x.$fn_name($other.x); $myself.y.$fn_name($other.y); } };
}



macro_rules! op_with_scalar {
    ($myself : ident, $fn_name : ident, $other : expr) => 
    { Self::Output::new($myself.x.$fn_name($other), $myself.y.$fn_name($other)) };
}
macro_rules! assign_with_scalar {
    ($myself : ident, $fn_name : ident, $other : expr) =>
    { { $myself.x.$fn_name($other); $myself.y.$fn_name($other); } };
}


macro_rules! op_component {
    ($myself : ident, $op_name : tt) =>
    {  $myself.x $op_name $myself.y };
}


/// 2D Coordinate
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2<T>
{
    pub x : T,
    pub y : T,
}

pub trait Splat2 : Sized + Copy { fn splat2(self) -> C<Self> { C::splat_const(self) }}
impl<T:Copy> Splat2 for T {}

impl<T> From<(T,T)> for C<T> { fn from(value: (T,T)) -> Self { Self::new(value.0, value.1) }}
impl<T> From<C<T>> for (T,T) { fn from(value: C<T>) -> Self { (value.x, value.y) }}

impl<T : Copy> From<[T;2]> for C<T> { fn from(value: [T;2]) -> Self { Self::new(value[0], value[1]) }}
impl<T> From<C<T>> for [T;2] { fn from(value: C<T>) -> Self { [value.x, value.y] }}

impl<T : Integer + ToReal> From<C<T>> for C<real> { fn from(value: C<T>) -> Self { value.map(|e| e.to_real()) }}

impl<T : Debug>   Debug for C<T>   { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({:?}, {:?})", self.x, self.y) }}
impl<T : Display> Display for C<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})", self.x, self.y) } }



// Point Related
type CIter<T> = C2Iter<T>;
pub struct C2Iter<T>
{
    cur : C<T>,
    end : C<T>,
}

impl<T : Integer> CoordinateInteger for C<T> 
{ 
    fn iter_area(self) -> impl Iterator<Item=Self> { CIter { cur: zero(), end: self } }
}
impl<T : Integer> Iterator for CIter<T>
{
    type Item=C<T>;

    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.cur.y >= self.end.y { return None; }
        let old = self.cur;

        self.cur.x += one();
        if self.cur.x < self.end.x { return Some(old); }

        self.cur.x  = zero();
        self.cur.y += one();

        Some(old)
    }
}


// Vector Related
impl C<real> 
{ 
    /// set the angle of the `(x, y)` axis
    pub fn set_angle(&mut self, angle : Angle) { *self = angle.to_vec2(self.length()) }
}

impl<T : Copy> C<T> 
{
    pub const fn splat_const(xy : T) -> Self { Self::new(xy, xy) }
}

impl<T> C<T> 
{
    pub const fn new(x : T, y : T) -> Self { Self { x, y }}
    #[inline] pub fn map<Z, F>(self, map_fn : F) -> C<Z> where F : Fn(T) -> Z { C::<Z>::new( map_fn(self.x), map_fn(self.y)) }
}

impl<T : ToReal> C<T> 
{
    pub fn to_vec2(self) -> C2<real> {  C2::<real>::new(self.x.to_real(), self.y.to_real())}
    pub fn to_vec3(self, z : real) -> C3<real> { C3::<real>::new(self.x.to_real(), self.y.to_real(), z)}
    pub fn to_vec4(self, z : real, w : real) -> C4<real> { C4::<real>::new(self.x.to_real(), self.y.to_real(), z , w)}
}

impl<T : ToInt> C<T> 
{
    pub fn to_point2(self) -> C2<int> {  C2::<int>::new(self.x.to_int(), self.y.to_int())}
    pub fn to_point3(self, z : int) -> C3<int> { C3::<int>::new(self.x.to_int(), self.y.to_int(), z)}
    pub fn to_point4(self, z : int, w : int) -> C4<int> { C4::<int>::new(self.x.to_int(), self.y.to_int(), z , w)}
}

impl<T : ToUint> C<T> 
{
    pub fn to_point2u(self) -> C2<uint> {  C2::<uint>::new(self.x.to_uint(), self.y.to_uint())}
    pub fn to_point3u(self, z : uint) -> C3<uint> { C3::<uint>::new(self.x.to_uint(), self.y.to_uint(), z)}
    pub fn to_point4u(self, z : uint, w : uint) -> C4<uint> { C4::<uint>::new(self.x.to_uint(), self.y.to_uint(), z , w)}
}

impl<T : Reversable + HaveZero + Copy> C<T>
{
    pub fn x_rx(self) -> Self { Self::new(self.x, self.x.rev()) }
    pub fn x_ry(self) -> Self { Self::new(self.x, self.y.rev()) }
    pub fn y_rx(self) -> Self { Self::new(self.y, self.x.rev()) }
    pub fn y_ry(self) -> Self { Self::new(self.y, self.y.rev()) }
    pub fn rx_x(self) -> Self { Self::new(self.x.rev(), self.x) }
    pub fn rx_y(self) -> Self { Self::new(self.x.rev(), self.y) }
    pub fn rx_rx(self) -> Self { Self::new(self.x.rev(), self.x.rev()) }
    pub fn rx_ry(self) -> Self { Self::new(self.x.rev(), self.y.rev()) }
    pub fn rx_0(self) -> Self { Self::new(self.x.rev(), T::ZERO) }
    pub fn ry_x(self) -> Self { Self::new(self.y.rev(), self.x) }
    pub fn ry_y(self) -> Self { Self::new(self.y.rev(), self.y) }
    pub fn ry_rx(self) -> Self { Self::new(self.y.rev(), self.x.rev()) }
    pub fn ry_ry(self) -> Self { Self::new(self.y.rev(), self.y.rev()) }
    pub fn ry_0(self) -> Self { Self::new(self.y.rev(), T::ZERO) }
    pub fn _0_rx(self) -> Self { Self::new(T::ZERO, self.x.rev()) }
    pub fn _0_ry(self) -> Self { Self::new(T::ZERO, self.y.rev()) }
}

impl<T : HaveZero + Copy> C<T> 
{ 
    pub fn x_x(self) -> Self { Self::new(self.x, self.x) }
    pub fn x_y(self) -> Self { Self::new(self.x, self.y) }
    pub fn x_0(self) -> Self { Self::new(self.x, T::ZERO) }
    pub fn y_x(self) -> Self { Self::new(self.y, self.x) }
    pub fn y_y(self) -> Self { Self::new(self.y, self.y) }
    pub fn y_0(self) -> Self { Self::new(self.y, T::ZERO) }
    pub fn _0_x(self) -> Self { Self::new(T::ZERO, self.x) }
    pub fn _0_y(self) -> Self { Self::new(T::ZERO, self.y) }
    pub fn _0_0(self) -> Self { Self::new(T::ZERO, T::ZERO) }
}

impl<T : Scalar> C<T>
{
    pub fn clamp<Min : Into<Self>, Max : Into<Self>>(self, min : Min, max : Max) -> Self 
    { 
        let mini : Self = min.into();
        let maxi : Self = max.into();
        Self::new(
            self.x.clamp_partial(mini.x, maxi.x),
            self.y.clamp_partial(mini.y, maxi.y),
        )
    }
}

impl<T : Number> CoordinateNumber for C<T>
{
    fn length_squared(self) -> T { self.x*self.x + self.y*self.y }
    fn area(self) -> Self::Precision { (op_component!(self, *)).absolute() }

    #[inline] fn min_element(self) -> Self::Precision { self.x().min_partial(self.y()) }
    #[inline] fn max_element(self) -> Self::Precision { self.x().max_partial(self.y()) }
}

impl<T : UnitArithmetic> CoordinateUnit for C<T>
{
    type Precision=T;

    #[inline] fn splat(xy : Self::Precision) -> Self { Self::splat_const(xy) }

    fn sum_axis(self) -> Self::Precision { op_component!(self, +) }

    fn length_manhattan(self) -> Self::Precision { self.absolute().sum_axis() }
    
    /// Unsigned point
    type UPoint= C<uint>;

    /// Signed point
    type IPoint = C<int>;

    /// Signed Vector
    type Vec   = C<real>;
    
    const X : Self = Self::new(T::ONE, T::ZERO);
    const Y : Self = Self::new(T::ZERO, T::ONE);
    const Z : Self = Self::new(T::ZERO, T::ZERO);
    const W : Self = Self::new(T::ZERO, T::ZERO);
        
    #[inline] fn x(self) -> Self::Precision { self.x }
    #[inline] fn y(self) -> Self::Precision { self.y }
    #[inline] fn z(self) -> Self::Precision { T::ZERO }
    #[inline] fn w(self) -> Self::Precision { T::ZERO }

    #[inline] fn set_x(&mut self, x : Self::Precision) -> &mut Self { self.x = x; self }
    #[inline] fn set_y(&mut self, y : Self::Precision) -> &mut Self { self.y = y; self }
    #[inline] fn set_z(&mut self, _z : Self::Precision) -> &mut Self { self }
    #[inline] fn set_w(&mut self, _w : Self::Precision) -> &mut Self { self }
    
    #[inline] fn with_x(mut self, x : Self::Precision) -> Self { self.x = x; self }
    #[inline] fn with_y(mut self, y : Self::Precision) -> Self { self.y = y; self }
    #[inline] fn with_z(self, _z : Self::Precision) -> Self { self }
    #[inline] fn with_w(self, _w : Self::Precision) -> Self { self }

    fn left(self) -> Self { self-Self::X }
    fn down(self) -> Self { self-Self::Y }

    fn any<F>(self, any_fn : F) -> bool where F : Fn(Self::Precision) -> bool { any_fn(self.x) || any_fn(self.y) }
    fn all<F>(self, all_fn : F) -> bool where F : Fn(Self::Precision) -> bool { all_fn(self.x) && all_fn(self.y) }

    fn cmp_any<F>(self, other : Self, any_fn : F) -> bool where F : Fn(Self::Precision, Self::Precision) -> bool 
    { any_fn(self.x, other.x) || any_fn(self.y, other.y) }
    fn cmp_all<F>(self, other : Self, any_fn : F) -> bool where F : Fn(Self::Precision, Self::Precision) -> bool
    { any_fn(self.x, other.x) && any_fn(self.y, other.y) }
    
    fn new_2d(x : Self::Precision, y : Self::Precision) -> Self { Self::new(x, y) }
    fn new_3d(x : Self::Precision, y : Self::Precision, _z : Self::Precision) -> Self { Self::new(x, y) }
    fn new_4d(x : Self::Precision, y : Self::Precision, _z : Self::Precision, _w : Self::Precision) -> Self { Self::new(x, y) }
}



// The rest of the code is the same code for all coordinate type :

impl<T : NumberArithmetic> NumberArithmetic for C<T>{}
impl<T : FloatingNumber> CoordinateFloatingNumber for C<T> {}

impl<T:Copy> From<T> for C<T> 
{
    fn from(value: T) -> Self { C::<T>::splat_const(value) }
}

impl<T : Scalar> C<T>
{
    /// using the `(x, y)` axis
    pub fn angle(self) -> Angle { Angle::from_radian(self.y.to_real().atan2(self.x.to_real())) }
}

impl<T : Scalar> C<T>
{
    pub fn min<Other : Into<Self>>(self, other : Other) -> Self { let o : Self = other.into(); op_with_myself!(self,min_partial,o) }
    pub fn max<Other : Into<Self>>(self, other : Other) -> Self { let o : Self = other.into(); op_with_myself!(self,max_partial,o) }
}

impl<T : HaveHalf + Copy> HaveHalf for C<T>  { const HALF : Self = Self::splat_const(T::HALF); }
impl<T : HaveZero + Copy> HaveZero for C<T> { const ZERO : Self = Self::splat_const(T::ZERO); }
impl<T : HaveOne + Copy> HaveOne for C<T>  { const ONE  : Self = Self::splat_const(T::ONE); }

impl<T : Signed + Scalar + Integer> C<T>
{
    /// Convert the point to it's unsigned equivalence
    /// 
    /// Don't confuse it with `to_point2u` that cast it to a `point2u`
    pub fn to_upoint(self) -> C<T::UnsignedEquivalent> where <T as number::Signed>::UnsignedEquivalent: number::Scalar { self.map(|c| c.to_unsigned()) }
}
impl<T : Unsigned + Scalar + Integer> C<T>
{
    /// Convert the point to it's signed equivalence
    /// 
    /// Don't confuse it with `to_point2` that cast it to a `point2`
    pub fn to_point(self) -> C<T::SignedEquivalent> where <T as number::Unsigned>::SignedEquivalent: number::Scalar { self.map(|c| c.to_signed()) }
}
impl<T : Scalar> C<T>
{
    /// Convert the point to it's real equivalence
    pub fn to_vec(self) -> C<real> { self.map(|c| c.to_real()) }
}


impl<T : UnitArithmetic<T>> UnitArithmetic<Self> for C<T> {}
impl<T : Absolute> Absolute for C<T>  { fn absolute(self) -> Self { self.map(|e| e.absolute()) }}

impl<T : UnsignedIntegerOrFloatingNumber> From<C<T>> for Point2 where T : { fn from(value: C<T>) -> Self { value.to_point2() }}
impl<T : SignedIntegerOrFloatingNumber> From<C<T>> for UPoint2 where T : { fn from(value: C<T>) -> Self { value.to_point2u() }}


impl<T : Add<A,Output = T>, A : Scalar> Add<A> for C<T> { type Output=C<T>; fn add(self, rhs: A) -> Self::Output { op_with_scalar!(self,add,rhs)  }}
impl<T : AddAssign<A>, A : Scalar> AddAssign<A> for C<T> { fn add_assign(&mut self, rhs: A) { assign_with_scalar!(self,add_assign,rhs) }}

impl<T : Sub<A,Output = T>, A : Scalar> Sub<A> for C<T> { type Output=C<T>; fn sub(self, rhs: A) -> Self::Output { op_with_scalar!(self,sub,rhs)  }}
impl<T : SubAssign<A>, A : Scalar> SubAssign<A> for C<T> { fn sub_assign(&mut self, rhs: A) { assign_with_scalar!(self,sub_assign,rhs) }}

impl<T : Mul<A,Output = T>, A : Scalar> Mul<A> for C<T> { type Output=C<T>; fn mul(self, rhs: A) -> Self::Output { op_with_scalar!(self,mul,rhs)  }}
impl<T : MulAssign<A>, A : Scalar> MulAssign<A> for C<T> { fn mul_assign(&mut self, rhs: A) { assign_with_scalar!(self,mul_assign,rhs) }}

impl<T : Div<A,Output = T>, A : Scalar> Div<A> for C<T> { type Output=C<T>; fn div(self, rhs: A) -> Self::Output { op_with_scalar!(self,div,rhs)  }}
impl<T : DivAssign<A>, A : Scalar> DivAssign<A> for C<T> { fn div_assign(&mut self, rhs: A) { assign_with_scalar!(self,div_assign,rhs) }}

impl<T : Rem<A,Output = T>, A : Scalar> Rem<A> for C<T> { type Output=C<T>; fn rem(self, rhs: A) -> Self::Output { op_with_scalar!(self,rem,rhs)  }}
impl<T : RemAssign<A>, A : Scalar> RemAssign<A> for C<T> { fn rem_assign(&mut self, rhs: A) { assign_with_scalar!(self,rem_assign,rhs) }}

impl<T : HaveZero + Copy + Sub<T,Output = T>> Neg for C<T> { type Output = Self; fn neg(self) -> Self::Output { Self::ZERO - self }}


impl<T : Add<T,Output = T>> Add<Self> for C<T> { type Output=C<T>; fn add(self, rhs: Self) -> Self::Output { op_with_myself!(self,add,rhs) }}
impl<T : AddAssign> AddAssign<Self> for C<T> { fn add_assign(&mut self, rhs: Self) { assign_with_myself!(self,add_assign,rhs) }}

impl<T : Sub<T,Output = T>> Sub<Self> for C<T> { type Output=C<T>; fn sub(self, rhs: Self) -> Self::Output  { op_with_myself!(self,sub,rhs) }}
impl<T : SubAssign> SubAssign<Self> for C<T> { fn sub_assign(&mut self, rhs: Self) { assign_with_myself!(self,sub_assign,rhs) }}

impl<T : Mul<T,Output = T>> Mul<Self> for C<T> { type Output=C<T>; fn mul(self, rhs: Self) -> Self::Output { op_with_myself!(self,mul,rhs) }}
impl<T : MulAssign> MulAssign<Self> for C<T>{ fn mul_assign(&mut self, rhs: Self) { assign_with_myself!(self,mul_assign,rhs) }}

impl<T : Div<T,Output = T>> Div<Self> for C<T> { type Output=C<T>; fn div(self, rhs: Self) -> Self::Output { op_with_myself!(self,div,rhs) }}
impl<T : DivAssign> DivAssign<Self> for C<T>{ fn div_assign(&mut self, rhs: Self) { assign_with_myself!(self,div_assign,rhs) }}

impl<T : Rem<T,Output = T>> Rem<Self> for C<T> { type Output=C<T>; fn rem(self, rhs: Self) -> Self::Output { op_with_myself!(self,rem,rhs) }}
impl<T : RemAssign> RemAssign<Self> for C<T>{ fn rem_assign(&mut self, rhs: Self) { assign_with_myself!(self,rem_assign,rhs) }}