use std::{fmt::{Debug, Display}, ops::*};
use crate::*;

type C<T> = C3<T>;

macro_rules! op_with_myself { 
    ($myself : ident, $fn_name : ident, $other : expr) => 
    { Self::new($myself.x.$fn_name($other.x), $myself.y.$fn_name($other.y), $myself.z.$fn_name($other.z)) };
}
macro_rules! assign_with_myself {
    ($myself : ident, $fn_name : ident, $other : expr) =>
    { { $myself.x.$fn_name($other.x); $myself.y.$fn_name($other.y); $myself.z.$fn_name($other.z); } };
}



macro_rules! op_with_scalar {
    ($myself : ident, $fn_name : ident, $other : expr) => 
    { Self::Output::new($myself.x.$fn_name($other), $myself.y.$fn_name($other), $myself.z.$fn_name($other)) };
}
macro_rules! assign_with_scalar {
    ($myself : ident, $fn_name : ident, $other : expr) =>
    { { $myself.x.$fn_name($other); $myself.y.$fn_name($other); $myself.z.$fn_name($other); } };
}


macro_rules! op_component {
    ($myself : ident, $op_name : tt) =>
    {  $myself.x $op_name $myself.y $op_name $myself.z };
}

/// 3D Coordinate
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C3<T>
{
    pub x : T,
    pub y : T,
    pub z : T,
}

pub trait Splat3 : Sized + Copy { fn splat3(self) -> C<Self> { C::splat_const(self) }}
impl<T:Copy> Splat3 for T {}

impl<T> From<(T,T,T)> for C<T> { fn from(value: (T,T,T)) -> Self { Self::new(value.0, value.1, value.2) }}
impl<T> From<C<T>> for (T,T,T) { fn from(value: C<T>) -> Self { (value.x, value.y, value.z) }}

impl<T : Copy> From<[T;3]> for C<T> { fn from(value: [T;3]) -> Self { Self::new(value[0], value[1], value[2]) }}
impl<T> From<C<T>> for [T;3] { fn from(value: C<T>) -> Self { [value.x, value.y, value.z] }}

impl<T : Integer + ToReal> From<C<T>> for C<real> { fn from(value: C<T>) -> Self { value.map(|e| e.to_real()) }}

impl<T : Debug>   Debug for C<T>   { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z) }}
impl<T : Display> Display for C<T> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {}, {})", self.x, self.y, self.z) } }

// Point Related
type CIter<T> = C3Iter<T>;
pub struct C3Iter<T>
{
    cur : C<T>,
    end : C<T>,
}

impl<T : Integer> CoordinateInteger for C<T> { fn iter_area(self) -> impl Iterator<Item=Self> { CIter { cur: zero(), end: self } }}
impl<T : Integer> Iterator for CIter<T>
{
    type Item=C<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.z >= self.end.z { return None; }
        let old = self.cur;

        self.cur.x += one();
        if self.cur.x < self.end.x { return Some(old); }

        self.cur.x  = zero();
        self.cur.y += one();
        if self.cur.y < self.end.y { return Some(old); }

        self.cur.y  = zero();
        self.cur.z += one();

        Some(old)
    }
}

// Vector Related
impl C<real>
{
    /// set the angle of the `(x, y)` axis
    pub fn set_angle(&mut self, angle : Angle) { *self = angle.to_vec2(self.length()).to_vec3(self.z) }
}

impl<T : Copy> C<T> 
{
    pub const fn splat_const(xyz : T) -> Self { Self::new(xyz, xyz, xyz) }
}

impl<T> C<T> 
{
    pub const fn new(x : T, y : T, z : T) -> Self { Self { x, y, z }}
    #[inline] pub fn map<Z, F>(self, map_fn : F) -> C<Z> where F : Fn(T) -> Z { C::<Z>::new( map_fn(self.x), map_fn(self.y), map_fn(self.z)) }
}

impl<T : ToReal> C<T> 
{
    pub fn to_vec2(self) -> C2<real> {  C2::<real>::new(self.x.to_real(), self.y.to_real())}
    pub fn to_vec3(self) -> C3<real> { C3::<real>::new(self.x.to_real(), self.y.to_real(), self.z.to_real())}
    pub fn to_vec4(self, w : real) -> C4<real> { C4::<real>::new(self.x.to_real(), self.y.to_real(), self.z.to_real() , w)}
}

impl<T : ToInt> C<T> 
{
    pub fn to_point2(self) -> C2<int> {  C2::<int>::new(self.x.to_int(), self.y.to_int())}
    pub fn to_point3(self) -> C3<int> { C3::<int>::new(self.x.to_int(), self.y.to_int(), self.z.to_int())}
    pub fn to_point4(self, w : int) -> C4<int> { C4::<int>::new(self.x.to_int(), self.y.to_int(), self.z.to_int(), w)}
}

impl<T : ToUint> C<T> 
{
    pub fn to_point2u(self) -> C2<uint> {  C2::<uint>::new(self.x.to_uint(), self.y.to_uint())}
    pub fn to_point3u(self) -> C3<uint> { C3::<uint>::new(self.x.to_uint(), self.y.to_uint(), self.z.to_uint())}
    pub fn to_point4u(self, w : uint) -> C4<uint> { C4::<uint>::new(self.x.to_uint(), self.y.to_uint(), self.z.to_uint() , w)}
}

