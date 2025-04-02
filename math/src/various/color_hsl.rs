use std::ops::*;

use crate::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorHSL
{
    /// Color coefficient. Ex:  `0` = red, `0.25` = green, `0.5` = blue, `0.75` = magenta
    pub h : Coef,
    /// Greyscale : `0`, `1`: pure color. 
    pub s : Coef,
    /// Black & White level. `0` = black, `0.5` = pure color, `1` = white
    pub l : Coef,
    pub a : Coef,
}

impl ColorHSL
{
    #[inline] pub fn new<C : Scalar>(h : C, s : C, l : C, a : C) -> Self 
    { Self { h : h.to_coef(), s : s.to_coef(), l : l.to_coef(), a : a.to_coef(), }}

    pub const fn new_grayscale(coef : Coef) -> Self { Self::new_hsl(0., 0., coef) }

    /// H : Color coefficient. Ex:  `0` = red, `0.25` = green, `0.5` = blue, `0.75` = magenta
    /// 
    /// S : Greyscale : `0`, `1`: pure color. 
    /// 
    /// L : Black & White level. `0` = black, `0.5` = pure color, `1` = white
    pub const fn new_hsla(h : Coef, s : Coef, l : Coef, a : Coef) -> Self { Self { h, s, l, a } }

    /// H : Color coefficient. Ex:  `0` = red, `0.25` = green, `0.5` = blue, `0.75` = magenta
    /// 
    /// S : Greyscale : `0`, `1`: pure color. 
    /// 
    /// L : Black & White level. `0` = black, `0.5` = pure color, `1` = white
    pub const fn new_hsl(h : Coef, s : Coef, l : Coef) -> Self { Self::new_hsla(h, s, l, 1.) }

    pub fn to_rgb(self) -> Color 
    {
        // Thank to MacroQuad, the following code was copied and edited from the MacroQuad crate
        let r;
        let g;
        let b;
    
        if self.s == 0.0 {  r = self.l; g = self.l; b = self.l; }
        else {
            fn hue_to_rgb(p: Coef, q: Coef, mut t: Coef) -> Coef {
                if t < 0.0 { t += 1.0 }
                if t > 1.0 { t -= 1.0 }
                if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
                if t < 1.0 / 2.0 { return q; }
                if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
                p
            }
    
            let q = if self.l < 0.5 {
                self.l * (1.0 + self.s)
            } else {
                self.l + self.s - self.l * self.s
            };
            let p = 2.0 * self.l - q;
            r = hue_to_rgb(p, q, self.h + 1.0 / 3.0);
            g = hue_to_rgb(p, q, self.h);
            b = hue_to_rgb(p, q, self.h - 1.0 / 3.0);
        }
    
        Color::new(r, g, b, self.a)
    }

    pub fn add_h(mut self, dh : Coef) -> Self { self.h += dh; self }
    pub fn add_s(mut self, ds : Coef) -> Self { self.s += ds; self }
    pub fn add_l(mut self, dl : Coef) -> Self { self.l += dl; self }
    pub fn add_a(mut self, da : Coef) -> Self { self.a += da; self }

    pub fn with_h(mut self, h : Coef) -> Self { self.h = h; self }
    pub fn with_s(mut self, s : Coef) -> Self { self.s = s; self }
    pub fn with_l(mut self, l : Coef) -> Self { self.l = l; self }
    pub fn with_a(mut self, a : Coef) -> Self { self.a = a; self }
 
    pub const fn unpack_hsl(self) -> (Coef, Coef, Coef) { (self.h, self.s, self.l) }
    pub const fn unpack_hsla(self) -> (Coef, Coef, Coef, Coef) { (self.h, self.s, self.l, self.a) }

}

impl ColorHSL
{
    /// Create a vec4 composed with the `hue`, `saturation`, `lightness`, `alpha` components.
    pub fn to_vec4(&self) -> Vec4 { Vec4::new(self.h, self.s, self.l, self.a) }
    /// Create a vec3 composed with the `hue`, `saturation`, `lightness` components.
    pub fn to_vec3(&self) -> Vec3 { Vec3::new(self.h, self.s, self.l) }

    /// Create a color from a vec4 with the `hue`, `saturation`, `lightness`, `alpha` components.
    pub fn from_vec4(vec: Vec4) -> Self { Self::new_hsla(vec.x, vec.y, vec.z, vec.w) }
    /// Create a color from a vec3 with the `hue`, `saturation`, `lightness`, components.
    pub fn from_vec3(vec: Vec3) -> Self { Self::new_hsl(vec.x, vec.y, vec.z) }
}

impl<I : Integer> From<[I; 3]> for ColorHSL { fn from(val : [I; 3]) -> Self { Self::new_hsl(val[0].to_coef(), val[1].to_coef(), val[2].to_coef())}}
impl<I : Integer> Into<[I; 3]> for ColorHSL { fn into(self) -> [I; 3] { [ I::from_coef(self.h), I::from_coef(self.s), I::from_coef(self.l) ]}}

impl<I : Integer> From<[I; 4]> for ColorHSL { fn from(val : [I; 4]) -> Self { Self::new_hsla(val[0].to_coef(), val[1].to_coef(), val[2].to_coef(), val[3].to_coef())}}
impl<I : Integer> Into<[I; 4]> for ColorHSL { fn into(self) -> [I; 4] { [ I::from_coef(self.h), I::from_coef(self.s), I::from_coef(self.l), I::from_coef(self.a) ]}}

impl<I : Integer> From<(I, I, I)> for ColorHSL { fn from(val : (I, I, I)) -> Self { Self::new_hsl(val.0.to_coef(), val.1.to_coef(), val.2.to_coef())}}
impl<I : Integer> Into<(I, I, I)> for ColorHSL { fn into(self) -> (I, I, I) { (I::from_coef(self.h), I::from_coef(self.s), I::from_coef(self.l)) }}

impl<I : Integer> From<(I, I, I, I)> for ColorHSL { fn from(val : (I, I, I, I)) -> Self { Self::new_hsla(val.0.to_coef(), val.1.to_coef(), val.2.to_coef(), val.3.to_coef())}}
impl<I : Integer> Into<(I, I, I, I)> for ColorHSL { fn into(self) -> (I, I, I, I) { (I::from_coef(self.h), I::from_coef(self.s), I::from_coef(self.l), I::from_coef(self.a)) }}

impl Into<Vec4> for ColorHSL { fn into(self) -> Vec4 { self.to_vec4() }}
impl Into<Vec3> for ColorHSL { fn into(self) -> Vec3 { self.to_vec3() }}

impl From<Color> for ColorHSL { fn from(val : Color) -> ColorHSL { val.to_hsl() }}

impl From<Vec3> for ColorHSL { fn from(val : Vec3) -> ColorHSL { Self::from_vec3(val) }}
impl From<Vec4> for ColorHSL { fn from(val : Vec4) -> ColorHSL { Self::from_vec4(val) }}
