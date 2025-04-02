use std::ops::*;
use super::*;

/// Represent a length in pixel depending of a vector
#[derive(Clone, Copy, Default, Debug)]
pub struct UiNumberRelative
{
    /// Not included in the equality with the PartialEq trait
    pixel_pos_cached : Option<(real, Axis, Vec2)>,

    /// Coef based on the minimal axis
    min : Coef,
    /// Coef based on the maximal axis
    max : Coef,
    
    /// Coef based on the current axis
    axis : Coef,

    /// Coef based on the opposite axis
    opposite_axis : Coef,

    /// Coef based on the X axis
    x_axis : Coef,

    /// Coef based on the Y axis
    y_axis : Coef,
}
impl PartialEq for UiNumberRelative
{
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max && self.axis == other.axis && self.opposite_axis == other.opposite_axis && self.x_axis == other.x_axis && self.y_axis == other.y_axis
    }
}

impl UiNumberRelative
{
    /// Coef based on the minimal axis
    pub fn min(&self) -> Coef { self.min }
    /// Coef based on the maximal axis
    pub fn max(&self) -> Coef { self.max }
    /// Coef based on the current axis
    pub fn axis(&self) -> Coef { self.axis }
    /// Coef based on the opposite axis
    pub fn opposite_axis(&self) -> Coef { self.opposite_axis }
    /// Coef based on the X axis
    pub fn x_axis(&self) -> Coef { self.x_axis }
    /// Coef based on the Y axis
    pub fn y_axis(&self) -> Coef { self.y_axis }

    /// Coef based on the minimal axis
    pub fn set_min(&mut self, min : Coef) -> &mut Self { if min != self.min { self.min = min; self.invalidate_cache(); } self }
    /// Coef based on the maximal axis
    pub fn set_max(&mut self, max : Coef) -> &mut Self { if max != self.max { self.max = max; self.invalidate_cache(); } self }
    /// Coef based on the current axis
    pub fn set_axis(&mut self, axis : Coef) -> &mut Self { if axis != self.axis { self.axis = axis; self.invalidate_cache(); } self }
    /// Coef based on the opposite axis
    pub fn set_opposite_axis(&mut self, opposite_axis : Coef) -> &mut Self { if opposite_axis != self.opposite_axis { self.opposite_axis = opposite_axis; self.invalidate_cache(); } self }
    /// Coef based on the X axis
    pub fn set_x_axis(&mut self, x_axis : Coef) -> &mut Self { if x_axis != self.x_axis { self.x_axis = x_axis; self.invalidate_cache(); } self }
    /// Coef based on the Y axis
    pub fn set_y_axis(&mut self, y_axis : Coef) -> &mut Self { if y_axis != self.y_axis { self.y_axis = y_axis; self.invalidate_cache(); } self }


    pub fn with_min(mut self, min : Coef) -> Self { self.min = min; self }
    pub fn with_max(mut self, max : Coef) -> Self { self.max = max; self }
    pub fn with_x_axis(mut self, x_axis : Coef) -> Self { self.x_axis = x_axis; self }
    pub fn with_y_axis(mut self, y_axis : Coef) -> Self { self.y_axis = y_axis; self }
    pub fn with_axis(mut self, axis : Coef) -> Self { self.axis = axis; self }
    pub fn with_opposite_axis(mut self, opposite_axis : Coef) -> Self { self.opposite_axis = opposite_axis; self }

    pub fn to_px(&self, parent : Vec2, axis : Axis) -> real 
    {
        if let Some((px, axis_cached, parent_cached)) = self.pixel_pos_cached
        {
            if axis_cached == axis && parent_cached == parent { return px; }
        }

        let (min, max) = if parent.x < parent.y { (parent.x, parent.y) } else { (parent.y, parent.x) };
        
        self.min * min + self.max * max  + self.x_axis * parent.x + self.y_axis * parent.y +
            self.axis * match axis
            {
                Axis::X => parent.x,
                Axis::Y => parent.y,
            } +
            self.opposite_axis * match axis
            {
                Axis::X => parent.y,
                Axis::Y => parent.x,
            }
    }


    fn invalidate_cache(&mut self) { self.pixel_pos_cached = None; }
    pub fn update_cache(&mut self, parent : Vec2, axis : Axis)
    {
        self.pixel_pos_cached = Some((self.to_px(parent, axis), axis, parent));
    }
}

