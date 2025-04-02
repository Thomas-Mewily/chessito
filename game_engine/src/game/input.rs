use crate::*;

impl<G:IGame<Input=(),InputProvider=()>> IInputProvider<G> for () 
{ 
    fn get_input(&mut self, _game : &mut G, _time : GameTime, _c : &mut DefaultContext<G::Global>) -> () 
    { ___() }
}

pub trait IInputProvider<G : IGame<InputProvider = Self>> : Default + Sized + Clone
{ 
    #[allow(unused_variables)]
    fn new(ctx : &mut DefaultContext<G::Global>) -> Self { Default::default() }
    fn get_input(&mut self, game : &mut G, time : GameTime, ctx : &mut DefaultContext<G::Global>) -> G::Input;
}


/// Input that will affect the gameplay
pub trait IInput : Clone + PartialEq + Sized + std::fmt::Debug
{
    fn have_side_effect(&self) -> bool { true }
    fn combine(&mut self, other : Self) { *self = other; }
}

impl IInput for () {}