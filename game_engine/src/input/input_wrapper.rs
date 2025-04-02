use std::{borrow::Borrow, collections::HashMap, default, fmt::{format, Debug}, ops::{Deref, DerefMut, Sub}};
use macroquad::{prelude, prelude::Touch as MTouch};
use crate::*;

pub type KeyCode = prelude::KeyCode;

#[derive(Default, Debug)]
pub struct ContextKeyboard
{
    pressed : HashMap<KeyCode, InputBool>,
}

impl ContextKeyboard
{
    pub fn key(&self, key : KeyCode) -> InputBool 
    {
        match self.pressed.get(&key)
        {
            Some(b) => *b,
            None => ___(),
        }
    }

    pub(crate) fn tick_begin(&mut self)
    {
        for k in macroquad::input::get_keys_down()
        {
            if !self.pressed.contains_key(&k) { self.pressed.insert(k, ___()); }
            self.pressed.get_mut(&k).unwrap().intercept_press(true);
        }
    }

    pub(crate) fn update(&mut self)
    {
        let mq_pressed = macroquad::prelude::get_keys_down();
        for pressed in mq_pressed.iter().copied()
        {
            match self.pressed.get_mut(&pressed)
            {
                Some(v) => v.update(true, ()),
                None => { self.pressed.insert(pressed, ___()); },
            }
        }
        
        for (k, a) in self.pressed.iter_mut() 
        { if !mq_pressed.contains(k) { a.update(false, ()) }}
    }
}

//#[derive(Debug)]
pub struct ContextInput
{
    /// Also include the mouse when pressed
    touch : Vec<Touch>,

    mouse : Touch,

    keyboard : ContextKeyboard,

    //keyboard : Hash
}
impl Default for ContextInput
{
    fn default() -> Self 
    {
        Self { touch: vec![], mouse: Touch::default().with_kind(TouchKind::Mouse), keyboard : ___() }
    }
}


impl Debug for ContextInput
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "input {{ {} nb touch, {} just pressed, {} just released }}", self.touch.len(), self.touch_just_pressed().count(), self.touch_just_released().count())
        //f.debug_struct("ContextInput").field("touch", &self.touch).field("mouse", &self.mouse).finish()
    }
}


impl ContextInput
{
    pub fn key(&self, key : KeyCode) -> InputBool { self.keyboard.key(key) }
    pub fn key_just_pressed(&self) -> impl Iterator<Item=&KeyCode> { self.keyboard.pressed.iter().filter_map(|(k, a)| if a.just_pressed() { Some(k) } else { None }) }
    pub fn key_just_released(&self) -> impl Iterator<Item=&KeyCode> { self.keyboard.pressed.iter().filter_map(|(k, a)| if a.just_released() { Some(k) } else { None }) }
    pub fn key_pressed(&self) -> impl Iterator<Item=&KeyCode> { self.keyboard.pressed.iter().filter_map(|(k, a)| if a.is_pressed() { Some(k) } else { None }) }

    pub fn mouse(&self) -> &Touch { &self.mouse }

    pub fn touch(&self) -> impl Iterator<Item=&Touch> { self.touch.iter() }
    pub fn touch_just_pressed(&self) -> impl Iterator<Item=&Touch> { self.touch.iter().filter(|t| t.press().just_pressed()) }
    pub fn touch_just_released(&self) -> impl Iterator<Item=&Touch> { self.touch.iter().filter(|t| t.press().just_released()) }
    pub fn touch_pressed(&self) -> impl Iterator<Item=&Touch> { self.touch.iter().filter(|t| t.press().is_pressed()) }
    pub fn touch_released(&self) -> impl Iterator<Item=&Touch> { self.touch.iter().filter(|t| t.press().is_released()) }
    
    pub(crate) fn mq_mouse_pressed() -> bool { macroquad::prelude::is_mouse_button_down(macroquad::input::MouseButton::Left) }
    pub(crate) fn mq_mouse_pos(cam : &ContextCamera) -> Vec2 { let (mx, my) = macroquad::prelude::mouse_position(); cam.mq_to_engine(vec2(mx as real, my as real)) }

    pub(crate) const MAX_TOUCH_SCREEN_TOUCH : usize = 4;