impl HaveZero for UiNumberRelative { const ZERO : Self = Self  { min : 0., max : 0., axis : 0., x_axis : 0., y_axis : 0., opposite_axis : 0., pixel_pos_cached : None }; }
impl HaveOne  for UiNumberRelative  { const ONE  : Self = Self  { min : 0., max : 0., axis : 1., x_axis : 0., y_axis : 0., opposite_axis : 0., pixel_pos_cached : None }; }
impl HaveHalf for UiNumberRelative  { const HALF  : Self = Self  { min : 0., max : 0., axis : 0.5, x_axis : 0., y_axis : 0., opposite_axis : 0., pixel_pos_cached : None }; }
impl Absolute for UiNumberRelative { fn absolute(self) -> Self { Self { min: self.min.absolute(), max: self.max.absolute(), axis : self.axis.absolute(), opposite_axis : self.opposite_axis.absolute(), x_axis : self.x_axis.absolute(), y_axis : self.y_axis.absolute(), pixel_pos_cached : None } }}
impl UnitArithmetic<Self> for UiNumberRelative {}

impl Reversable for UiNumberRelative { fn rev(self) -> Self { -self }}

impl UiUnit
{
    pub fn new() -> Self { Self::default() }
}

macro_rules! ui_relative_bin_op_real {
    ($left : expr, $right : expr, $fn_name: ident) => {
        {
            let r = $right;
            UiNumberRelative 
            { 
                min  : $left.min .$fn_name(r),
                max  : $left.max .$fn_name(r),
                axis : $left.axis.$fn_name(r),
                opposite_axis : $left.opposite_axis.$fn_name(r),
                x_axis : $left.x_axis.$fn_name(r),
                y_axis : $left.y_axis.$fn_name(r),
                pixel_pos_cached : None,
            }
        }
    };
}
macro_rules! ui_relative_bin_op {
    ($left : expr, $right : expr, $fn_name: ident) => {
        UiNumberRelative 
        { 
            min  : $left.min .$fn_name($right.min ),
            max  : $left.max .$fn_name($right.max ),
            axis : $left.axis.$fn_name($right.axis),
            opposite_axis : $left.opposite_axis.$fn_name($right.opposite_axis),
            x_axis : $left.x_axis.$fn_name($right.x_axis),
            y_axis : $left.y_axis.$fn_name($right.y_axis),
            pixel_pos_cached : None,
        }
    };
}
impl Neg for UiNumberRelative { type Output=Self; fn neg(self) -> Self::Output { Self::ZERO - self }}
        
impl Add<Self> for UiNumberRelative { type Output = Self; fn add(self, rhs: Self) -> Self::Output { ui_relative_bin_op!(self, rhs, add) }}
impl Sub<Self> for UiNumberRelative { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { ui_relative_bin_op!(self, rhs, sub) }}
impl Mul<Self> for UiNumberRelative { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { ui_relative_bin_op!(self, rhs, mul) }}
impl Div<Self> for UiNumberRelative { type Output = Self; fn div(self, rhs: Self) -> Self::Output { ui_relative_bin_op!(self, rhs, div) }}
impl Rem<Self> for UiNumberRelative { type Output = Self; fn rem(self, rhs: Self) -> Self::Output { ui_relative_bin_op!(self, rhs, rem) }}
impl AddAssign<Self> for UiNumberRelative { fn add_assign(&mut self, rhs: Self) { *self = ui_relative_bin_op!(*self, rhs, add) }}
impl SubAssign<Self> for UiNumberRelative { fn sub_assign(&mut self, rhs: Self) { *self = ui_relative_bin_op!(*self, rhs, sub) }}
impl MulAssign<Self> for UiNumberRelative { fn mul_assign(&mut self, rhs: Self) { *self = ui_relative_bin_op!(*self, rhs, mul) }}
impl DivAssign<Self> for UiNumberRelative { fn div_assign(&mut self, rhs: Self) { *self = ui_relative_bin_op!(*self, rhs, div) }}
impl RemAssign<Self> for UiNumberRelative { fn rem_assign(&mut self, rhs: Self) { *self = ui_relative_bin_op!(*self, rhs, rem) }}

impl Add<real> for UiNumberRelative { type Output = Self; fn add(self, rhs: real) -> Self::Output { ui_relative_bin_op_real!(self, rhs, add) }}
impl Sub<real> for UiNumberRelative { type Output = Self; fn sub(self, rhs: real) -> Self::Output { ui_relative_bin_op_real!(self, rhs, sub) }}
impl Mul<real> for UiNumberRelative { type Output = Self; fn mul(self, rhs: real) -> Self::Output { ui_relative_bin_op_real!(self, rhs, mul) }}
impl Div<real> for UiNumberRelative { type Output = Self; fn div(self, rhs: real) -> Self::Output { ui_relative_bin_op_real!(self, rhs, div) }}
impl Rem<real> for UiNumberRelative { type Output = Self; fn rem(self, rhs: real) -> Self::Output { ui_relative_bin_op_real!(self, rhs, rem) }}
impl AddAssign<real> for UiNumberRelative { fn add_assign(&mut self, rhs: real) { *self = ui_relative_bin_op_real!(*self, rhs, add) }}
impl SubAssign<real> for UiNumberRelative { fn sub_assign(&mut self, rhs: real) { *self = ui_relative_bin_op_real!(*self, rhs, sub) }}
impl MulAssign<real> for UiNumberRelative { fn mul_assign(&mut self, rhs: real) { *self = ui_relative_bin_op_real!(*self, rhs, mul) }}
impl DivAssign<real> for UiNumberRelative { fn div_assign(&mut self, rhs: real) { *self = ui_relative_bin_op_real!(*self, rhs, div) }}
impl RemAssign<real> for UiNumberRelative { fn rem_assign(&mut self, rhs: real) { *self = ui_relative_bin_op_real!(*self, rhs, rem) }}



