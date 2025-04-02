use std::ops::*;

use crate::*;

pub type Color = Colored<real>;
pub type Pixel = Colored<u8>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Colored<T : Scalar>
{
    pub r : T,
    pub g : T,
    pub b : T,
    pub a : T,
}

impl<T : Scalar> Default for Colored<T>
{ 
    fn default() -> Self { Self::WHITE }
}

impl<T : Scalar> Colored<T>
{
    pub const BLACK : Self = Self { r: T::MIN_RANGE_VAL, g: T::MIN_RANGE_VAL, b: T::MIN_RANGE_VAL, a: T::MAX_RANGE_VAL };
    pub const WHITE : Self = Self { r: T::MAX_RANGE_VAL, g: T::MAX_RANGE_VAL, b: T::MAX_RANGE_VAL, a: T::MAX_RANGE_VAL };
    
    pub const RED    : Self = Self::new_rgba(T::MAX_RANGE_VAL, T::MIN_RANGE_VAL, T::MIN_RANGE_VAL, T::MAX_RANGE_VAL);
    pub const GREEN  : Self = Self::new_rgba(T::MIN_RANGE_VAL, T::MAX_RANGE_VAL, T::MIN_RANGE_VAL, T::MAX_RANGE_VAL);
    pub const BLUE   : Self = Self::new_rgba(T::MIN_RANGE_VAL, T::MIN_RANGE_VAL, T::MAX_RANGE_VAL, T::MAX_RANGE_VAL);
    
    pub const CYAN   : Self = Self::new_rgba(T::MIN_RANGE_VAL, T::MAX_RANGE_VAL, T::MAX_RANGE_VAL, T::MAX_RANGE_VAL);
    pub const PINK   : Self = Self::new_rgba(T::MAX_RANGE_VAL, T::MIN_RANGE_VAL, T::MAX_RANGE_VAL, T::MAX_RANGE_VAL);
    pub const YELLOW : Self = Self::new_rgba(T::MAX_RANGE_VAL, T::MAX_RANGE_VAL, T::MIN_RANGE_VAL, T::MAX_RANGE_VAL);
    
    
    /// Transparent Black
    pub const TRANSPARENT : Self = Self::new_rgba(T::MAX_RANGE_VAL, T::MAX_RANGE_VAL, T::MAX_RANGE_VAL, T::MAX_RANGE_VAL);
    

    /// Component belong to `[MIN_RANGE_VAL, MAX_RANGE_VAL]` : 
    /// 
    /// ie : 
    /// 
    /// `[MIN, MAX]` for integer,
    /// `[0. .. 1.]` for floating
    #[inline] pub const fn new(r : T, g : T, b : T, a : T) -> Self 
    { Self::new_rgba(r, g, b, a) }

    #[inline] pub const fn new_rgba(r : T, g : T, b : T, a : T) -> Self { Self { r, g, b, a } }
    #[inline] pub const fn new_rgb(r : T, g : T, b : T) -> Self { Self::new_rgba(r, g, b, T::MAX_RANGE_VAL) }
    #[inline] pub const fn new_greyscale(rgb : T) -> Self { Self::new_rgb(rgb, rgb, rgb) }

    pub fn to_hsl(self) -> ColorHSL
    {
        // Thank to MacroQuad, the following code was copied and edited the code from the MacroQuad crate
        let mut h: real;
        let s: real;
        let l: real;

        let (r, g, b, a) = self.unpack_rgba_coef();

        let max = self.to_vec3_coef().max_element();
        let min = self.to_vec3_coef().min_element();

        // Luminosity is the average of the max and min rgb color intensities.
        l = (max + min) / 2.0;

        // Saturation
        let delta = max - min;
        if delta == 0.0 { return ColorHSL::new(0.0, 0.0, l, a); }

        // it's not gray
        if l < 0.5 {
            s = delta / (max + min);
        } else {
            s = delta / (2.0 - max - min);
        }

        // Hue
        let r2 = (((max - r) / 6.0) + (delta / 2.0)) / delta;
        let g2 = (((max - g) / 6.0) + (delta / 2.0)) / delta;
        let b2 = (((max - b) / 6.0) + (delta / 2.0)) / delta;

        h = match max {
            x if x == r => b2 - g2,
            x if x == g => (1.0 / 3.0) + r2 - b2,
            _ => (2.0 / 3.0) + g2 - r2,
        };

        // Fix wraparounds
        if h < 0. { h += 1.0; } else if h > 1. { h -= 1.0; }

        ColorHSL::new(h, s, l, a)
    }

    pub const fn r(self) -> T { self.r }
    pub const fn with_r(mut self, r : T) -> Self { self.r = r; self }
    pub fn set_r(&mut self, r : T) -> &mut Self { self.r = r; self }

