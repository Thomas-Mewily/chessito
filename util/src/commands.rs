use std::{marker::PhantomData, ops::Deref};
use super::*;

pub trait ICommand<Data>
{
    fn execute(&mut self, data : &mut Data);
    fn undo(&mut self, data : &mut Data);
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandStack<T : ICommand<Data>, Data>
{
    pub time_line : Vec<T>,
    // point to the next not executed timeline
    pub idx : usize,
    _phantom : PhantomData<Data>,
}
impl<T : ICommand<Data>, Data> Default for CommandStack<T, Data> { fn default() -> Self { Self { time_line: vec![], idx : 0, _phantom: PhantomData } }}

impl<T : ICommand<Data>, Data> CommandStack<T, Data>
{
    pub fn new() -> Self { ___() }

    pub fn current(&self) -> Option<&T> { if self.idx > 0 { Some(&self.time_line[self.idx-1]) } else { None }}

    // push at the cursor and override the rest of the timeline
    /*
    pub fn push_override(&mut self, val : T)
    {
        self.time_line.drain(self.idx..);
        self.push_end(val);
    } */
    pub fn push(&mut self, val : T)
    {
        self.time_line.drain(self.idx..);
        self.time_line.push(val)
    }

    pub fn execute_all(&mut self, data : &mut Data) { while self.cursor_execute(data) {} }

    pub fn cursor_can_execute(&mut self) -> bool { self.idx < self.time_line.len() }
    pub fn cursor_execute(&mut self, data : &mut Data) -> bool
    {
        if !self.cursor_can_execute() { return false; }
        self.time_line[self.idx].execute(data);
        self.idx+=1;
        true
    }

    pub fn cursor_can_undo(&mut self) -> bool { self.idx > 0 }
    pub fn cursor_undo(&mut self, data : &mut Data) -> bool
    {
        if !self.cursor_can_undo() { return false; }
        self.time_line[self.idx-1].undo(data);
        self.idx -= 1;
        true
    }
}


#[derive(Default, Debug, Clone, PartialEq)]
pub struct CommandGameBased<T : ICommand<Data>, Data>
{
    pub data : Data,
    pub commands : CommandStack<T, Data>,
}
impl<T : ICommand<Data>, Data> Deref for CommandGameBased<T, Data> { type Target=Data; fn deref(&self) -> &Self::Target { &self.data }}

impl<T : ICommand<Data>, Data> From<Data> for CommandGameBased<T, Data> { fn from(value: Data) -> Self { Self::new(value) }}
impl<T : ICommand<Data>, Data> CommandGameBased<T, Data> { pub fn new(data : Data) -> Self { Self { data: data, commands: ___() } }}