impl<T : Reversable + HaveZero + Copy> C<T>
{
    pub fn x_x_rx(self) -> Self { Self::new(self.x, self.x, self.x.rev()) }
    pub fn x_x_ry(self) -> Self { Self::new(self.x, self.x, self.y.rev()) }
    pub fn x_x_rz(self) -> Self { Self::new(self.x, self.x, self.z.rev()) }
    pub fn x_y_rx(self) -> Self { Self::new(self.x, self.y, self.x.rev()) }
    pub fn x_y_ry(self) -> Self { Self::new(self.x, self.y, self.y.rev()) }
    pub fn x_y_rz(self) -> Self { Self::new(self.x, self.y, self.z.rev()) }
    pub fn x_z_rx(self) -> Self { Self::new(self.x, self.z, self.x.rev()) }
    pub fn x_z_ry(self) -> Self { Self::new(self.x, self.z, self.y.rev()) }
    pub fn x_z_rz(self) -> Self { Self::new(self.x, self.z, self.z.rev()) }
    pub fn x_rx_x(self) -> Self { Self::new(self.x, self.x.rev(), self.x) }
    pub fn x_rx_y(self) -> Self { Self::new(self.x, self.x.rev(), self.y) }
    pub fn x_rx_z(self) -> Self { Self::new(self.x, self.x.rev(), self.z) }
    pub fn x_rx_rx(self) -> Self { Self::new(self.x, self.x.rev(), self.x.rev()) }
    pub fn x_rx_ry(self) -> Self { Self::new(self.x, self.x.rev(), self.y.rev()) }
    pub fn x_rx_rz(self) -> Self { Self::new(self.x, self.x.rev(), self.z.rev()) }
    pub fn x_rx_0(self) -> Self { Self::new(self.x, self.x.rev(), T::ZERO) }
    pub fn x_ry_x(self) -> Self { Self::new(self.x, self.y.rev(), self.x) }
    pub fn x_ry_y(self) -> Self { Self::new(self.x, self.y.rev(), self.y) }
    pub fn x_ry_z(self) -> Self { Self::new(self.x, self.y.rev(), self.z) }
    pub fn x_ry_rx(self) -> Self { Self::new(self.x, self.y.rev(), self.x.rev()) }
    pub fn x_ry_ry(self) -> Self { Self::new(self.x, self.y.rev(), self.y.rev()) }
    pub fn x_ry_rz(self) -> Self { Self::new(self.x, self.y.rev(), self.z.rev()) }
    pub fn x_ry_0(self) -> Self { Self::new(self.x, self.y.rev(), T::ZERO) }
    pub fn x_rz_x(self) -> Self { Self::new(self.x, self.z.rev(), self.x) }
    pub fn x_rz_y(self) -> Self { Self::new(self.x, self.z.rev(), self.y) }
    pub fn x_rz_z(self) -> Self { Self::new(self.x, self.z.rev(), self.z) }
    pub fn x_rz_rx(self) -> Self { Self::new(self.x, self.z.rev(), self.x.rev()) }
    pub fn x_rz_ry(self) -> Self { Self::new(self.x, self.z.rev(), self.y.rev()) }
    pub fn x_rz_rz(self) -> Self { Self::new(self.x, self.z.rev(), self.z.rev()) }
    pub fn x_rz_0(self) -> Self { Self::new(self.x, self.z.rev(), T::ZERO) }
    pub fn x_0_rx(self) -> Self { Self::new(self.x, T::ZERO, self.x.rev()) }
    pub fn x_0_ry(self) -> Self { Self::new(self.x, T::ZERO, self.y.rev()) }
    pub fn x_0_rz(self) -> Self { Self::new(self.x, T::ZERO, self.z.rev()) }
    pub fn y_x_rx(self) -> Self { Self::new(self.y, self.x, self.x.rev()) }
    pub fn y_x_ry(self) -> Self { Self::new(self.y, self.x, self.y.rev()) }
    pub fn y_x_rz(self) -> Self { Self::new(self.y, self.x, self.z.rev()) }
    pub fn y_y_rx(self) -> Self { Self::new(self.y, self.y, self.x.rev()) }
    pub fn y_y_ry(self) -> Self { Self::new(self.y, self.y, self.y.rev()) }
    pub fn y_y_rz(self) -> Self { Self::new(self.y, self.y, self.z.rev()) }
    pub fn y_z_rx(self) -> Self { Self::new(self.y, self.z, self.x.rev()) }
    pub fn y_z_ry(self) -> Self { Self::new(self.y, self.z, self.y.rev()) }
    pub fn y_z_rz(self) -> Self { Self::new(self.y, self.z, self.z.rev()) }
    pub fn y_rx_x(self) -> Self { Self::new(self.y, self.x.rev(), self.x) }
    pub fn y_rx_y(self) -> Self { Self::new(self.y, self.x.rev(), self.y) }
    pub fn y_rx_z(self) -> Self { Self::new(self.y, self.x.rev(), self.z) }
    pub fn y_rx_rx(self) -> Self { Self::new(self.y, self.x.rev(), self.x.rev()) }
    pub fn y_rx_ry(self) -> Self { Self::new(self.y, self.x.rev(), self.y.rev()) }
    pub fn y_rx_rz(self) -> Self { Self::new(self.y, self.x.rev(), self.z.rev()) }
    pub fn y_rx_0(self) -> Self { Self::new(self.y, self.x.rev(), T::ZERO) }
    pub fn y_ry_x(self) -> Self { Self::new(self.y, self.y.rev(), self.x) }
    pub fn y_ry_y(self) -> Self { Self::new(self.y, self.y.rev(), self.y) }
    pub fn y_ry_z(self) -> Self { Self::new(self.y, self.y.rev(), self.z) }
    pub fn y_ry_rx(self) -> Self { Self::new(self.y, self.y.rev(), self.x.rev()) }
    pub fn y_ry_ry(self) -> Self { Self::new(self.y, self.y.rev(), self.y.rev()) }
    pub fn y_ry_rz(self) -> Self { Self::new(self.y, self.y.rev(), self.z.rev()) }
    pub fn y_ry_0(self) -> Self { Self::new(self.y, self.y.rev(), T::ZERO) }
    pub fn y_rz_x(self) -> Self { Self::new(self.y, self.z.rev(), self.x) }
    pub fn y_rz_y(self) -> Self { Self::new(self.y, self.z.rev(), self.y) }
    pub fn y_rz_z(self) -> Self { Self::new(self.y, self.z.rev(), self.z) }
    pub fn y_rz_rx(self) -> Self { Self::new(self.y, self.z.rev(), self.x.rev()) }
    pub fn y_rz_ry(self) -> Self { Self::new(self.y, self.z.rev(), self.y.rev()) }
    pub fn y_rz_rz(self) -> Self { Self::new(self.y, self.z.rev(), self.z.rev()) }
    pub fn y_rz_0(self) -> Self { Self::new(self.y, self.z.rev(), T::ZERO) }
    pub fn y_0_rx(self) -> Self { Self::new(self.y, T::ZERO, self.x.rev()) }
    pub fn y_0_ry(self) -> Self { Self::new(self.y, T::ZERO, self.y.rev()) }
    pub fn y_0_rz(self) -> Self { Self::new(self.y, T::ZERO, self.z.rev()) }
    pub fn z_x_rx(self) -> Self { Self::new(self.z, self.x, self.x.rev()) }
    pub fn z_x_ry(self) -> Self { Self::new(self.z, self.x, self.y.rev()) }
    pub fn z_x_rz(self) -> Self { Self::new(self.z, self.x, self.z.rev()) }
    pub fn z_y_rx(self) -> Self { Self::new(self.z, self.y, self.x.rev()) }
    pub fn z_y_ry(self) -> Self { Self::new(self.z, self.y, self.y.rev()) }
    pub fn z_y_rz(self) -> Self { Self::new(self.z, self.y, self.z.rev()) }
    pub fn z_z_rx(self) -> Self { Self::new(self.z, self.z, self.x.rev()) }
    pub fn z_z_ry(self) -> Self { Self::new(self.z, self.z, self.y.rev()) }
    pub fn z_z_rz(self) -> Self { Self::new(self.z, self.z, self.z.rev()) }
    pub fn z_rx_x(self) -> Self { Self::new(self.z, self.x.rev(), self.x) }
    pub fn z_rx_y(self) -> Self { Self::new(self.z, self.x.rev(), self.y) }
    pub fn z_rx_z(self) -> Self { Self::new(self.z, self.x.rev(), self.z) }
    pub fn z_rx_rx(self) -> Self { Self::new(self.z, self.x.rev(), self.x.rev()) }
    pub fn z_rx_ry(self) -> Self { Self::new(self.z, self.x.rev(), self.y.rev()) }
    pub fn z_rx_rz(self) -> Self { Self::new(self.z, self.x.rev(), self.z.rev()) }
    pub fn z_rx_0(self) -> Self { Self::new(self.z, self.x.rev(), T::ZERO) }
    pub fn z_ry_x(self) -> Self { Self::new(self.z, self.y.rev(), self.x) }
    pub fn z_ry_y(self) -> Self { Self::new(self.z, self.y.rev(), self.y) }
    pub fn z_ry_z(self) -> Self { Self::new(self.z, self.y.rev(), self.z) }
    pub fn z_ry_rx(self) -> Self { Self::new(self.z, self.y.rev(), self.x.rev()) }
    pub fn z_ry_ry(self) -> Self { Self::new(self.z, self.y.rev(), self.y.rev()) }
    pub fn z_ry_rz(self) -> Self { Self::new(self.z, self.y.rev(), self.z.rev()) }
    pub fn z_ry_0(self) -> Self { Self::new(self.z, self.y.rev(), T::ZERO) }
    pub fn z_rz_x(self) -> Self { Self::new(self.z, self.z.rev(), self.x) }
    pub fn z_rz_y(self) -> Self { Self::new(self.z, self.z.rev(), self.y) }
    pub fn z_rz_z(self) -> Self { Self::new(self.z, self.z.rev(), self.z) }
    pub fn z_rz_rx(self) -> Self { Self::new(self.z, self.z.rev(), self.x.rev()) }
    pub fn z_rz_ry(self) -> Self { Self::new(self.z, self.z.rev(), self.y.rev()) }
    pub fn z_rz_rz(self) -> Self { Self::new(self.z, self.z.rev(), self.z.rev()) }
    pub fn z_rz_0(self) -> Self { Self::new(self.z, self.z.rev(), T::ZERO) }
    pub fn z_0_rx(self) -> Self { Self::new(self.z, T::ZERO, self.x.rev()) }
    pub fn z_0_ry(self) -> Self { Self::new(self.z, T::ZERO, self.y.rev()) }
    pub fn z_0_rz(self) -> Self { Self::new(self.z, T::ZERO, self.z.rev()) }
    pub fn rx_x_x(self) -> Self { Self::new(self.x.rev(), self.x, self.x) }
    pub fn rx_x_y(self) -> Self { Self::new(self.x.rev(), self.x, self.y) }
    pub fn rx_x_z(self) -> Self { Self::new(self.x.rev(), self.x, self.z) }
    pub fn rx_x_rx(self) -> Self { Self::new(self.x.rev(), self.x, self.x.rev()) }
    pub fn rx_x_ry(self) -> Self { Self::new(self.x.rev(), self.x, self.y.rev()) }
    pub fn rx_x_rz(self) -> Self { Self::new(self.x.rev(), self.x, self.z.rev()) }
    pub fn rx_x_0(self) -> Self { Self::new(self.x.rev(), self.x, T::ZERO) }
    pub fn rx_y_x(self) -> Self { Self::new(self.x.rev(), self.y, self.x) }
    pub fn rx_y_y(self) -> Self { Self::new(self.x.rev(), self.y, self.y) }
    pub fn rx_y_z(self) -> Self { Self::new(self.x.rev(), self.y, self.z) }
    pub fn rx_y_rx(self) -> Self { Self::new(self.x.rev(), self.y, self.x.rev()) }
    pub fn rx_y_ry(self) -> Self { Self::new(self.x.rev(), self.y, self.y.rev()) }
    pub fn rx_y_rz(self) -> Self { Self::new(self.x.rev(), self.y, self.z.rev()) }
    pub fn rx_y_0(self) -> Self { Self::new(self.x.rev(), self.y, T::ZERO) }
    pub fn rx_z_x(self) -> Self { Self::new(self.x.rev(), self.z, self.x) }
    pub fn rx_z_y(self) -> Self { Self::new(self.x.rev(), self.z, self.y) }
    pub fn rx_z_z(self) -> Self { Self::new(self.x.rev(), self.z, self.z) }
    pub fn rx_z_rx(self) -> Self { Self::new(self.x.rev(), self.z, self.x.rev()) }
    pub fn rx_z_ry(self) -> Self { Self::new(self.x.rev(), self.z, self.y.rev()) }
    pub fn rx_z_rz(self) -> Self { Self::new(self.x.rev(), self.z, self.z.rev()) }
    pub fn rx_z_0(self) -> Self { Self::new(self.x.rev(), self.z, T::ZERO) }
    pub fn rx_rx_x(self) -> Self { Self::new(self.x.rev(), self.x.rev(), self.x) }
    pub fn rx_rx_y(self) -> Self { Self::new(self.x.rev(), self.x.rev(), self.y) }
    pub fn rx_rx_z(self) -> Self { Self::new(self.x.rev(), self.x.rev(), self.z) }
    pub fn rx_rx_rx(self) -> Self { Self::new(self.x.rev(), self.x.rev(), self.x.rev()) }
    pub fn rx_rx_ry(self) -> Self { Self::new(self.x.rev(), self.x.rev(), self.y.rev()) }
    pub fn rx_rx_rz(self) -> Self { Self::new(self.x.rev(), self.x.rev(), self.z.rev()) }
    pub fn rx_rx_0(self) -> Self { Self::new(self.x.rev(), self.x.rev(), T::ZERO) }
    pub fn rx_ry_x(self) -> Self { Self::new(self.x.rev(), self.y.rev(), self.x) }
    pub fn rx_ry_y(self) -> Self { Self::new(self.x.rev(), self.y.rev(), self.y) }
    pub fn rx_ry_z(self) -> Self { Self::new(self.x.rev(), self.y.rev(), self.z) }
    pub fn rx_ry_rx(self) -> Self { Self::new(self.x.rev(), self.y.rev(), self.x.rev()) }
    pub fn rx_ry_ry(self) -> Self { Self::new(self.x.rev(), self.y.rev(), self.y.rev()) }
    pub fn rx_ry_rz(self) -> Self { Self::new(self.x.rev(), self.y.rev(), self.z.rev()) }
    pub fn rx_ry_0(self) -> Self { Self::new(self.x.rev(), self.y.rev(), T::ZERO) }
    pub fn rx_rz_x(self) -> Self { Self::new(self.x.rev(), self.z.rev(), self.x) }
    pub fn rx_rz_y(self) -> Self { Self::new(self.x.rev(), self.z.rev(), self.y) }
    pub fn rx_rz_z(self) -> Self { Self::new(self.x.rev(), self.z.rev(), self.z) }
    pub fn rx_rz_rx(self) -> Self { Self::new(self.x.rev(), self.z.rev(), self.x.rev()) }
    pub fn rx_rz_ry(self) -> Self { Self::new(self.x.rev(), self.z.rev(), self.y.rev()) }
    pub fn rx_rz_rz(self) -> Self { Self::new(self.x.rev(), self.z.rev(), self.z.rev()) }
    pub fn rx_rz_0(self) -> Self { Self::new(self.x.rev(), self.z.rev(), T::ZERO) }
    pub fn rx_0_x(self) -> Self { Self::new(self.x.rev(), T::ZERO, self.x) }
    pub fn rx_0_y(self) -> Self { Self::new(self.x.rev(), T::ZERO, self.y) }
    pub fn rx_0_z(self) -> Self { Self::new(self.x.rev(), T::ZERO, self.z) }
    pub fn rx_0_rx(self) -> Self { Self::new(self.x.rev(), T::ZERO, self.x.rev()) }
    pub fn rx_0_ry(self) -> Self { Self::new(self.x.rev(), T::ZERO, self.y.rev()) }
    pub fn rx_0_rz(self) -> Self { Self::new(self.x.rev(), T::ZERO, self.z.rev()) }
    pub fn rx_0_0(self) -> Self { Self::new(self.x.rev(), T::ZERO, T::ZERO) }
    pub fn ry_x_x(self) -> Self { Self::new(self.y.rev(), self.x, self.x) }
    pub fn ry_x_y(self) -> Self { Self::new(self.y.rev(), self.x, self.y) }
    pub fn ry_x_z(self) -> Self { Self::new(self.y.rev(), self.x, self.z) }
    pub fn ry_x_rx(self) -> Self { Self::new(self.y.rev(), self.x, self.x.rev()) }
    pub fn ry_x_ry(self) -> Self { Self::new(self.y.rev(), self.x, self.y.rev()) }
    pub fn ry_x_rz(self) -> Self { Self::new(self.y.rev(), self.x, self.z.rev()) }
    pub fn ry_x_0(self) -> Self { Self::new(self.y.rev(), self.x, T::ZERO) }
    pub fn ry_y_x(self) -> Self { Self::new(self.y.rev(), self.y, self.x) }
    pub fn ry_y_y(self) -> Self { Self::new(self.y.rev(), self.y, self.y) }
    pub fn ry_y_z(self) -> Self { Self::new(self.y.rev(), self.y, self.z) }
    pub fn ry_y_rx(self) -> Self { Self::new(self.y.rev(), self.y, self.x.rev()) }
    pub fn ry_y_ry(self) -> Self { Self::new(self.y.rev(), self.y, self.y.rev()) }
    pub fn ry_y_rz(self) -> Self { Self::new(self.y.rev(), self.y, self.z.rev()) }
    pub fn ry_y_0(self) -> Self { Self::new(self.y.rev(), self.y, T::ZERO) }
    pub fn ry_z_x(self) -> Self { Self::new(self.y.rev(), self.z, self.x) }
    pub fn ry_z_y(self) -> Self { Self::new(self.y.rev(), self.z, self.y) }
    pub fn ry_z_z(self) -> Self { Self::new(self.y.rev(), self.z, self.z) }
    pub fn ry_z_rx(self) -> Self { Self::new(self.y.rev(), self.z, self.x.rev()) }
    pub fn ry_z_ry(self) -> Self { Self::new(self.y.rev(), self.z, self.y.rev()) }
    pub fn ry_z_rz(self) -> Self { Self::new(self.y.rev(), self.z, self.z.rev()) }
    pub fn ry_z_0(self) -> Self { Self::new(self.y.rev(), self.z, T::ZERO) }
    pub fn ry_rx_x(self) -> Self { Self::new(self.y.rev(), self.x.rev(), self.x) }
    pub fn ry_rx_y(self) -> Self { Self::new(self.y.rev(), self.x.rev(), self.y) }
    pub fn ry_rx_z(self) -> Self { Self::new(self.y.rev(), self.x.rev(), self.z) }
    pub fn ry_rx_rx(self) -> Self { Self::new(self.y.rev(), self.x.rev(), self.x.rev()) }
    pub fn ry_rx_ry(self) -> Self { Self::new(self.y.rev(), self.x.rev(), self.y.rev()) }
    pub fn ry_rx_rz(self) -> Self { Self::new(self.y.rev(), self.x.rev(), self.z.rev()) }
    pub fn ry_rx_0(self) -> Self { Self::new(self.y.rev(), self.x.rev(), T::ZERO) }
    pub fn ry_ry_x(self) -> Self { Self::new(self.y.rev(), self.y.rev(), self.x) }
    pub fn ry_ry_y(self) -> Self { Self::new(self.y.rev(), self.y.rev(), self.y) }
    pub fn ry_ry_z(self) -> Self { Self::new(self.y.rev(), self.y.rev(), self.z) }
    pub fn ry_ry_rx(self) -> Self { Self::new(self.y.rev(), self.y.rev(), self.x.rev()) }
    pub fn ry_ry_ry(self) -> Self { Self::new(self.y.rev(), self.y.rev(), self.y.rev()) }
    pub fn ry_ry_rz(self) -> Self { Self::new(self.y.rev(), self.y.rev(), self.z.rev()) }
    pub fn ry_ry_0(self) -> Self { Self::new(self.y.rev(), self.y.rev(), T::ZERO) }
    pub fn ry_rz_x(self) -> Self { Self::new(self.y.rev(), self.z.rev(), self.x) }
    pub fn ry_rz_y(self) -> Self { Self::new(self.y.rev(), self.z.rev(), self.y) }
    pub fn ry_rz_z(self) -> Self { Self::new(self.y.rev(), self.z.rev(), self.z) }
    pub fn ry_rz_rx(self) -> Self { Self::new(self.y.rev(), self.z.rev(), self.x.rev()) }
    pub fn ry_rz_ry(self) -> Self { Self::new(self.y.rev(), self.z.rev(), self.y.rev()) }
    pub fn ry_rz_rz(self) -> Self { Self::new(self.y.rev(), self.z.rev(), self.z.rev()) }
    pub fn ry_rz_0(self) -> Self { Self::new(self.y.rev(), self.z.rev(), T::ZERO) }
    pub fn ry_0_x(self) -> Self { Self::new(self.y.rev(), T::ZERO, self.x) }
    pub fn ry_0_y(self) -> Self { Self::new(self.y.rev(), T::ZERO, self.y) }
    pub fn ry_0_z(self) -> Self { Self::new(self.y.rev(), T::ZERO, self.z) }
    pub fn ry_0_rx(self) -> Self { Self::new(self.y.rev(), T::ZERO, self.x.rev()) }
    pub fn ry_0_ry(self) -> Self { Self::new(self.y.rev(), T::ZERO, self.y.rev()) }
    pub fn ry_0_rz(self) -> Self { Self::new(self.y.rev(), T::ZERO, self.z.rev()) }
    pub fn ry_0_0(self) -> Self { Self::new(self.y.rev(), T::ZERO, T::ZERO) }
    pub fn rz_x_x(self) -> Self { Self::new(self.z.rev(), self.x, self.x) }
    pub fn rz_x_y(self) -> Self { Self::new(self.z.rev(), self.x, self.y) }
    pub fn rz_x_z(self) -> Self { Self::new(self.z.rev(), self.x, self.z) }
    pub fn rz_x_rx(self) -> Self { Self::new(self.z.rev(), self.x, self.x.rev()) }
    pub fn rz_x_ry(self) -> Self { Self::new(self.z.rev(), self.x, self.y.rev()) }
    pub fn rz_x_rz(self) -> Self { Self::new(self.z.rev(), self.x, self.z.rev()) }
    pub fn rz_x_0(self) -> Self { Self::new(self.z.rev(), self.x, T::ZERO) }
    pub fn rz_y_x(self) -> Self { Self::new(self.z.rev(), self.y, self.x) }
    pub fn rz_y_y(self) -> Self { Self::new(self.z.rev(), self.y, self.y) }
    pub fn rz_y_z(self) -> Self { Self::new(self.z.rev(), self.y, self.z) }
    pub fn rz_y_rx(self) -> Self { Self::new(self.z.rev(), self.y, self.x.rev()) }
    pub fn rz_y_ry(self) -> Self { Self::new(self.z.rev(), self.y, self.y.rev()) }
    pub fn rz_y_rz(self) -> Self { Self::new(self.z.rev(), self.y, self.z.rev()) }
    pub fn rz_y_0(self) -> Self { Self::new(self.z.rev(), self.y, T::ZERO) }
    pub fn rz_z_x(self) -> Self { Self::new(self.z.rev(), self.z, self.x) }
    pub fn rz_z_y(self) -> Self { Self::new(self.z.rev(), self.z, self.y) }
    pub fn rz_z_z(self) -> Self { Self::new(self.z.rev(), self.z, self.z) }
    pub fn rz_z_rx(self) -> Self { Self::new(self.z.rev(), self.z, self.x.rev()) }
    pub fn rz_z_ry(self) -> Self { Self::new(self.z.rev(), self.z, self.y.rev()) }
    pub fn rz_z_rz(self) -> Self { Self::new(self.z.rev(), self.z, self.z.rev()) }
    pub fn rz_z_0(self) -> Self { Self::new(self.z.rev(), self.z, T::ZERO) }
    pub fn rz_rx_x(self) -> Self { Self::new(self.z.rev(), self.x.rev(), self.x) }
    pub fn rz_rx_y(self) -> Self { Self::new(self.z.rev(), self.x.rev(), self.y) }
    pub fn rz_rx_z(self) -> Self { Self::new(self.z.rev(), self.x.rev(), self.z) }
    pub fn rz_rx_rx(self) -> Self { Self::new(self.z.rev(), self.x.rev(), self.x.rev()) }
    pub fn rz_rx_ry(self) -> Self { Self::new(self.z.rev(), self.x.rev(), self.y.rev()) }
    pub fn rz_rx_rz(self) -> Self { Self::new(self.z.rev(), self.x.rev(), self.z.rev()) }
    pub fn rz_rx_0(self) -> Self { Self::new(self.z.rev(), self.x.rev(), T::ZERO) }
    pub fn rz_ry_x(self) -> Self { Self::new(self.z.rev(), self.y.rev(), self.x) }
    pub fn rz_ry_y(self) -> Self { Self::new(self.z.rev(), self.y.rev(), self.y) }
    pub fn rz_ry_z(self) -> Self { Self::new(self.z.rev(), self.y.rev(), self.z) }
    pub fn rz_ry_rx(self) -> Self { Self::new(self.z.rev(), self.y.rev(), self.x.rev()) }
    pub fn rz_ry_ry(self) -> Self { Self::new(self.z.rev(), self.y.rev(), self.y.rev()) }
    pub fn rz_ry_rz(self) -> Self { Self::new(self.z.rev(), self.y.rev(), self.z.rev()) }
    pub fn rz_ry_0(self) -> Self { Self::new(self.z.rev(), self.y.rev(), T::ZERO) }
    pub fn rz_rz_x(self) -> Self { Self::new(self.z.rev(), self.z.rev(), self.x) }
    pub fn rz_rz_y(self) -> Self { Self::new(self.z.rev(), self.z.rev(), self.y) }
    pub fn rz_rz_z(self) -> Self { Self::new(self.z.rev(), self.z.rev(), self.z) }
    pub fn rz_rz_rx(self) -> Self { Self::new(self.z.rev(), self.z.rev(), self.x.rev()) }
    pub fn rz_rz_ry(self) -> Self { Self::new(self.z.rev(), self.z.rev(), self.y.rev()) }
    pub fn rz_rz_rz(self) -> Self { Self::new(self.z.rev(), self.z.rev(), self.z.rev()) }
    pub fn rz_rz_0(self) -> Self { Self::new(self.z.rev(), self.z.rev(), T::ZERO) }
    pub fn rz_0_x(self) -> Self { Self::new(self.z.rev(), T::ZERO, self.x) }
    pub fn rz_0_y(self) -> Self { Self::new(self.z.rev(), T::ZERO, self.y) }
    pub fn rz_0_z(self) -> Self { Self::new(self.z.rev(), T::ZERO, self.z) }
    pub fn rz_0_rx(self) -> Self { Self::new(self.z.rev(), T::ZERO, self.x.rev()) }
    pub fn rz_0_ry(self) -> Self { Self::new(self.z.rev(), T::ZERO, self.y.rev()) }
    pub fn rz_0_rz(self) -> Self { Self::new(self.z.rev(), T::ZERO, self.z.rev()) }
    pub fn rz_0_0(self) -> Self { Self::new(self.z.rev(), T::ZERO, T::ZERO) }
    pub fn _0_x_rx(self) -> Self { Self::new(T::ZERO, self.x, self.x.rev()) }
    pub fn _0_x_ry(self) -> Self { Self::new(T::ZERO, self.x, self.y.rev()) }
    pub fn _0_x_rz(self) -> Self { Self::new(T::ZERO, self.x, self.z.rev()) }
    pub fn _0_y_rx(self) -> Self { Self::new(T::ZERO, self.y, self.x.rev()) }
    pub fn _0_y_ry(self) -> Self { Self::new(T::ZERO, self.y, self.y.rev()) }
    pub fn _0_y_rz(self) -> Self { Self::new(T::ZERO, self.y, self.z.rev()) }
    pub fn _0_z_rx(self) -> Self { Self::new(T::ZERO, self.z, self.x.rev()) }
    pub fn _0_z_ry(self) -> Self { Self::new(T::ZERO, self.z, self.y.rev()) }
    pub fn _0_z_rz(self) -> Self { Self::new(T::ZERO, self.z, self.z.rev()) }
    pub fn _0_rx_x(self) -> Self { Self::new(T::ZERO, self.x.rev(), self.x) }
    pub fn _0_rx_y(self) -> Self { Self::new(T::ZERO, self.x.rev(), self.y) }
    pub fn _0_rx_z(self) -> Self { Self::new(T::ZERO, self.x.rev(), self.z) }
    pub fn _0_rx_rx(self) -> Self { Self::new(T::ZERO, self.x.rev(), self.x.rev()) }
    pub fn _0_rx_ry(self) -> Self { Self::new(T::ZERO, self.x.rev(), self.y.rev()) }
    pub fn _0_rx_rz(self) -> Self { Self::new(T::ZERO, self.x.rev(), self.z.rev()) }
    pub fn _0_rx_0(self) -> Self { Self::new(T::ZERO, self.x.rev(), T::ZERO) }
    pub fn _0_ry_x(self) -> Self { Self::new(T::ZERO, self.y.rev(), self.x) }
    pub fn _0_ry_y(self) -> Self { Self::new(T::ZERO, self.y.rev(), self.y) }
    pub fn _0_ry_z(self) -> Self { Self::new(T::ZERO, self.y.rev(), self.z) }
    pub fn _0_ry_rx(self) -> Self { Self::new(T::ZERO, self.y.rev(), self.x.rev()) }
    pub fn _0_ry_ry(self) -> Self { Self::new(T::ZERO, self.y.rev(), self.y.rev()) }
    pub fn _0_ry_rz(self) -> Self { Self::new(T::ZERO, self.y.rev(), self.z.rev()) }
    pub fn _0_ry_0(self) -> Self { Self::new(T::ZERO, self.y.rev(), T::ZERO) }
    pub fn _0_rz_x(self) -> Self { Self::new(T::ZERO, self.z.rev(), self.x) }
    pub fn _0_rz_y(self) -> Self { Self::new(T::ZERO, self.z.rev(), self.y) }
    pub fn _0_rz_z(self) -> Self { Self::new(T::ZERO, self.z.rev(), self.z) }
    pub fn _0_rz_rx(self) -> Self { Self::new(T::ZERO, self.z.rev(), self.x.rev()) }
    pub fn _0_rz_ry(self) -> Self { Self::new(T::ZERO, self.z.rev(), self.y.rev()) }
    pub fn _0_rz_rz(self) -> Self { Self::new(T::ZERO, self.z.rev(), self.z.rev()) }
    pub fn _0_rz_0(self) -> Self { Self::new(T::ZERO, self.z.rev(), T::ZERO) }
    pub fn _0_0_rx(self) -> Self { Self::new(T::ZERO, T::ZERO, self.x.rev()) }
    pub fn _0_0_ry(self) -> Self { Self::new(T::ZERO, T::ZERO, self.y.rev()) }
    pub fn _0_0_rz(self) -> Self { Self::new(T::ZERO, T::ZERO, self.z.rev()) }
}

