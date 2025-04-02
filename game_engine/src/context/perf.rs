use std::{borrow::Borrow, default, fmt::Debug, ops::{Deref, DerefMut}};
use macroquad::{math::Vec2, prelude, texture::Texture2D, window::screen_width};
use crate::*;

#[derive(Default)]
pub struct ContextPerformance
{
    last_whole_tps : usize,
    tps_count   : usize,
    tps_start_s : usize,

    last_whole_fps : usize,
    fps_count : usize,

    last_whole_ups : usize,
    ups_count : usize,
}

impl ContextPerformance
{
    pub fn tps(&self) -> usize { self.last_whole_tps } 
    pub fn ups(&self) -> usize { self.last_whole_ups } 
    pub fn fps(&self) -> usize { self.last_whole_fps } 
}

impl Debug for ContextPerformance { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{} tps {} ups {} fps", self.tps(), self.ups(), self.tps()) }}

impl<Glob> ContextEvent<Glob> for ContextPerformance
{
    fn tick_begin(c : &mut DefaultContext<Glob>)
    {
        let perf = &mut c.perf;
        perf.tps_count += 1;

        if c.extern_time.total.whole_s() != perf.tps_start_s
        {
            perf.last_whole_tps = perf.tps_count;
            perf.tps_count = 0;
            perf.tps_start_s += 1;

            perf.last_whole_fps = perf.fps_count;
            perf.fps_count = 0;

            perf.last_whole_ups = perf.ups_count;
            perf.ups_count = 0;
        }
    }

    fn update_begin(ctx : &mut DefaultContext<Glob>) { ctx.perf.ups_count += 1; }
    fn draw_begin  (ctx : &mut DefaultContext<Glob>) { ctx.perf.fps_count += 1; }
}

