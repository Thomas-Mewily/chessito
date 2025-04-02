use std::{default, fmt::{Debug, Display, Formatter, Result}, ops::*};
use crate::*;


#[derive(Default, Debug)]
pub struct ContextTime
{
    pub tick_time : TimeWithDelta,
    pub nb_tick   : Tick,
}
impl Deref for ContextTime { type Target=TimeWithDelta; fn deref(&self) -> &Self::Target { &self.tick_time }}
impl DerefMut for ContextTime { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.tick_time }}

impl<Glob> ContextEvent<Glob> for ContextTime
{
    fn tick_begin  (ctx : &mut DefaultContext<Glob>)
    {
        let delta = Time::from_s(macroquad::prelude::get_frame_time() as real);
        ctx.extern_time.delta  = delta;
        ctx.extern_time.total += delta;

        ctx.extern_time.nb_tick += one();
    }
}