impl<T : HaveZero + Copy> C<T> 
{ 
    pub fn x_x_x(self) -> Self { Self::new(self.x, self.x, self.x) }
    pub fn x_x_y(self) -> Self { Self::new(self.x, self.x, self.y) }
    pub fn x_x_z(self) -> Self { Self::new(self.x, self.x, self.z) }
    pub fn x_x_0(self) -> Self { Self::new(self.x, self.x, T::ZERO) }
    pub fn x_y_x(self) -> Self { Self::new(self.x, self.y, self.x) }
    pub fn x_y_y(self) -> Self { Self::new(self.x, self.y, self.y) }
    pub fn x_y_z(self) -> Self { Self::new(self.x, self.y, self.z) }
    pub fn x_y_0(self) -> Self { Self::new(self.x, self.y, T::ZERO) }
    pub fn x_z_x(self) -> Self { Self::new(self.x, self.z, self.x) }
    pub fn x_z_y(self) -> Self { Self::new(self.x, self.z, self.y) }
    pub fn x_z_z(self) -> Self { Self::new(self.x, self.z, self.z) }
    pub fn x_z_0(self) -> Self { Self::new(self.x, self.z, T::ZERO) }
    pub fn x_0_x(self) -> Self { Self::new(self.x, T::ZERO, self.x) }
    pub fn x_0_y(self) -> Self { Self::new(self.x, T::ZERO, self.y) }
    pub fn x_0_z(self) -> Self { Self::new(self.x, T::ZERO, self.z) }
    pub fn x_0_0(self) -> Self { Self::new(self.x, T::ZERO, T::ZERO) }
    pub fn y_x_x(self) -> Self { Self::new(self.y, self.x, self.x) }
    pub fn y_x_y(self) -> Self { Self::new(self.y, self.x, self.y) }
    pub fn y_x_z(self) -> Self { Self::new(self.y, self.x, self.z) }
    pub fn y_x_0(self) -> Self { Self::new(self.y, self.x, T::ZERO) }
    pub fn y_y_x(self) -> Self { Self::new(self.y, self.y, self.x) }
    pub fn y_y_y(self) -> Self { Self::new(self.y, self.y, self.y) }
    pub fn y_y_z(self) -> Self { Self::new(self.y, self.y, self.z) }
    pub fn y_y_0(self) -> Self { Self::new(self.y, self.y, T::ZERO) }
    pub fn y_z_x(self) -> Self { Self::new(self.y, self.z, self.x) }
    pub fn y_z_y(self) -> Self { Self::new(self.y, self.z, self.y) }
    pub fn y_z_z(self) -> Self { Self::new(self.y, self.z, self.z) }
    pub fn y_z_0(self) -> Self { Self::new(self.y, self.z, T::ZERO) }
    pub fn y_0_x(self) -> Self { Self::new(self.y, T::ZERO, self.x) }
    pub fn y_0_y(self) -> Self { Self::new(self.y, T::ZERO, self.y) }
    pub fn y_0_z(self) -> Self { Self::new(self.y, T::ZERO, self.z) }
    pub fn y_0_0(self) -> Self { Self::new(self.y, T::ZERO, T::ZERO) }
    pub fn z_x_x(self) -> Self { Self::new(self.z, self.x, self.x) }
    pub fn z_x_y(self) -> Self { Self::new(self.z, self.x, self.y) }
    pub fn z_x_z(self) -> Self { Self::new(self.z, self.x, self.z) }
    pub fn z_x_0(self) -> Self { Self::new(self.z, self.x, T::ZERO) }
    pub fn z_y_x(self) -> Self { Self::new(self.z, self.y, self.x) }
    pub fn z_y_y(self) -> Self { Self::new(self.z, self.y, self.y) }
    pub fn z_y_z(self) -> Self { Self::new(self.z, self.y, self.z) }
    pub fn z_y_0(self) -> Self { Self::new(self.z, self.y, T::ZERO) }
    pub fn z_z_x(self) -> Self { Self::new(self.z, self.z, self.x) }
    pub fn z_z_y(self) -> Self { Self::new(self.z, self.z, self.y) }
    pub fn z_z_z(self) -> Self { Self::new(self.z, self.z, self.z) }
    pub fn z_z_0(self) -> Self { Self::new(self.z, self.z, T::ZERO) }
    pub fn z_0_x(self) -> Self { Self::new(self.z, T::ZERO, self.x) }
    pub fn z_0_y(self) -> Self { Self::new(self.z, T::ZERO, self.y) }
    pub fn z_0_z(self) -> Self { Self::new(self.z, T::ZERO, self.z) }
    pub fn z_0_0(self) -> Self { Self::new(self.z, T::ZERO, T::ZERO) }
    pub fn _0_x_x(self) -> Self { Self::new(T::ZERO, self.x, self.x) }
    pub fn _0_x_y(self) -> Self { Self::new(T::ZERO, self.x, self.y) }
    pub fn _0_x_z(self) -> Self { Self::new(T::ZERO, self.x, self.z) }
    pub fn _0_x_0(self) -> Self { Self::new(T::ZERO, self.x, T::ZERO) }
    pub fn _0_y_x(self) -> Self { Self::new(T::ZERO, self.y, self.x) }
    pub fn _0_y_y(self) -> Self { Self::new(T::ZERO, self.y, self.y) }
    pub fn _0_y_z(self) -> Self { Self::new(T::ZERO, self.y, self.z) }
    pub fn _0_y_0(self) -> Self { Self::new(T::ZERO, self.y, T::ZERO) }
    pub fn _0_z_x(self) -> Self { Self::new(T::ZERO, self.z, self.x) }
    pub fn _0_z_y(self) -> Self { Self::new(T::ZERO, self.z, self.y) }
    pub fn _0_z_z(self) -> Self { Self::new(T::ZERO, self.z, self.z) }
    pub fn _0_z_0(self) -> Self { Self::new(T::ZERO, self.z, T::ZERO) }
    pub fn _0_0_x(self) -> Self { Self::new(T::ZERO, T::ZERO, self.x) }
    pub fn _0_0_y(self) -> Self { Self::new(T::ZERO, T::ZERO, self.y) }
    pub fn _0_0_z(self) -> Self { Self::new(T::ZERO, T::ZERO, self.z) }
    pub fn _0_0_0(self) -> Self { Self::new(T::ZERO, T::ZERO, T::ZERO) }
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
            self.z.clamp_partial(mini.z, maxi.z),
        )
    }
}

