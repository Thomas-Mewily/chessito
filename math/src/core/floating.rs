// File related to real
use crate::*;

#[allow(non_camel_case_types)]
pub type real = f32;
//pub type real = f64;

pub const REAL_PI        : real = real::PI;
pub const REAL_FRAC_PI_2 : real = real::PI / 2.;

pub trait FloatConstant { const PI : Self; }
macro_rules! impl_float_constant{
    ($primitive_name: ident) => 
    { impl FloatConstant for $primitive_name { const PI : Self =  std::$primitive_name::consts::PI; } };
}
map_on_non_zero_floating!(impl_float_constant);
impl FloatConstant for f0 { const PI : Self = Self{}; }


/// Same as real, but just to mark the different use case.
/// Generally, a coef is between `[0.0, 1.0]`
pub type Coef = real;


pub struct CoefIterator
{
    pub nb_step  : uint,
    pub i        : uint,
}
impl Iterator for CoefIterator
{
    type Item=Coef;

    fn next(&mut self) -> Option<Self::Item> 
    {
        let old_i = self.i;
        self.i += 1;
        if old_i >= self.nb_step { None } else { Some(old_i.to_real() / (self.nb_step - 1).to_real()) }
    }
}
impl CoefIterator {
    pub fn new(nb_step : uint) -> Self { Self { nb_step, i : 0 }}
}

pub trait CoefIter { fn iter_coef(self) -> CoefIterator; }
macro_rules! impl_coef_iter{
    ($primitive_name: ty) => 
    { impl CoefIter for $primitive_name { fn iter_coef(self) -> CoefIterator { CoefIterator::new(self.to_uint()) }} };
}
map_on_integer!(impl_coef_iter);