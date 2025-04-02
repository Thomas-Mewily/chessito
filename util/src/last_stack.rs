use std::ops::{Deref, DerefMut};

use super::*;

/// A stack that ALWAY have one element.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct LastStack<T>
{
    /// will be readed frequently
    last : T,
    stack : Vec<T>
}
impl<T> Deref for LastStack<T>
{
    type Target=T;
    fn deref(&self) -> &Self::Target { &self.last }
}
impl<T> DerefMut for LastStack<T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.last }}

impl<T> LastStack<T>
{
    pub fn new(value : T) -> Self { Self { last: value, stack: ___() }}

    pub fn last(&self) -> &T { &self.last }
    pub fn last_mut(&mut self) -> &mut T { &mut self.last }

    pub fn clear(&mut self) { self.stack.clear() }

    pub fn set(&mut self, value : T) { self.last = value; } 
    pub fn push(&mut self) where T : Clone { self.stack.push(self.last.clone()) } 
    pub fn pop(&mut self) { self.last = self.stack.pop().unwrap(); } 

    pub fn len(&self) -> usize  { self.stack.len() + 1 } 

    pub fn clone_last(&self) -> T where T : Clone { self.last.clone() }

    pub fn remove_all(&mut self, val : &T) where T : PartialEq
    {
        self.stack.retain(|e| e != val);
        if &self.last == val
        {
            self.pop();
        }
    }
}