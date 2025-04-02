use crate::*;


pub trait LibToEngine { type Associate; fn to_engine(self) -> Self::Associate; }
pub trait EngineToLib { type Associate; fn to_lib(self) -> Self::Associate; }


impl LibToEngine for macroquad::prelude::Vec2 { type Associate = Vec2; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(self.x as real, self.y as real) }  }
impl EngineToLib for Vec2 { type Associate = macroquad::prelude::Vec2; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.x as f32, self.y as f32) }  }


impl LibToEngine for macroquad::prelude::Vec3 { type Associate = Vec3; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(self.x as real, self.y as real, self.z as real) }  }
impl EngineToLib for Vec3 { type Associate = macroquad::prelude::Vec3; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.x as f32, self.y as f32, self.z as f32) }  }


impl LibToEngine for macroquad::prelude::Vec4 { type Associate = Vec4; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(self.x as real, self.y as real, self.z as real, self.w as real) } }
impl EngineToLib for Vec4 { type Associate = macroquad::prelude::Vec4; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.x as f32, self.y as f32, self.z as f32, self.w as f32) } }

impl LibToEngine for macroquad::prelude::Rect { type Associate = Rect2; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(vec2(self.x as real, self.y as real), vec2(self.w as real, self.h as real).into()) } }
impl EngineToLib for Rect2 { type Associate = macroquad::prelude::Rect; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.pos.x as f32 , self.pos.y as f32, self.size.x as f32, self.size.y as f32) } }

impl LibToEngine for macroquad::prelude::Color { type Associate = Color; #[inline] fn to_engine(self) -> Self::Associate { Self::Associate::new(self.r as real, self.g as real, self.b as real, self.a as real) } }
impl EngineToLib for Color { type Associate = macroquad::prelude::Color; #[inline] fn to_lib(self) -> Self::Associate { Self::Associate::new(self.r as f32, self.g as f32, self.b as f32, self.a as f32) } }


