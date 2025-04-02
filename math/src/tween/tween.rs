use std::{fmt::{Debug, Formatter, Result}, ops::*};
use crate::*;

#[derive(Debug, Default, PartialEq)]
pub struct Tween<T : Mul<real,Output=T> + Add<T,Output=T> + Clone + Copy + PartialEq + Debug>
{
    src  : T,
    dest : T,

    begin    : Time,
    duration : Time,

    ease : Easing,
}

impl<T : Mul<real,Output=T> + Add<T,Output=T> + Clone + Copy + PartialEq + Debug> Copy for Tween<T>{}
impl<T : Mul<real,Output=T> + Add<T,Output=T> + Clone + Copy + PartialEq + Debug> Clone for Tween<T>{
    fn clone(&self) -> Self {
        Self { begin: self.begin.clone(), duration: self.duration.clone(), src: self.src.clone(), dest: self.dest.clone(), ease : Easing::default() }
    }
}


impl<T : Mul<real,Output=T> + Add<T,Output=T> + Clone + Copy + PartialEq + Debug> Tween<T>
{
    pub fn new(val : T) -> Self { Self::new_animated(val.clone(), val, zero(), zero(), Default::default())}

    pub fn new_animated(src : T, dest : T, begin : Time, duration : Time, ease : Easing) -> Self
    {
        Self { src, dest, begin, duration, ease }
    }

    pub fn begin(&self) -> Time { self.begin }
    pub fn set_begin(&mut self, begin : Time) -> &mut Self { self.begin = begin; self }
    pub fn with_begin(mut self, begin : Time) -> Self { (&mut self).set_begin(begin); self }

    pub fn duration(&self) -> Time { self.duration }
    pub fn set_duration(&mut self, duration : Time) -> &mut Self { self.duration = duration; self }
    pub fn with_duration(mut self, duration : Time) -> Self { (&mut self).set_duration(duration); self }

    pub fn end(&self) -> Time { self.begin() + self.duration() }
    pub fn set_end(&mut self, end : Time) -> &mut Self { self.duration = end - self.begin; self }
    pub fn with_end(mut self, end : Time) -> Self { (&mut self).set_end(end); self }


    pub fn ease(&self) -> Easing { self.ease }
    pub fn set_ease(&mut self, ease : Easing) -> &mut Self { self.ease = ease; self }
    pub fn with_ease(mut self, ease : Easing) -> Self { (&mut self).set_ease(ease); self }


    pub fn src(&self) -> T { self.src }
    pub fn src_mut(&mut self) -> &mut T { &mut self.src }
    pub fn set_src(&mut self, src : T) -> &mut Self { self.src = src; self }
    pub fn set_src_relative_to_dest(&mut self, delta_dest : T) -> &mut Self { self.set_src(self.dest() + delta_dest); self}
    pub fn with_src(mut self, src : T) -> Self { (&mut self).set_src(src); self }

    pub fn dest(&self) -> T { self.dest }
    pub fn dest_mut(&mut self) -> &mut T { &mut self.dest }
    pub fn set_dest(&mut self, dest : T) -> &mut Self { self.dest = dest; self }
    pub fn set_dest_relative_to_src(&mut self, delta_src : T) -> &mut Self { self.set_dest(self.src() + delta_src); self}
    pub fn with_dest(mut self, dest : T) -> Self { (&mut self).set_dest(dest); self }

    pub fn start(&mut self, time : Time) -> &mut Self { self.set_begin(time) }




    pub fn for_all_mut<F>(&mut self, f : F) where F : Fn(&mut T) { self.for_src_and_dest_mut(f) }
    pub fn for_src_and_dest_mut<F>(&mut self, f : F) where F : Fn(&mut T)
    { f(&mut self.src); f(&mut self.dest); }

    /// erase the src with the current
    pub fn animate_to(&mut self, time : Time, dest : T) -> &mut Self 
    {
        self.set_src(self.get(time));
        self.set_dest(dest);
        self.start(time);
        self
    }

    pub fn time_to_coef(&self, time : Time) -> Coef
    {
        if self.duration.is_zero() { 1. } else { (time - self.begin) / self.duration }
    }

    pub fn cur(&self, time : Time) -> T { self.get(time) }
    pub fn get(&self, time : Time) -> T { self.get_from_coef(self.time_to_coef(time)) }

    pub fn get_from_coef(&self, coef : Coef) -> T
    {
        if coef >= 1.
        { 
            return self.dest;
        }
        if coef <= 0.
        { 
            return self.src;
        }

        self.src.lerp_unrestricted(self.dest, self.ease.apply(coef))
    }
}