impl<T : Number> CoordinateNumber for C<T>
{
    fn length_squared(self) -> T { self.x*self.x + self.y*self.y + self.z*self.z }
    fn area(self) -> Self::Precision { (op_component!(self, *)).absolute() }
    
    #[inline] fn min_element(self) -> Self::Precision { self.x().min_partial(self.y().min_partial(self.z())) }
    #[inline] fn max_element(self) -> Self::Precision { self.x().max_partial(self.y().max_partial(self.z())) }
}

impl<T : UnitArithmetic> CoordinateUnit for C<T>
{
    type Precision=T;
    fn sum_axis(self) -> Self::Precision { op_component!(self, +) }
    fn length_manhattan(self) -> Self::Precision { self.absolute().sum_axis() }
    
    #[inline] fn splat(xyz : Self::Precision) -> Self { Self::splat_const(xyz) }
    
    /// Unsigned point
    type UPoint= C<uint>;

    /// Signed point
    type IPoint = C<int>;

    /// Signed Vector
    type Vec   = C<real>;
    
    const X : Self = Self::new(T::ONE, T::ZERO, T::ZERO);
    const Y : Self = Self::new(T::ZERO, T::ONE, T::ZERO);
    const Z : Self = Self::new(T::ZERO, T::ZERO, T::ONE);
    const W : Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
    
