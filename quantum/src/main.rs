#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]



/* 
use std::{default, fmt::{format, write, Debug, Display, Formatter, Result as DisplayResult}, mem, num::{NonZeroU8, Saturating, Wrapping}, ops::*, time::{Duration, Instant}};
use std::{marker::PhantomData, ops::{Index, IndexMut}};

use math::*;
use text_extension::{code_formatter::{CodeDebug, CodeFormatter, OpenCloseKind}, console_color::*};
use util::*;

type PresenceFlags = u8;
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Tile
{
    id_presence : PresenceFlags,
    turn_moved  : u8,
}
impl Display for Tile
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult { write!(f, "{:02b}", self.id_presence) }
}

pub type TeamsRep = u8;
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Teams
{
    Blue = 0,
    Red  = 1,
}
impl From<Teams> for TeamsRep { fn from(value: Teams) -> Self { value as TeamsRep }}


pub type AbilityRep = u8;
#[repr(u8)]
pub enum Ability
{
    Pawn  = 0,
    Tower = 1,
}
impl From<Ability> for AbilityRep { fn from(value: Ability) -> Self { value as AbilityRep }}


#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Piece
{
    teams   : TeamsRep,
    ability : AbilityRep,
}
impl Piece
{
    pub fn new() -> Self { ___() }
}
impl Display for Piece
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult 
    {
        if self.teams.flag_have(Teams::Blue) { write!(f, "blue ")?; }
        if self.teams.flag_have(Teams::Red) { write!(f, "red ")?; }

        if self.ability.flag_have(Ability::Pawn) { write!(f, "pawn ")?; }
        if self.ability.flag_have(Ability::Tower) { write!(f, "tower ")?; }
        Ok(())
    }
}


impl CodeDebug for QuantumTree
{
    fn fmt_code_value(&self, f : &mut CodeFormatter) -> DisplayResult {
        match self
        {
            QuantumTree::Terminal(t) => t.fmt_code_value(f),
            QuantumTree::Branch(left, right) => 
            { 
                f.push_ident_nl()?;
                left.fmt_code_value(f)?;
                right.fmt_code_value(f)?;
                f.pop_ident_nl()?;
                Ok(())
            },
        }
    }
}
impl Display for QuantumTree
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult { self.fmt_code_value(&mut CodeFormatter::new(f)) }
}

#[derive(Clone)]
pub struct QuantumState
{
    tree : QuantumTree,
    turn : usize,
}
impl QuantumState
{
    pub fn new() -> Self
    {
        Self { tree: QuantumTree::Terminal(FixedQuantumState::new()), turn: 0 }
    }

    pub fn simulate_one_turn(&mut self)
    {
        
    }
}
impl CodeDebug for QuantumState
{
    fn fmt_code_value(&self, f : &mut CodeFormatter) -> DisplayResult {
        f.display(&"turn ")?;
        f.write(&self.turn)?;
        f.write(&self.tree)?;
        Ok(())
    }
}
impl Display for QuantumState
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult { self.fmt_code_value(&mut CodeFormatter::new(f)) }
}


#[derive(Clone, PartialEq, Eq)]
pub struct FixedQuantumState
{
    grid   : Grid2<Tile>,
    id_len : usize,
    id     : [Piece; std::mem::size_of::<Tile>() * 8],
}
impl DerefMut for FixedQuantumState { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.grid }}
impl Deref for FixedQuantumState
{
    type Target=Grid2<Tile>;
    fn deref(&self) -> &Self::Target { &self.grid }
}

impl FixedQuantumState
{
    pub fn new() -> Self
    {
        let mut id = [Piece::___(); std::mem::size_of::<Tile>() * 8];
        let mut id_len = 0;

        let pawn = id_len; 
        id[pawn].teams.flag_add(Teams::Red);
        id[pawn].ability.flag_add(Ability::Pawn);
        id_len += 1;

        let tower = id_len; 
        id[tower].teams.flag_add(Teams::Blue);
        id[tower].ability.flag_add(Ability::Tower);
        id_len += 1;

        let mut grid : Grid2<Tile> = Grid2::new(4.splat2());
        grid[point2(2, 0)].id_presence.flag_add(tower as PresenceFlags);
        grid[point2(1, 2)].id_presence.flag_add(pawn as PresenceFlags);

        Self { grid, id, id_len }
    }
}
impl CodeDebug for FixedQuantumState
{
    fn fmt_code_value(&self, f : &mut CodeFormatter<'_,'_>) -> DisplayResult {
        f.writeln("===========")?;
        for y in (0..self.grid.size().y).rev()
        {
            for x in 0..self.grid.size().x
            {
                f.display(&self.get(point2(x, y)))?;
                f.display(&" ")?;
            }
            f.new_line()?;
        }
        f.writeln("===========")?;
        for i in 0..self.id_len
        {
            f.display(&format!("{:02b}", PresenceFlags::ONE << i as PresenceFlags))?;
            f.display(&" => ")?;
            f.display(&self.id[i])?;
            f.new_line()?;
        }
        Ok(())
    }
}
impl Display for FixedQuantumState
{
    fn fmt(&self, f : &mut Formatter<'_>) -> DisplayResult { self.fmt_code_value(&mut CodeFormatter::new(f)) }
}


fn main() 
{
    /* 
    let mut game = QuantumState::new();
    game.simulate_one_turn();
    println!("{}", game);
    */

}
*/

fn main() {}