    pub const fn g(self) -> T { self.g }
    pub const fn with_g(mut self, g : T) -> Self { self.g = g; self }
    pub fn set_g(&mut self, g : T) -> &mut Self { self.g = g; self }

    pub const fn b(self) -> T { self.b }
    pub const fn with_b(mut self, b : T) -> Self { self.b = b; self }
    pub fn set_b(&mut self, b : T) -> &mut Self { self.b = b; self }

    pub const fn a(self) -> T { self.a }
    pub const fn with_a(mut self, a : T) -> Self { self.a = a; self }
    pub fn set_a(&mut self, a : T) -> &mut Self { self.a = a; self }

    pub const fn unpack_rgb(self) -> (T, T, T) { (self.r, self.g, self.b) }
    pub const fn unpack_rgba(self) -> (T, T, T, T) { (self.r, self.g, self.b, self.a) }

    pub fn unpack_rgb_coef(self) -> (Coef, Coef, Coef) { (self.r.to_coef(), self.g.to_coef(), self.b.to_coef()) }
    pub fn unpack_rgba_coef(self) -> (Coef, Coef, Coef, Coef) { (self.r.to_coef(), self.g.to_coef(), self.b.to_coef(), self.a.to_coef()) }

    pub fn from_rgb_hex(hex: u32) -> Self 
    {
        let bytes: [u8; 4] = hex.to_be_bytes();
        Self::new
        (
            T::from_coef(bytes[1].to_coef()),
            T::from_coef(bytes[2].to_coef()),
            T::from_coef(bytes[3].to_coef()),
            T::MAX_RANGE_VAL,
        )
    }
    pub fn from_rgba_hex(hex: u32) -> Self 
    {
        let bytes: [u8; 4] = hex.to_be_bytes();
        Self::new
        (
            T::from_coef(bytes[0].to_coef()), 
            T::from_coef(bytes[1].to_coef()),
            T::from_coef(bytes[2].to_coef()),
            T::from_coef(bytes[3].to_coef())
        )
    }
}


impl<T:Scalar> UnitArithmetic<Self> for Colored<T> {}

impl<T:Scalar> HaveOne for Colored<T> { const ONE  : Self = Self::new_rgba(T::MAX_RANGE_VAL, T::MAX_RANGE_VAL, T::MAX_RANGE_VAL, T::MAX_RANGE_VAL); }
impl<T:Scalar> HaveZero for Colored<T> { const ZERO : Self = Self::new_rgba(T::MIN_RANGE_VAL, T::MIN_RANGE_VAL, T::MIN_RANGE_VAL, T::MIN_RANGE_VAL); }
impl<T:Scalar> Absolute for Colored<T> { fn absolute(self) -> Self { self.to_c4().absolute().into() }}

impl<T:Scalar> Add<Self> for Colored<T> { type Output=Self; fn add(self, rhs: Colored<T>) -> Self::Output { Self { r : self.r.add(rhs.r), g : self.g.add(rhs.g), b : self.b.add(rhs.b), a : self.a.add(rhs.a) } }}
impl<T:Scalar> AddAssign<Self> for Colored<T> { fn add_assign(&mut self, rhs: Self) { *self = self.add(rhs); }}

impl<T:Scalar> Sub<Self> for Colored<T> { type Output=Self; fn sub(self, rhs: Colored<T>) -> Self::Output { Self { r : self.r.sub(rhs.r), g : self.g.sub(rhs.g), b : self.b.sub(rhs.b), a : self.a.sub(rhs.a) } }}
impl<T:Scalar> SubAssign<Self> for Colored<T> { fn sub_assign(&mut self, rhs: Self) { *self = self.sub(rhs); }}

impl<T:Scalar> Mul<Self> for Colored<T> { type Output=Self; fn mul(self, rhs: Colored<T>) -> Self::Output { Self { r : self.r.mul(rhs.r), g : self.g.mul(rhs.g), b : self.b.mul(rhs.b), a : self.a.mul(rhs.a) } }}
impl<T:Scalar> MulAssign<Self> for Colored<T> { fn mul_assign(&mut self, rhs: Self) { *self = self.mul(rhs); }}

impl<T:Scalar> Div<Self> for Colored<T> { type Output=Self; fn div(self, rhs: Colored<T>) -> Self::Output { Self { r : self.r.div(rhs.r), g : self.g.div(rhs.g), b : self.b.div(rhs.b), a : self.a.div(rhs.a) } }}
impl<T:Scalar> DivAssign<Self> for Colored<T> { fn div_assign(&mut self, rhs: Self) { *self = self.div(rhs); }}

