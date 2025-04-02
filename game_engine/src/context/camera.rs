use std::fmt::Debug;

use super::*;
use macroquad::camera as mq;
use macroquad::camera::Camera2D as mq_cam;

pub struct Camera2
{
    pub rect_px : Rect2,

    /// the minimun legal view rectangle of the world
    pub world : Rect2,
    /// the current total view rectangle of the world, >= world because the container can have extra space
    /// pub total_world : Rect2,

    //pub pixel_perfect_scaling : bool,

    pub unused_coef_center  : Vec2Coef,

    pub(crate) cam     : mq_cam,
}
impl Default for Camera2
{
    fn default() -> Self {
        Self 
        { 
            unused_coef_center: Vec2::HALF,
            rect_px: ___(),
            //total_world: ___(),
            world: ___(), 
            cam: ___(),
        }
    }
}

impl Camera2
{
    /* 
    pub fn from_rect(rect : Rect2) -> Self
    {
        Self { container_px: rect, scale : one(), cam: ___() }
    }*/
}

impl Camera2
{

    /// Update the associatd lib camera
    pub(crate) fn update(&mut self, window : Vec2)
    {   
        let scale = Vec2::splat((self.rect_px.size / self.world.size).min_element()); // * (window / self.screen_px.size);
        
        // for pixel perfect mode :
        // scale = Vec2::splat(scale.x.round());

        let pixel_size = self.rect_px.size;

        let target_size = self.world.size * scale;
        let unused_pixel_size = pixel_size - target_size;
        
        let mut pixel_offset = -unused_pixel_size * self.unused_coef_center;
        pixel_offset += window * 0.5;

        self.cam = 
        mq_cam
        {
            rotation: 0.,
            zoom: (vec2(2., -2.) * scale / window).to_lib(),
            target: ((pixel_offset - self.rect_px.pos + self.world.pos*scale) / scale).to_lib(),
            offset: Vec2::ZERO.to_lib(),
            render_target: None,
            viewport: None,
        }
    }
}

impl Clone for Camera2
{
    fn clone(&self) -> Self {
        Self 
        { 
            rect_px: self.rect_px.clone(),

            world: self.world, 
            //total_world : self.total_world,

            unused_coef_center : self.unused_coef_center,
            
            cam: 
            mq_cam 
            {
                rotation: self.cam.rotation,
                zoom: self.cam.zoom,
                target: self.cam.target,
                offset: self.cam.offset,
                render_target: None,
                viewport: self.cam.viewport,
            },
        }
    }
}

pub struct ContextCamera
{
    /// Don't forget to call `update_and_apply()` after any edit
    current : LastStack<Camera2>,
    window  : Vec2,
}

impl Default for ContextCamera
{
    fn default() -> Self {
        let mut s = Self { current: LastStack::new(___()), window: Default::default() };
        s.reset();
        s
    }
}

impl ContextCamera
{
    pub(crate) fn reset(&mut self) 
    { 
        self.window = vec2(macroquad::prelude::screen_width() as real, macroquad::prelude::screen_height() as real);

        self.current.clear();

        let window_rect = Rect2::ZERO.with_size(self.window);

        self.current.set
        (
            Camera2 
            { 
                rect_px : window_rect, 
                world        : window_rect,
                .. ___()
            }
        );
        
        self.apply();
    }
}

impl<Glob> ContextEvent<Glob> for ContextCamera
{
    fn input_begin(ctx : &mut DefaultContext<Glob>) { ctx.pen.cam.reset(); }
    fn input_end  (ctx : &mut DefaultContext<Glob>) { assert!(ctx.pen.cam.current.len() == 1, "Forgot to pop a camera when getting the input"); }

    fn draw_begin(ctx : &mut DefaultContext<Glob>) { ctx.pen.cam.reset(); }
    fn draw_end  (ctx : &mut DefaultContext<Glob>) { assert!(ctx.pen.cam.current.len() == 1, "Forgot to pop a camera when drawing"); }
}

impl ContextCamera
{
    /// window size
    pub fn window_size_px(&self) -> Vec2 { self.window }
    /// aka parent size
    pub fn parent_size_px(&self) -> Vec2 { self.parent_rect_px().size }
    pub fn parent_pos_px(&self) -> Vec2 { self.parent_rect_px().pos }
    pub fn parent_rect_px(&self) -> Rect2 { self.current().rect_px }

    pub fn current(&self) -> &Camera2 { &self.current}
    pub(crate) fn current_mut(&mut self) -> &mut Camera2 { &mut self.current}

    pub fn push(&mut self) -> &mut Self { self.current.push(); self }
    pub fn pop (&mut self) -> &mut Self { self.current.pop(); self.apply_without_update(); self }

    pub(crate) fn apply_without_update(&mut self) { mq::set_camera(&self.current.cam); }

    pub fn update(&mut self) -> &mut Self { self.current.update(self.window); self  }
    pub fn apply(&mut self) -> &mut Self { self.update(); self.apply_without_update(); self }
}

impl ContextCamera
{
    pub(crate) fn mq_to_engine(&self, mut macro_quad_pos : Vec2) -> Vec2
    {
        macro_quad_pos.y = self.window_size_px().y - macro_quad_pos.y;
        macro_quad_pos
    }

    //pub(crate) fn px_to_world(&self, pos : Vec2) -> Vec2 { self.current.cam.screen_to_world(self.to_engine(pos).to_lib()).to_engine() }
    pub(crate) fn mq_to_world(&self, pos : Vec2) -> Vec2 
    { 
        self.px_to_world(self.mq_to_engine(pos))
    }

    pub(crate) fn px_to_world(&self, pos : Vec2) -> Vec2 
    { 
        self.current.cam.screen_to_world(pos.to_lib()).to_engine() 
    }

}


impl ContextCamera
{
    pub fn set_unused_coef_center(&mut self, unused_coef_center : impl Into<Vec2>) -> &mut Self
    {
        self.current.unused_coef_center = unused_coef_center.into();
        self
    }

    pub fn set_rect_px(&mut self, rect_px : Rect2) -> &mut Self
    {
        self.current.rect_px = rect_px;
        self.current.world   = rect_px;
        self
    }

    pub fn set_world(&mut self, world : Rect2) -> &mut Self
    {
        self.current.world = world;
        self
    }
}