#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Axis
{
    X,
    Y,
}

/// Represent a length in pixel independently of the screen resolution
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct UiUnit
{
    pub parent : UiNumberRelative,
    pub window : UiNumberRelative,
}

impl UiUnit
{
    pub fn with_parent(mut self, parent : UiNumberRelative) -> Self { self.parent = parent; self }

    pub fn update_cache(&mut self, cam : &ContextCamera, axis : Axis)
    {
        self.parent.update_cache(cam.parent_size_px(), axis);
        self.window.update_cache(cam.window_size_px(), axis);
    }

    pub fn to_px_x(&self, cam : &ContextCamera) -> real { self.to_px(cam, Axis::X) }
    pub fn to_px_y(&self, cam : &ContextCamera) -> real { self.to_px(cam, Axis::Y) }
    pub fn to_px(&self, cam : &ContextCamera, axis : Axis) -> real 
    {
        self.parent.to_px(cam.parent_size_px(), axis) + self.window.to_px(cam.window_size_px(), axis)
    }
}

macro_rules! ui_bin_op_real {
    ($left : expr, $right : expr, $fn_name: ident) => {
        {
            let r = $right;
            UiUnit
            { 
                parent : $left.parent.$fn_name(r),
                window : $left.window.$fn_name(r),
            }
        }
    };
}
macro_rules! ui_bin_op {
    ($left : expr, $right : expr, $fn_name: ident) => {
        UiUnit
        { 
            parent : $left.parent.$fn_name($right.parent),
            window : $left.window.$fn_name($right.window),
        }
    };
}
impl Neg for UiUnit { type Output=Self; fn neg(self) -> Self::Output { Self::ZERO - self }}
        
impl Add<Self> for UiUnit { type Output = Self; fn add(self, rhs: Self) -> Self::Output { ui_bin_op!(self, rhs, add) }}
impl Sub<Self> for UiUnit { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { ui_bin_op!(self, rhs, sub) }}
impl Mul<Self> for UiUnit { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { ui_bin_op!(self, rhs, mul) }}
impl Div<Self> for UiUnit { type Output = Self; fn div(self, rhs: Self) -> Self::Output { ui_bin_op!(self, rhs, div) }}
impl Rem<Self> for UiUnit { type Output = Self; fn rem(self, rhs: Self) -> Self::Output { ui_bin_op!(self, rhs, rem) }}
impl AddAssign<Self> for UiUnit { fn add_assign(&mut self, rhs: Self) { *self = ui_bin_op!(*self, rhs, add) }}
impl SubAssign<Self> for UiUnit { fn sub_assign(&mut self, rhs: Self) { *self = ui_bin_op!(*self, rhs, sub) }}
impl MulAssign<Self> for UiUnit { fn mul_assign(&mut self, rhs: Self) { *self = ui_bin_op!(*self, rhs, mul) }}
impl DivAssign<Self> for UiUnit { fn div_assign(&mut self, rhs: Self) { *self = ui_bin_op!(*self, rhs, div) }}
impl RemAssign<Self> for UiUnit { fn rem_assign(&mut self, rhs: Self) { *self = ui_bin_op!(*self, rhs, rem) }}

impl Add<real> for UiUnit { type Output = Self; fn add(self, rhs: real) -> Self::Output { ui_bin_op_real!(self, rhs, add) }}
impl Sub<real> for UiUnit { type Output = Self; fn sub(self, rhs: real) -> Self::Output { ui_bin_op_real!(self, rhs, sub) }}
impl Mul<real> for UiUnit { type Output = Self; fn mul(self, rhs: real) -> Self::Output { ui_bin_op_real!(self, rhs, mul) }}
impl Div<real> for UiUnit { type Output = Self; fn div(self, rhs: real) -> Self::Output { ui_bin_op_real!(self, rhs, div) }}
impl Rem<real> for UiUnit { type Output = Self; fn rem(self, rhs: real) -> Self::Output { ui_bin_op_real!(self, rhs, rem) }}
impl AddAssign<real> for UiUnit { fn add_assign(&mut self, rhs: real) { *self = ui_bin_op_real!(*self, rhs, add) }}
impl SubAssign<real> for UiUnit { fn sub_assign(&mut self, rhs: real) { *self = ui_bin_op_real!(*self, rhs, sub) }}
impl MulAssign<real> for UiUnit { fn mul_assign(&mut self, rhs: real) { *self = ui_bin_op_real!(*self, rhs, mul) }}
impl DivAssign<real> for UiUnit { fn div_assign(&mut self, rhs: real) { *self = ui_bin_op_real!(*self, rhs, div) }}
impl RemAssign<real> for UiUnit { fn rem_assign(&mut self, rhs: real) { *self = ui_bin_op_real!(*self, rhs, rem) }}