impl<T:Scalar> Mul<T> for Colored<T> { type Output=Self; fn mul(self, rhs: T) -> Self::Output { Self { r : self.r.mul(rhs), g : self.g.mul(rhs), b : self.b.mul(rhs), a : self.a.mul(rhs) } }}
impl<T:Scalar> MulAssign<T> for Colored<T> { fn mul_assign(&mut self, rhs: T) { *self = self.mul(rhs); }}

impl<T:Scalar> Div<T> for Colored<T> { type Output=Self; fn div(self, rhs: T) -> Self::Output { Self { r : self.r.div(rhs), g : self.g.div(rhs), b : self.b.div(rhs), a : self.a.div(rhs) } }}
impl<T:Scalar> DivAssign<T> for Colored<T> { fn div_assign(&mut self, rhs: T) { *self = self.div(rhs); }}




impl<T:Scalar> From<C4<T>> for Colored<T> { fn from(value: C4<T>) -> Self { Self::from_c4(value) }}
impl<T:Scalar> From<C3<T>> for Colored<T> { fn from(value: C3<T>) -> Self { Self::from_c3(value) }}
impl<T:Scalar> From<ColorHSL> for Colored<T> { fn from(val : ColorHSL) -> Colored<T> { Self::from_c4(val.to_rgb().to_vec4_coef().map(|e| T::from_coef(e))) }}

impl<T:Scalar> From<[T; 3]> for Colored<T> { fn from(val : [T; 3]) -> Self { Self::new_rgb(val[0], val[1], val[2])}}
impl<T:Scalar> Into<[T; 3]> for Colored<T> { fn into(self) -> [T; 3] { [self.r, self.g, self.b] }}

impl<T:Scalar> From<[T; 4]> for Colored<T> { fn from(val : [T; 4]) -> Self { Self::new_rgba(val[0], val[1], val[2], val[3])}}
impl<T:Scalar> Into<[T; 4]> for Colored<T> { fn into(self) -> [T; 4] { [self.r, self.g, self.b, self.a] }}

impl<T:Scalar> From<(T, T, T)> for Colored<T> { fn from(val : (T, T, T)) -> Self { Self::new_rgb(val.0, val.1, val.2)}}
impl<T:Scalar> Into<(T, T, T)> for Colored<T> { fn into(self) -> (T, T, T) { (self.r, self.g, self.b) }}

impl<T:Scalar> From<(T, T, T, T)> for Colored<T> { fn from(val : (T, T, T, T)) -> Self { Self::new_rgba(val.0, val.1, val.2, val.3)}}
impl<T:Scalar> Into<(T, T, T, T)> for Colored<T> { fn into(self) -> (T, T, T, T) { (self.r, self.g, self.b, self.a) }}

impl<T:Scalar> Colored<T>
{
    /// Create a `C4` composed with the `red`, `green`, `blue`, `alpha` components.
    pub fn to_c4(&self) -> C4<T> { C4::new(self.r, self.g, self.b, self.a) }
    /// Create a `C3` composed with the `red`, `green`, `blue` components.
    pub fn to_c3(&self) -> C3<T> { C3::new(self.r, self.g, self.b) }
    
    /// Create a color from a `C4` with the `red`, `green`, `blue`, `alpha` components.
    pub fn from_c4(c4: C4<T>) -> Self { Self::new_rgba(c4.x, c4.y, c4.z, c4.w) }
    /// Create a color from a `C3` with the `red`, `green`, `blue` components.
    pub fn from_c3(c3: C3<T>) -> Self { Self::new_rgb(c3.x, c3.y, c3.z) }


    /// Create a `Vec4` composed with the **normalized** (`[0..1]`) `red`, `green`, `blue`, `alpha` components.
    pub fn to_vec4_coef(&self) -> Vec4Coef { self.to_c4().map(|e| e.to_coef()) }
    /// Create a `vec3` composed with the **normalized** (`[0..1]`) `red`, `green`, `blue` components.
    pub fn to_vec3_coef(&self) -> Vec3Coef { self.to_c3().map(|e| e.to_coef()) }

    /// Create a color from a `C4` with the `red`, `green`, `blue`, `alpha` components.
    pub fn from_vec4_coef(c4: Vec4Coef) -> Self { Self::new_rgba(T::from_coef(c4.x), T::from_coef(c4.y), T::from_coef(c4.z), T::from_coef(c4.w)) }
    /// Create a color from a `C3` with the `red`, `green`, `blue` components.
    pub fn from_vec3_coef(c3: Vec3Coef) -> Self { Self::new_rgba(T::from_coef(c3.x), T::from_coef(c3.y), T::from_coef(c3.z), T::MAX_RANGE_VAL) }
}