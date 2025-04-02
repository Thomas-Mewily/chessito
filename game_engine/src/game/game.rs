use crate::*;

pub trait IGame : Sized + Clone 
{ 
    type Input : IInput;
    type InputProvider : IInputProvider<Self>;
    type Result;
    type Global;

    #[allow(async_fn_in_trait, unused_variables)]
    async fn init(&mut self, ctx : &mut DefaultContext<Self::Global>) {}

    // The input affect the game
    #[allow(async_fn_in_trait)]
    async fn update(&mut self, input : Self::Input, time : GameTime, ctx : &mut DefaultContext<Self::Global>) -> Option<Self::Result>;
    
    fn draw(&self, time : GameTime, ctx : &mut DefaultContext<Self::Global>);
}

#[derive(Clone, Copy, Default, Debug)]
pub struct TimeWithDelta
{
    /// current total time
    pub total  : Time,
    /// current delta time
    pub delta  : Time,
}
impl TimeWithDelta { pub fn old_t(self) -> Time { self.total - self.delta } }
impl Deref for TimeWithDelta { type Target=Time; fn deref(&self) -> &Self::Target { &self.total }}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct GameTime
{
    pub(crate) total : Time,
    pub(crate) delta : Time,
    /// Last time affect the gameplay
    pub(crate) last_input : Time,
}
impl GameTime
{
    pub fn elapsed_since_last_input(&self) -> Time { self.total - self.last_input } 

    pub fn total(&self) -> Time { self.total }
    pub fn delta(&self) -> Time { self.delta }
    pub fn last_input(&self) -> Time { self.last_input }
}