    #[inline] fn x(self) -> Self::Precision { self.x }
    #[inline] fn y(self) -> Self::Precision { self.y }
    #[inline] fn z(self) -> Self::Precision { self.z }
    #[inline] fn w(self) -> Self::Precision { T::ZERO }

    #[inline] fn set_x(&mut self, x : Self::Precision) -> &mut Self { self.x = x; self }
    #[inline] fn set_y(&mut self, y : Self::Precision) -> &mut Self { self.y = y; self }
    #[inline] fn set_z(&mut self, z : Self::Precision) -> &mut Self { self.z = z; self }
    #[inline] fn set_w(&mut self, _w : Self::Precision) -> &mut Self { self }

    #[inline] fn with_x(mut self, x : Self::Precision) -> Self { self.x = x; self }
    #[inline] fn with_y(mut self, y : Self::Precision) -> Self { self.y = y; self }
    #[inline] fn with_z(mut self, z : Self::Precision) -> Self { self.z = z; self }
    #[inline] fn with_w(self, _w : Self::Precision) -> Self { self }

    fn left(self) -> Self { self-Self::X }
    fn down(self) -> Self { self-Self::Y }

    fn any<F>(self, any_fn : F) -> bool where F : Fn(Self::Precision) -> bool { any_fn(self.x) || any_fn(self.y) || any_fn(self.z) }
    fn all<F>(self, all_fn : F) -> bool where F : Fn(Self::Precision) -> bool { all_fn(self.x) && all_fn(self.y) && all_fn(self.z) }

    fn cmp_any<F>(self, other : Self, any_fn : F) -> bool where F : Fn(Self::Precision, Self::Precision) -> bool 
    { any_fn(self.x, other.x) || any_fn(self.y, other.y) || any_fn(self.z, other.z) }
    fn cmp_all<F>(self, other : Self, any_fn : F) -> bool where F : Fn(Self::Precision, Self::Precision) -> bool
    { any_fn(self.x, other.x) && any_fn(self.y, other.y) && any_fn(self.z, other.z) }

    fn new_2d(x : Self::Precision, y : Self::Precision) -> Self { Self::new(x, y, Self::Precision::ZERO) }
    fn new_3d(x : Self::Precision, y : Self::Precision, z : Self::Precision) -> Self { Self::new(x, y, z) }
    fn new_4d(x : Self::Precision, y : Self::Precision, z : Self::Precision, _w : Self::Precision) -> Self { Self::new(x, y, z) }
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