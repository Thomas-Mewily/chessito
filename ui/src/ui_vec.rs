use std::{fmt::Debug, ops::*};
use crate::*;

/// UiVector: Each axis has multiples coefficients to adapt itself to the screen.
pub type UiVec = C2<UiUnit>;
pub const fn ui_vec2(x: UiUnit, y: UiUnit) -> UiVec { UiVec::new(x, y) }

pub trait UiVecExtension
{
    fn to_px(&self, cam : &ContextCamera) -> Vec2; 
    fn update_cache(&mut self, cam : &ContextCamera);
}
impl UiVecExtension for UiVec
{
    fn to_px(&self, cam : &ContextCamera) -> Vec2 { vec2(self.x.to_px_x(cam), self.y.to_px_y(cam)) }
    fn update_cache(&mut self, cam : &ContextCamera) { self.x.update_cache(cam, Axis::X); self.y.update_cache(cam, Axis::Y); }
}


pub type UiRect = Rectangle<UiVec>;
pub trait UiRectExtension
{
    fn to_px(&self, cam : &ContextCamera) -> Rect2; 
    fn update_cache(&mut self, cam : &ContextCamera);
}
impl UiRectExtension for UiRect
{
    fn to_px(&self, cam : &ContextCamera) -> Rect2 { Rect2::new(self.pos.to_px(cam), self.size.to_px(cam)) }
    fn update_cache(&mut self, cam : &ContextCamera) { self.pos.update_cache(cam); self.size.update_cache(cam); }
}

// PartialEq
#[derive(Clone, PartialEq, Copy, Debug, Default)]
pub struct UiPos
{
    pub pos : Tween<UiRect>,
}
impl Deref for UiPos
{
    type Target=Tween<UiRect>;
    fn deref(&self) -> &Self::Target { &self.pos }
}
impl DerefMut for UiPos
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.pos }
}


impl UiPos
{
    pub fn new(pos : Tween<UiRect>) -> Self 
    {
        Self { pos }
    }

    pub fn fullscreen() -> Self
    {
        Self::new(Tween::new(UiRect::ONE))
    }

    pub fn to_px(&self,  cam : &ContextCamera) -> Rect2 { self.dest().to_px(cam) }
    pub fn update_cache(&mut self,  cam : &ContextCamera) 
    { 
        self.src_mut().update_cache(cam);
        self.dest_mut().update_cache(cam);
    }
}
