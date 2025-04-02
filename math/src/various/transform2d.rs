use crate::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Transform2D
{
    pub pos   : Vec2,
    pub scale : Vec2,
    pub angle : Angle,
}
impl Default for Transform2D { fn default() -> Self { Self { pos: zero(), scale: one(), angle: zero() } }}