impl HaveZero for UiUnit { const ZERO : Self = Self { parent : UiNumberRelative::ZERO, window : UiNumberRelative::ZERO }; }
impl HaveOne  for UiUnit  { const ONE  : Self = Self { parent : UiNumberRelative::ONE,  window : UiNumberRelative::ZERO }; }
impl HaveHalf for UiUnit  { const HALF  : Self = Self { parent : UiNumberRelative::HALF,  window : UiNumberRelative::HALF }; }
impl Absolute for UiUnit { fn absolute(self) -> Self { Self { parent: self.parent.absolute(), window: self.window.absolute() } }}
impl UnitArithmetic<Self> for UiUnit {}

impl Reversable for UiUnit { fn rev(self) -> Self { -self }}

pub trait ToUiNumber
{
    /// parent max axis
    fn ui_max (self) -> UiUnit;
    /// parent min axis
    fn ui_min (self) -> UiUnit;
    /// parent current axis
    fn ui_axis(self) -> UiUnit;
    /// parent opposite axis
    fn ui_opposite_axis(self) -> UiUnit;
    /// parent X axis
    fn ui_x_axis(self) -> UiUnit;
    /// parent Y axis
    fn ui_y_axis(self) -> UiUnit;

    /// window max axis
    fn ui_window_max (self) -> UiUnit;
    /// window min axis
    fn ui_window_min (self) -> UiUnit;
    /// window current axis
    fn ui_window_axis(self) -> UiUnit;
    /// window opposite axis
    fn ui_window_opposite_axis(self) -> UiUnit;
    /// window X axis
    fn ui_window_x_axis(self) -> UiUnit;
    /// window Y axis
    fn ui_window_y_axis(self) -> UiUnit;
}

impl<C : ToReal> ToUiNumber for C
{
    /// Coef for the maximal axis
    fn ui_max(self) -> UiUnit { UiUnit { parent : UiNumberRelative { max: self.to_real(), ..___() }, ..___() } }
    
    /// Coef for the minimal axis
    fn ui_min(self) -> UiUnit { UiUnit { parent : UiNumberRelative { min: self.to_real(), ..___() }, ..___() } }

    /// Coef for the current axis
    fn ui_axis(self) -> UiUnit { UiUnit { parent : UiNumberRelative { axis: self.to_real(), ..___() }, ..___() } }

    /// Coef for the opposite axis
    fn ui_opposite_axis(self) -> UiUnit { UiUnit { parent : UiNumberRelative { opposite_axis : self.to_real(), ..___() }, ..___() } }

    /// Coef for the x axis
    fn ui_x_axis(self) -> UiUnit { UiUnit { parent : UiNumberRelative { x_axis: self.to_real(), ..___() }, ..___() } }

    /// Coef for the y axis
    fn ui_y_axis(self) -> UiUnit { UiUnit { parent : UiNumberRelative { y_axis: self.to_real(), ..___() }, ..___() } }


    /// Coef for the window maximal axis
    fn ui_window_max(self) -> UiUnit { UiUnit { window : UiNumberRelative { max: self.to_real(), ..___() }, ..___() } }

    /// Coef for the window minimal axis
    fn ui_window_min(self) -> UiUnit { UiUnit { window : UiNumberRelative { min: self.to_real(), ..___() }, ..___() } }

    /// Coef for the window current axis
    fn ui_window_axis(self) -> UiUnit { UiUnit { window : UiNumberRelative { axis: self.to_real(), ..___() }, ..___() } }

    /// Coef for the window opposite axis
    fn ui_window_opposite_axis(self) -> UiUnit { UiUnit { window : UiNumberRelative { opposite_axis : self.to_real(), ..___() }, ..___() } }

    /// Coef for the window x axis
    fn ui_window_x_axis(self) -> UiUnit { UiUnit { window : UiNumberRelative { x_axis: self.to_real(), ..___() }, ..___() } }

    /// Coef for the window y axis
    fn ui_window_y_axis(self) -> UiUnit { UiUnit { window : UiNumberRelative { y_axis: self.to_real(), ..___() }, ..___() } }
}