    pub(crate) fn _tick_begin<Glob>(ctx : &mut DefaultContext<Glob>)
    {
        let (input, cam) = (&mut ctx.input, &ctx.pen.cam);

        input.mouse.press.intercept_press(Self::mq_mouse_pressed());

        input.keyboard.tick_begin();

        
        let mut touch = prelude::touches();
        touch.truncate(Self::MAX_TOUCH_SCREEN_TOUCH);
        
        for t in touch
        {
            match input.touch.iter().position(|p| *p.id == t.id)
            {
                Some(old_touch) => 
                {
                    let old = &mut input.touch[old_touch];
                    old.press.intercept_press(true);
                },
                None => 
                {
                    let pos = cam.mq_to_engine(t.position.to_engine());

                    input.touch.push(
                        Touch 
                        { 
                            id: TouchId(t.id),

                            position_px: pos.into(),
                            pressed_position_px: pos,
                            released_position_px: pos,

                            press: AlsoStoreOldBool::new(true, ()),
                            kind : TouchKind::Mobile,
                        }
                    )
                },
            }
        }
    }
    
    pub(crate) fn _input_begin<Glob>(ctx : &mut DefaultContext<Glob>)
    {
        let (input, cam) = (&mut ctx.input, &ctx.pen.cam);
        
        input.keyboard.update();

        input.touch.retain(|e| e.press().is_pressed() && e.kind != TouchKind::Mouse);

        let mut touch = prelude::touches();
        touch.truncate(Self::MAX_TOUCH_SCREEN_TOUCH);
        
        for t in touch
        {
            let pos = cam.mq_to_engine(t.position.to_engine());

            match input.touch.iter().position(|p| *p.id == t.id)
            {
                Some(old_touch) => 
                {
                    let old = &mut input.touch[old_touch];
                    old.update(pos, match t.phase
                        {
                            macroquad::input::TouchPhase::Started => true,
                            macroquad::input::TouchPhase::Stationary => true,
                            macroquad::input::TouchPhase::Moved => true,
                            macroquad::input::TouchPhase::Ended => false,
                            macroquad::input::TouchPhase::Cancelled => false,
                        });
                },
                _ => 
                {
                    // Should not happen because it is added first in _tick_begin, but I prefer to avoid panicking
                },
            }
        }

        input.mouse.update(Self::mq_mouse_pos(cam), Self::mq_mouse_pressed());
        input.touch.push(input.mouse);
    }

    /* 
    pub fn is_key_pressed(&self, key_code : KeyCode) -> bool { prelude::is_key_down(key_code) }
    pub fn is_key_release(&self, key_code : KeyCode) -> bool { !self.is_key_pressed(key_code) }

    pub fn is_key_just_pressed(&self, key_code : KeyCode) -> bool { prelude::is_key_pressed (key_code) }
    pub fn is_key_just_release(&self, key_code : KeyCode) -> bool { prelude::is_key_released(key_code) }
    */
}

impl<Glob> ContextEvent<Glob> for ContextInput
{
    fn tick_begin(ctx : &mut DefaultContext<Glob>)
    {
        ContextInput::_tick_begin(ctx);
    }

    fn input_begin (ctx : &mut DefaultContext<Glob>) 
    {
        ContextInput::_input_begin(ctx);
    }

    fn draw_end(ctx : &mut DefaultContext<Glob>)
    {
        let win_len = ctx.pen.cam.window_size_px().length();

        for t in &ctx.input.touch
        {
            if t.press().is_pressed()
            {
                let delta_len = t.delta_from_press_px().length();

                let coef = delta_len / win_len;

                let s_coef = 1.0.lerp(30. as real, coef.powf(1./1.25)) / 100.;
                let s = s_coef * ctx.pen.cam.window_size_px().min_element();
                //let s_origin = coef.lerp(4. as real, 0.25 as real) / 100. * ctx.pen.cam.window_size().min_element();
                let s_origin = 4.0.lerp(0.25 as real, coef.powf(1./2.)) / 100. * ctx.pen.cam.window_size_px().min_element();

                let greyscale_effect = (coef*5.).min(1.);
                ctx.pen.circle((t.position_px.x, t.position_px.y), s/2., Color::new_greyscale(1. - greyscale_effect));
                ctx.pen.circle((t.pressed_position_px.x, t.pressed_position_px.y), s_origin/2., Color::new_greyscale(greyscale_effect));
            }
        }
    }
}




#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct TouchId(u64);
impl Deref for TouchId { type Target=u64; fn deref(&self) -> &Self::Target { &self.0 } }
impl DerefMut for TouchId { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }
impl TouchId
{
    pub const MOUSE_ID : TouchId = TouchId(u64::MAX-17*9);
}
impl Debug for TouchId { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Touch#{}", self.0) }}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Touch
{
    id : TouchId,
    
    position_px : InputVec2,

    press : InputBool,

    // if the tap is so fast that no input() are call during it's lifestill, still register it
    //need_to_be_not_press_next_frame : ,
    
    /// The position when the touch was just pressed. By default the same as the first position
    pressed_position_px : Vec2,
    /// The position when the touch was just released. By default the same as the first position
    released_position_px : Vec2,

    kind  : TouchKind,
    
    //press_time   : TimeClock,
    //release_time : TimeClock,
}

impl Touch
{
    pub fn with_kind(mut self, kind  : TouchKind) -> Self { self.kind = kind; self }

    pub fn id(&self) -> TouchId { self.id }

    pub fn position_px(&self) -> InputVec2 { self.position_px }
    /// world position
    pub fn position(&self, cam : &ContextCamera) -> InputVec2 { InputVec2 { cur: cam.mq_to_world(self.position_px.cur), old: cam.mq_to_world(self.position_px.old), time:() } }

    pub fn pressed_position_px(&self) -> Vec2 { self.pressed_position_px }
    pub fn pressed_position(&self, cam : &ContextCamera) -> Vec2 { cam.mq_to_world(self.pressed_position_px) }

    pub fn released_position_px(&self) -> Vec2 { self.released_position_px }
    pub fn released_position(&self, cam : &ContextCamera) -> Vec2 { cam.mq_to_world(self.released_position_px) }

    pub fn delta_from_press_px(&self) -> Vec2 { self.position_px().cur - self.pressed_position_px }
    /// world position
    pub fn delta_from_press(&self, cam : &ContextCamera) -> Vec2 { self.position(cam).cur - self.pressed_position(cam) }

    pub fn press(&self) -> InputBool { self.press }
    pub fn kind(&self) -> TouchKind { self.kind }

    pub fn update(&mut self, position_px : Vec2, is_press : bool)
    {
        self.position_px.update(position_px, ());
        self.press.update(is_press, ());

        if self.press.just_pressed() { self.pressed_position_px = position_px; }
        if self.press.just_released() { self.released_position_px = position_px; }
    }

    //pub fn press_time(&self) -> TimeClock { self.press_time }
    //pub fn release_time(&self) -> TimeClock { self.release_time }
}

/* 
impl Debug for Touch
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        /* 
        write!(f, "{:?} pressed:({:?} at {:?}), released:({:?} at {:?}), pos:{}, pressed:{:?}, kind:{:?}", 
                self.id,
                self.press_position_px, self.press_time,
                self.release_position_px, self.release_time, 
                self.position_px, self.press, self.kind)
        */
        write!(f, "some touch !")
    }
}*/

impl Touch
{
    pub fn is_mouse     (&self) -> bool { matches!(self.kind, TouchKind::Mouse) }
    pub fn is_mobile    (&self) -> bool { matches!(self.kind, TouchKind::Mobile) }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum TouchKind
{
    Mobile,
    Mouse,
    #[default]
    Unknown,
}


#[derive(Clone, Copy, PartialEq)]
pub struct AlsoStoreOld<I : Copy + PartialEq, T : Copy + PartialEq=()>
{
    cur  : I,
    old  : I,
    time : T,
}
impl<I : Copy + PartialEq + Debug, T : Copy + PartialEq + Debug>
    Debug for AlsoStoreOld<I, T>  { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("AlsoStoreOld").field("val", &self.cur).field("old", &self.old).field("time", &self.time).finish() }}
impl<I : Copy + PartialEq + Default, T : Copy + PartialEq + Default>
    Default for AlsoStoreOld<I, T>  { fn default() -> Self { Self { cur: ___(), old: ___(), time : ___() } }}
impl<I : Copy + PartialEq, T : Copy + PartialEq>
    Deref for AlsoStoreOld<I, T> { type Target=I; fn deref(&self) -> &Self::Target { &self.cur }}


impl<I : Copy + PartialEq + Default, T : Copy + PartialEq + Default>
    From<I> for AlsoStoreOld<I, T> { fn from(value: I) -> Self { Self { cur: value, .. ___() } }}

impl<I : Copy + PartialEq + Default, T : Copy + PartialEq + Default> AlsoStoreOld<I, T> 
{
    pub fn any_change(&self) -> bool { !self.same() } 
    pub fn same(&self) -> bool { self.cur == self.old } 
}
impl<I : Copy + PartialEq + Sub<I>,  T : Copy + PartialEq> AlsoStoreOld<I, T> { pub fn delta(&self) -> <I as Sub<I>>::Output { self.cur - self.old }}

impl<T : Copy + PartialEq> AlsoStoreOld<bool, T> 
{
    /// `false` to `true`, `0` to `1`
    pub fn is_pull_up(&self) -> bool { self.cur && (!self.old) } 
    /// `true` to `false`, `1` to `0`
    pub fn is_pull_down(&self) -> bool { (!self.cur) && self.old }

    pub fn pull_changed(&self) -> bool { self.cur != self.old }


    
    pub fn is_pressed(&self) -> bool { self.cur } 
    pub fn was_pressed(&self) -> bool { self.old } 

    pub fn is_released(&self) -> bool { !self.is_pressed() } 
    pub fn was_released(&self) -> bool { !self.was_pressed() } 

    pub fn just_pressed(&self) -> bool { self.is_pull_up() } 
    pub fn just_released(&self) -> bool { self.is_pull_down() } 
}

impl<I : Copy + PartialEq + Default, T : Copy + PartialEq>  AlsoStoreOld<I, T> 
{
    pub fn new(cur : I, time : T) -> Self
    {
        Self::new_full(cur, ___(), time)
    }
}
impl<I : Copy + PartialEq, T : Copy + PartialEq>  AlsoStoreOld<I, T> 
{
    pub fn new_full(cur : I, old : I, time : T) -> Self
    {
        Self { cur, old, time }
    }

    pub fn update(&mut self, new : I, time : T) 
    {
        if self.old != self.cur
        {
            self.time = time;
            self.old = self.cur;
        }
        self.cur = new;
    }

    pub fn cur(&self) -> I { self.cur }
    pub fn old(&self) -> I { self.old }

    pub fn last_time_changed(&self) -> T { self.time }
}

pub type InputVec2<T=()> = AlsoStoreOld<Vec2,T>;
pub type InputBool<T=()> = AlsoStoreOldBool<T>;

/// Special because if a click occur so fast it can't be catch
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct AlsoStoreOldBool<T : Copy + PartialEq>
{
    val : AlsoStoreOld<bool, T>,
    // If true, override the current value to be true for the current input
    will_be_pressed : bool,
}
impl<T : Copy + PartialEq> Deref for AlsoStoreOldBool<T> { type Target=AlsoStoreOld<bool, T>; fn deref(&self) -> &Self::Target { &self.val }}
impl<T : Copy + PartialEq> DerefMut for AlsoStoreOldBool<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.val }}

impl<T : Copy + PartialEq> AlsoStoreOldBool<T>
{
    pub fn new(cur : bool, time : T) -> Self
    {
        Self::new_full(cur, ___(), time)
    }

    pub fn new_full(cur : bool, old : bool, time : T) -> Self
    {
        Self { val: AlsoStoreOld::new_full(cur, old, time), will_be_pressed: ___() }
    }

    pub fn update(&mut self, new : bool, time : T) 
    { 
        self.val.update(new | self.will_be_pressed, time);
        self.will_be_pressed = false;
    }

    pub fn intercept_press(&mut self, will_be_pressed : bool)
    {
        self.will_be_pressed |= will_be_pressed;
    }
}
//impl<T : Copy + PartialEq> From<bool> for AlsoStoreOldBool<T> { fn from(value: bool) -> Self { Self { val: value.into(), will_be_pressed: false } }}