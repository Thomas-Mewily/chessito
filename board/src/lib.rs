#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]

use std::{default, fmt::{format, write, Debug, Display, Formatter, Result as DisplayResult}, mem, num::{NonZeroU8, Saturating, Wrapping}, ops::*, time::{Duration, Instant}};
use std::{marker::PhantomData, ops::{Index, IndexMut}};

use math::*;
use text_extension::console_color::*;
use util::*;

use smallvec::{SmallVec,smallvec};

macro_rules! custom_assert {
    ($condition:expr) => {
        debug_assert!($condition);
        // assert!($condition);
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoardResult
{
    WinnerIs(Team),
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Team
{
    White  = 0,
    Black  = 1,
    Yellow = 2,
    Green  = 3,
}
impl Team
{
    pub const LENGHT : usize = 4; 
    pub const ALL : [Team; Self::LENGHT] = [Team::White, Team::Black, Team::Yellow, Team::Green];
    pub fn iter() -> impl Iterator<Item = Team>  { Self::ALL.iter().copied() }

    pub fn is_blue  (self) -> bool { matches!(self, Team::White  ) }
    pub fn is_red   (self) -> bool { matches!(self, Team::Black   ) }
    pub fn is_yellow(self) -> bool { matches!(self, Team::Yellow) }
    pub fn is_green (self) -> bool { matches!(self, Team::Green ) }

    pub fn next(self) -> Self
    {
        if self as usize + 1 >= Self::LENGHT { Self::White } else { Self::from_usize(self as usize + 1)}
    }

    pub fn from_usize(val : usize) -> Self { Self::ALL[val] }
}
impl Display for Team { fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult { write!(f, "{:?}", self) }}

impl Team
{
    pub fn belong(self, team : TeamsFlags) -> bool { team.have_flag(self.flags()) }
    pub fn flags(self) -> TeamsFlags { TeamsFlags(1 << (self as u8) << TeamsFlags::TEAM_OFFSET) }
}

pub type TeamsFlagsType = u8;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct TeamsFlags(u8);
impl TeamsFlags
{
    pub const TEAM_COUNT    : TeamsFlagsType = Team::LENGHT as u8;
    pub const TEAM_OFFSET   : TeamsFlagsType = 0;
    pub const TEAM_MASK     : TeamsFlags = Self(((1 << (Self::TEAM_COUNT))-1) << Self::TEAM_OFFSET);

    /* 
    pub const TEAM_BLUE   : Self = Self(1 << (Team::Blue    as u8) << Self::TEAM_OFFSET);
    pub const TEAM_RED    : Self = Self(1 << (Team::Red     as u8) << Self::TEAM_OFFSET);
    pub const TEAM_GREEN  : Self = Self(1 << (Team::Green   as u8) << Self::TEAM_OFFSET);
    pub const TEAM_YELLOW : Self = Self(1 << (Team::Yellow  as u8) << Self::TEAM_OFFSET);
    */

    pub fn teams_flags(self) -> Self { self & Self::TEAM_MASK }
    pub fn iter_team(self) -> impl Iterator<Item = Team> { Team::ALL.iter().copied().filter(move |e| e.belong(self))  }
    
    pub fn count(self) -> u32 { self.0.count_ones() }

    /// A piece can belong to multiple team
    pub fn is_also_team_blue(self) -> bool { self.have_flag(Team::White.flags()) }
    /// A piece can belong to multiple team
    pub fn is_also_team_red(self) -> bool { self.have_flag(Team::Black.flags()) }
    
    /// A piece can belong to multiple team
    pub fn is_also_team(self, t : Team) -> bool { t.belong(self) }

    pub fn display_color(self) -> &'static str
    {
        if self.is_also_team_blue() && self.is_also_team_red() { return YELLOW_FOREGROUND; } 
        if self.is_also_team_blue() { return CYAN_FOREGROUND; } 
        if self.is_also_team_red() { return RED_FOREGROUND; } 
        WHITE_FOREGROUND
    }
    
    /// Bit Flags related
    pub const ZERO : Self = Self(0);
    pub const fn is_none_flag(self) -> bool { self.0 == 0 }

    pub const fn is_exactly_flag(self, val : Self) -> bool { (self.0 & val.0) == val.0 }
    pub const fn have_flag   (self, val : Self) -> bool { (self.0 & val.0) != 0 }
    pub const fn with_flag   (self, val : Self) -> Self { Self(self.0 | val.0) }
    pub const fn without_flag(self, val : Self) -> Self { Self(self.0 & !val.0) }
    pub const fn toggle_flag (self, val : Self) -> Self { Self(self.0 ^ val.0) }
    pub const fn set_flag    (self, val : Self, set : bool) -> Self { if set { self.with_flag(val) } else { self.without_flag(val) } } 
}
impl BitOr for TeamsFlags { type Output=Self; fn bitor(self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) }}
impl BitAnd for TeamsFlags { type Output=Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) }}
impl BitXor for TeamsFlags { type Output=Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) }}
impl Not for TeamsFlags{ type Output=Self; fn not(self) -> Self::Output { Self(!self.0) }}
impl BitOrAssign for TeamsFlags { fn bitor_assign(&mut self, rhs: Self) { *self = self.bitor(rhs) }}
impl BitAndAssign for TeamsFlags { fn bitand_assign(&mut self, rhs: Self) { *self = self.bitand(rhs) }}
impl BitXorAssign for TeamsFlags { fn bitxor_assign(&mut self, rhs: Self)  { *self = self.bitxor(rhs) }}


pub type PieceFlagsType = u16;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PieceFlags(pub PieceFlagsType);
///! Current can hold 8 bit info, no more because of u8
impl PieceFlags
{
    /// Move Ability
    pub const ABILITY_COUNT      : PieceFlagsType = 7;
    pub const ABILITY_OFFSET     : PieceFlagsType = 0; // use the previous ???_OFFSET 
    pub const ABILITY_MASK       : PieceFlags = Self(((1 << (Self::ABILITY_COUNT))-1) << Self::ABILITY_OFFSET);

    pub const AB_CHESS_BISHOP    : PieceFlags = Self(0b1 << Self::ABILITY_OFFSET);
    pub const AB_CHESS_ROOK      : PieceFlags = Self(0b10 << Self::ABILITY_OFFSET);
    pub const AB_CHESS_KNIGHT    : PieceFlags = Self(0b100 << Self::ABILITY_OFFSET);
    pub const AB_CHESS_PAWN      : PieceFlags = Self(0b1000 << Self::ABILITY_OFFSET);
    pub const AB_CHESS_KING      : PieceFlags = Self(0b10000 << Self::ABILITY_OFFSET);
    pub const AB_DAME_KING       : PieceFlags = Self(0b100000 << Self::ABILITY_OFFSET);
    pub const AB_DAME_PAWN       : PieceFlags = Self(0b1000000 << Self::ABILITY_OFFSET);

    pub const AB_CHESS_QUEEN           : PieceFlags = Self(Self::AB_CHESS_BISHOP.0 | Self::AB_CHESS_ROOK.0);

    pub fn ability(self) -> Self { self & Self::ABILITY_MASK }
    pub fn have_any_ability(self) -> bool { self.ability() != Self::ZERO }

    pub fn can_be_promoted(self) -> bool { self.is_also_chess_pawn() || self.is_also_dame_pawn() }

    pub fn traditionnal_value(self) -> PieceValue
    {
        let mut val : PieceValue = 0;
        if self.have_flag(Self::AB_CHESS_PAWN  ) { val += 1; }
        if self.have_flag(Self::AB_CHESS_KNIGHT) { val += 3; }
        if self.have_flag(Self::AB_CHESS_BISHOP) { val += 3; }
        if self.have_flag(Self::AB_CHESS_ROOK  ) { val += 5; }
        if self.have_flag(Self::AB_CHESS_KING  ) { val += 2; }

        let nb_ability = self.ability().0.count_ones() as PieceValue;
        // +1 bonus for each extra ability. So value(QUEEN) = value(ROOK) + value(FOOL) + 1
        if nb_ability > 0 { val += nb_ability -1; }
        val
    }

    pub fn ai_value(self) -> PieceValue
    {
        // A single team can have multiple crowned value (those peiece can also belong to other oteam as well)
        // In any case, no overflow should happen.
        // The result should reflect the total piece value, so Saturating(PieceValue) is 
        if self.have_flag(Self::CROWN ) { return 1000; }

        let mut val : PieceValue = 0;
        if self.have_flag(Self::AB_CHESS_PAWN  ) { val += 1; }
        if self.have_flag(Self::AB_CHESS_KNIGHT) { val += 3; }
        if self.have_flag(Self::AB_CHESS_BISHOP) { val += 4; }
        if self.have_flag(Self::AB_CHESS_ROOK  ) { val += 5; }
        if self.have_flag(Self::AB_CHESS_KING  ) { val += 2; }

        if self.have_flag(Self::AB_DAME_PAWN  ) { val += 1; }
        if self.have_flag(Self::AB_DAME_KING  ) { val += 3; }


        let nb_ability = self.ability().0.count_ones() as PieceValue;
        // +1 bonus for each extra ability. So value(QUEEN) = value(ROOK) + value(FOOL) + 1
        if nb_ability > 0 { val += nb_ability -1; }
        val
    }

    pub fn promote(self, have_relic_kirby : bool) -> Self
    {
        let mut promotion = Self::ZERO;
        if self.is_also_chess_pawn() { promotion |= Self::AB_CHESS_QUEEN; }
        if self.is_also_dame_pawn()  { promotion |= Self::AB_DAME_KING; }

        if have_relic_kirby
        {
            self | promotion
        }else
        {
            (self & (!PieceFlags::ABILITY_MASK)) | promotion
        }
    }

    pub fn is_also_chess_pawn  (self) -> bool { self.have_flag(Self::AB_CHESS_PAWN  ) }
    pub fn is_also_chess_knight(self) -> bool { self.have_flag(Self::AB_CHESS_KNIGHT) }
    pub fn is_also_chess_king  (self) -> bool { self.have_flag(Self::AB_CHESS_KING  ) }
    pub fn is_also_chess_rook  (self) -> bool { self.have_flag(Self::AB_CHESS_ROOK  ) }
    pub fn is_also_chess_bishop(self) -> bool { self.have_flag(Self::AB_CHESS_BISHOP) }
    pub fn is_also_dame_king   (self) -> bool { self.have_flag(Self::AB_DAME_KING) }
    pub fn is_also_dame_pawn   (self) -> bool { self.have_flag(Self::AB_DAME_PAWN) }

    pub fn is_also_chess_queen (self) -> bool { (self & Self::ABILITY_MASK).is_exactly_flag(Self::AB_CHESS_QUEEN) }

    pub fn is_empty_ability(self) -> bool { self.ability() == PieceFlags::ZERO }


    // Flags
    pub const FLAGS_COUNT    : PieceFlagsType = 3;
    pub const FLAGS_OFFSET   : PieceFlagsType = Self::ABILITY_OFFSET + Self::ABILITY_COUNT; // use the previous ???_OFFSET 
    pub const FLAGS_MASK     : PieceFlagsType = ((1 << (Self::FLAGS_COUNT))-1) << Self::FLAGS_OFFSET;

    /// End of the game for the teams who lose any piece with a crown
    pub const CROWN          : PieceFlags = Self(0b1 << Self::FLAGS_OFFSET);
    pub const PARTIAL_PIN    : PieceFlags = Self(0b10 << Self::FLAGS_OFFSET);
    pub const TOTAL_PIN      : PieceFlags = Self(0b100 << Self::FLAGS_OFFSET);


    /// Bit Flags related
    pub const ZERO : Self = Self(0);
    pub const fn is_none_flag(self) -> bool { self.0 == 0 }

    pub const fn is_exactly_flag(self, val : Self) -> bool { (self.0 & val.0) == val.0 }
    pub const fn have_flag   (self, val : Self) -> bool { (self.0 & val.0) != 0 }
    pub const fn with_flag   (self, val : Self) -> Self { Self(self.0 | val.0) }
    pub const fn without_flag(self, val : Self) -> Self { Self(self.0 & !val.0) }
    pub const fn toggle_flag (self, val : Self) -> Self { Self(self.0 ^ val.0) }
    pub const fn set_flag    (self, val : Self, set : bool) -> Self { if set { self.with_flag(val) } else { self.without_flag(val) } }
    
}

impl BitOr for PieceFlags { type Output=Self; fn bitor(self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) }}
impl BitAnd for PieceFlags { type Output=Self; fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) }}
impl BitXor for PieceFlags { type Output=Self; fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) }}
impl Not for PieceFlags { type Output=Self; fn not(self) -> Self::Output { Self(!self.0) }}
impl BitOrAssign for PieceFlags { fn bitor_assign(&mut self, rhs: Self) { *self = self.bitor(rhs) }}
impl BitAndAssign for PieceFlags { fn bitand_assign(&mut self, rhs: Self) { *self = self.bitand(rhs) }}
impl BitXorAssign for PieceFlags { fn bitxor_assign(&mut self, rhs: Self)  { *self = self.bitxor(rhs) }}


impl PieceFlags
{
    pub fn display_char(self) -> char
    {
        let mut c = '?';
        if self.ability().is_none_flag() { c = ' '; }

        if self.is_exactly_flag(Self::AB_CHESS_PAWN  ) { c = 'o'; }
        if self.is_exactly_flag(Self::AB_CHESS_BISHOP) { c = 'B'; }
        if self.is_exactly_flag(Self::AB_CHESS_ROOK  ) { c = 'R'; }
        if self.is_exactly_flag(Self::AB_CHESS_QUEEN )       { c = 'Q'; }
        if self.is_exactly_flag(Self::AB_CHESS_KING  ) { c = 'K'; }
        if self.is_exactly_flag(Self::AB_CHESS_KNIGHT) { c = 'N'; }
        if self.is_exactly_flag(Self::AB_DAME_PAWN) { c = 'x'; }
        if self.is_exactly_flag(Self::AB_DAME_KING) { c = 'X'; }

        c
    }
}
impl Display for PieceFlags { fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult { write!(f, "{}", self.display_char()) }}

//pub type AtIntType = i8;
// Todo : change for i8. i16 for debug
pub type AtIntType = i16;
pub type At = C2<AtIntType>;
pub fn at(x: AtIntType, y: AtIntType) -> At { At::new(x, y) }

pub fn display_at(a : At, f : &mut Formatter<'_>) -> DisplayResult { write!(f, "{}{}", char::from_u32('a' as u32 + a.x as u32).unwrap(),  char::from_u32('1' as u32 + a.y as u32).unwrap()) }

pub type NbTimeMoved = Saturating<u8>;

pub type TileTravelType  = u16;
pub type TileTravel  = Saturating<TileTravelType>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Piece
{
    pub flags : PieceFlags,
    pub old_pos : At,
    pub last_turn_moved : Turn,
    pub last_action_moved : Turn,
    pub nb_time_moved : NbTimeMoved,

    pub teams_index : PieceTeamsIndex,

    /// manhattan distance
    pub distance_travel_total : TileTravel,
}
impl Default for Piece 
{ 
    fn default() -> Self 
    { 
        Self 
        { 
            flags: ___(), 
            old_pos: ___(),
            last_turn_moved: -1,
            last_action_moved : -1,
            nb_time_moved : Saturating(0),
            distance_travel_total : Saturating(0),
            teams_index: ___(),
        }
    }
}
impl Piece
{
    pub fn new(flags : PieceFlags, old_pos : At) -> Self { Self { flags, old_pos, ..Default::default() }}
    pub fn already_move(&self) -> bool { self.last_turn_moved >= 0 }

    pub fn teams_flags(&self) -> TeamsFlags { self.teams_index.teams_flags() }
    pub fn iter_team(&self) -> impl Iterator<Item = Team> + '_ { self.teams_index.iter_team()  }

    pub fn is_also_team_blue(&self) -> bool { self.is_also_team(Team::White) }
    pub fn is_also_team_red(&self) -> bool { self.is_also_team(Team::Black) }
    pub fn is_also_team(&self, t : Team) -> bool { self.teams_index.belong_to(t) }
    
}
impl Deref for Piece { type Target=PieceFlags; fn deref(&self) -> &Self::Target { &self.flags }}

pub type Score  = f64;
pub type Turn   = i16;
pub type Energy = i8;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Actions 
{
    all : Vec<Action>,
}
impl Deref for Actions { type Target=Vec<Action>; fn deref(&self) -> &Self::Target { &self.all }}
impl DerefMut for Actions { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.all }}
impl Actions 
{ 
    pub fn new() -> Self { Self { all: Vec::with_capacity(64) }}
    pub fn into_iter(self) -> impl Iterator<Item = Action> { self.all.into_iter() }

    pub fn iter_piece_action_and_result(&self, src : At) -> impl Iterator<Item=&Action> { self.iter().filter(move |e| e.id.is_move_src(src)) }
    pub fn iter_piece_action(&self, src : At) -> impl Iterator<Item=&Action> { self.iter_piece_action_and_result(src) }
    pub fn iter_piece_unit_action(&self, src : At) -> impl Iterator<Item=&UnitAction> { self.iter_piece_action(src).flat_map(|e| e.iter()) }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ActionID
{
    /// Move(src, dest)
    Move(At,At),
    // potion...
}
impl ActionID
{
    pub fn is_move_src(self, src : At) -> bool
    {
        match self
        {
            ActionID::Move(move_src, _) => move_src == src,
        }
    }
}

impl Display for ActionID
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self.clone()
        {
            ActionID::Move(src, dest) => { write!(f, "move ")?; display_at(src, f)?; write!(f, " to ")?; display_at(dest, f)?; },
        }
        Ok(())
    }
}

// small vec for more performance to avoid allocation allocation
pub type UnitActionVec = SmallVec<[UnitAction; 3]>;
#[derive(Clone, PartialEq, Debug)]
pub struct Action
{
    pub team : TeamsFlags,
    pub id : ActionID,
    
    /// score for the associated team if the action is play
    pub score : Score,
    /// Most action are 3 (including castling) : [Capture, Move, Energy]
    pub all : UnitActionVec,
}
impl Deref for Action { type Target=UnitActionVec; fn deref(&self) -> &Self::Target { &self.all }}
impl DerefMut for Action { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.all }}
impl Action 
{ 
    pub fn new(id : ActionID, team : TeamsFlags) -> Self { Self { id, all: smallvec![], team, score : 0. }}
    pub fn into_inter(self) -> impl Iterator<Item = UnitAction> { self.all.into_iter() }

    pub fn push(&mut self, game : &BoardGameFixedTime, unit_action : UnitAction) 
    {
        match unit_action
        {
            UnitAction::Capture(_, at) => { self.score += game[at].ai_value() as Score * 10.; },
            UnitAction::Promote(_) => { self.score += PieceFlags::AB_CHESS_QUEEN.ai_value() as Score * 10.; },
            UnitAction::EnergyAdd(how_many) => { self.score += how_many as Score; },
            //UnitAction::Swap(a, b) => { self.score += (a - b).length_manhattan() as Score * 0.1; },
            _ => {}
        }
        self.all.push(unit_action);
    }
}
impl Display for Action { fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult { write!(f, "{} {:?} (score: {})", self.id, self.all, self.score) }}

/// TeamPieceIdx::MAX = No Capture
pub type TeamPieceIdx = u8;
pub type TeamPieceIdxNonZero = NonZeroU8;

pub type MaybeTeamPieceIdx = Option<TeamPieceIdxNonZero>;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PieceTeamsIndex
{
    pub piece_index_for_each_team : [MaybeTeamPieceIdx;Team::LENGHT]
}
impl Index<Team> for PieceTeamsIndex { type Output=MaybeTeamPieceIdx; fn index(&self, index: Team) -> &Self::Output { &self.piece_index_for_each_team[index as u8 as usize] }}
impl IndexMut<Team> for PieceTeamsIndex { fn index_mut(&mut self, index: Team) -> &mut Self::Output { &mut self.piece_index_for_each_team[index as u8 as usize] }}

impl PieceTeamsIndex
{
    pub fn get_idx(&self, t : Team) -> Option<usize>
    { self[t].map(|e| TeamPieceIdx::from(e) as usize - 1) }

    pub fn iter(&self) -> impl Iterator<Item = (Team, usize)> + '_
    {
        self.piece_index_for_each_team.iter().copied().enumerate().filter_map(|(team_idx, idx)| if let Some(v) = idx { Some((Team::from_usize(team_idx), TeamPieceIdx::from(v) as usize - 1))} else { None })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Team, &mut TeamPieceIdxNonZero)> + '_
    {
        self.piece_index_for_each_team.iter_mut().enumerate().filter_map(|(team_idx, idx)| if let Some(v) = idx { Some((Team::from_usize(team_idx), v))} else { None })
    }

    pub fn belong_to(&self, t : Team) -> bool { self[t].is_some() }

    pub fn teams_flags(&self) -> TeamsFlags 
    {
        let mut flags = TeamsFlags::default();
        for t in self.iter_team()
        {
            flags |= t.flags();
        }
        flags
    }
    pub fn iter_team(&self) -> impl Iterator<Item = Team> + '_ { self.iter().map(|(t,_)| t)  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitAction
{
    Capture(Option<At>, At),
    Swap(At, At),
    EnergyAdd(Energy),
    Promote(At),
}
impl UnitAction
{
    pub fn is_capture(&self) -> bool { matches!(self, UnitAction::Capture(_,_)) }
    pub fn is_swap   (&self) -> bool { matches!(self, UnitAction::Swap(_,_)) }
    pub fn is_energy (&self) -> bool { matches!(self, UnitAction::EnergyAdd(_)) }
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct TeamPieceInfo
{
    pos : At,
}
impl Debug for TeamPieceInfo { fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult { display_at(self.pos, f) }}

pub type PieceValue    = u16;

/// Bit Flags type
pub type Relics = u8;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Relic
{
    /// An eye that prevent you from doing action that lead you in a losing position next turn if the adversary see it
    Anticipation = 0,
    /// Explode in a 3x3 square when a piece is captured. Pawn don't explode
    Explosive,
    /// Add a duck that can be controlled by the 2 players
    DuckButDifferent,
    /// Each piece absorb the piece captured piece
    Absorb,
    MoveTwiceInATurn,
}
impl From<Relic> for Relics
{
    fn from(value: Relic) -> Self {
        value as Relics
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct TeamData
{
    /// Use by the AI
    //  pub ai_score : Score,
    pub alive_piece_value : PieceValue,

    pub alive   : bool,
    pub is_present : bool,

    /// Used to decide who is first. Generally, add +1 for each action when the player are alive. Different player can have different score, so tie are allowed
    //  pub ranking_score : Score,

    /// Ordered by value to speed up the forcebrut
    pub piece_pos : SmallVec<[TeamPieceInfo; 16]>,

    pub relics : Relics,
}


//impl Display for TeamStat { fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult { write!(f, "score: {}, energy: {}", self.score, self.max_energy_when_start_turn) }}
impl Display for TeamData { fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult { write!(f, "score piece sum: {}", self.alive_piece_value) }}

#[derive(Clone, PartialEq, Debug)]
pub struct ActionAndResult
{
    pub action : Action,
    // The board is calculated temporary 
    // pub score : Option<Score>,
}


#[derive(Clone, PartialEq, Debug, Default)]
pub struct TeamsData
{
    val : [TeamData; Team::LENGHT],
}
impl Index<Team> for TeamsData { type Output=TeamData; fn index(&self, index: Team) -> &Self::Output { &self.val[index as u8 as usize] }}
impl IndexMut<Team> for TeamsData { fn index_mut(&mut self, index: Team) -> &mut Self::Output { &mut self.val[index as u8 as usize] }}

impl TeamsData
{
    fn fmt_team(&self, f: &mut Formatter<'_>, t : Team) -> DisplayResult 
    {
        write!(f, "{} : {{{}}}", t, self[t])
    }

    pub fn iter(&self) -> impl Iterator<Item = (Team, &TeamData)>
    {
        self.val.iter().enumerate().map(|(idx, team_data)| (Team::from_usize(idx), team_data))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Team, &mut TeamData)>
    {
        self.val.iter_mut().enumerate().map(|(idx, team_data)| (Team::from_usize(idx), team_data))
    }
}
impl Display for TeamsData
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult 
    {
        self.fmt_team(f, Team::White)?;
        write!(f, " ")?;
        self.fmt_team(f, Team::Black)?;
        write!(f, " ")?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct BoardGrid
{
    tiles : Vec<Piece>,
    size : At,
}
impl Index<At> for BoardGrid 
{ type Output=Piece; fn index(&self, index: At) -> &Self::Output { self.get(index) }}
impl IndexMut<At> for BoardGrid 
{ fn index_mut(&mut self, index: At) -> &mut Self::Output { self.get_mut(index) }}

impl Index<Point2> for BoardGrid 
{ type Output=Piece; fn index(&self, index: Point2) -> &Self::Output { self.get(index.map(|e| e as AtIntType)) }}
impl IndexMut<Point2> for BoardGrid 
{ fn index_mut(&mut self, index: Point2) -> &mut Self::Output { self.get_mut(index.map(|e| e as AtIntType)) }}

impl BoardGrid
{
    pub fn iter(&self) -> impl Iterator<Item=&Piece> { self.tiles.iter() }
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Piece> { self.tiles.iter_mut() }
    pub fn iter_idx(&self) -> impl Iterator<Item=At> { self.size.iter_area() }
}


impl BoardGrid
{
    pub fn new_empty(mut size : At) -> Self
    {
        size = size.max(At::ZERO);
        let mut s = Self
        {
            size,
            tiles : 
            {
                let nb_element = size.x * size.y;
                let mut v = Vec::with_capacity(nb_element as usize);
                for _ in 0.. nb_element
                {
                    v.push(___());
                }
                v
            }
        };
        for at in s.iter_idx()
        {
            s[at].old_pos = at;
            s[at].last_turn_moved = -1;
        }
        s
    }

    pub fn range_x(&self) -> Range<AtIntType> { 0..self.size.x }
    pub fn range_y(&self) -> Range<AtIntType> { 0..self.size.y }

    pub fn size (&self) -> At { self.size }

    #[inline] pub fn is_inside_x(&self, x : AtIntType) -> bool { x >= 0 && x < self.size.x }
    #[inline] pub fn is_inside_y(&self, y : AtIntType) -> bool { y >= 0 && y < self.size.y }

    pub fn is_inside(&self, idx : At) -> bool { self.is_inside_x(idx.x) && self.is_inside_y(idx.y) }

    fn coordinate_to_idx(&self, p : At) -> usize { p.x as usize *self.size.y as usize +p.y as usize }

    pub fn try_get(&self, pos : At) -> Option<&Piece> 
    { 
        let p = pos.into();
        if self.is_inside(p) { Some(&self.tiles[self.coordinate_to_idx(p)]) } else { None }
    }
    pub fn try_get_mut(&mut self, pos : At) -> Option<&mut Piece> 
    {
        if self.is_inside(pos) { Some(&mut self.tiles[(pos.x*self.size.y+pos.y) as usize]) } else { None }
    }

    /// panics if outside the grid
    pub fn get(&self, p : At) -> &Piece { self.try_get(p).unwrap() }
    /// panics if outside the grid
    pub fn get_mut(&mut self, p : At) -> &mut Piece { self.try_get_mut(p).unwrap() }

    pub fn swap(&mut self, a : At, b : At) 
    { 
        let a = self.coordinate_to_idx(a);
        let b = self.coordinate_to_idx(b);
        self.tiles.swap(a, b)
    }

    /// panics if outside the grid
    pub fn set(&mut self, val : Piece, p : At) { self[p] = val; }
    pub fn try_set(&mut self, val : Piece, p : At) -> bool { let idx = p.into(); if self.is_inside(idx) { self[idx] = val; true } else { false } }
}


#[derive(Clone, PartialEq, Debug)]
pub struct BoardGameNotStarted
{
    pub board : BoardGrid,
    pub team_data : TeamsData,

    pub captured : Vec<Piece>,

    pub nb_team_alive : u8,
    pub is_draw   : bool,

    /// Score for the current team
    // current_team_score : Score,
    /// current playing team
    pub current_team : Team,
    /// when energy reach zero, change the team turn
    pub current_nb_energy : Energy,
    pub current_nb_action_this_turn : Turn,
}
impl Deref for BoardGameNotStarted { type Target=BoardGrid; fn deref(&self) -> &Self::Target { &self.board }}
impl DerefMut for BoardGameNotStarted { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.board }}

impl Default for BoardGameNotStarted
{
    fn default() -> Self 
    {
        Self 
        { 
            board: ___(),

            current_team: Team::Yellow,

            current_nb_energy : 0,
            current_nb_action_this_turn : -1,

            team_data: ___(),
            nb_team_alive: 0,
            is_draw: false,
            captured: ___(),
        }
    }
}
impl BoardGameNotStarted
{
    pub fn new_empty(size : At) -> Self { Self { board: BoardGrid::new_empty(size), ..___() } }

    pub fn current_team_direction(&self) -> At { self.team_direction(self.current_team) }
    pub fn team_direction(&self, t : Team) -> At
    {
        match t
        {
            Team::White    =>  At::Y,
            Team::Black     => -At::Y,
            Team::Yellow  =>  At::X,
            Team::Green   => -At::X,
        }
    }


    fn integrity_is_ok(&self) -> bool
    {
        if self.team_data.val.iter().filter(|e| e.alive).count() as usize != self.nb_team_alive as usize { return false; }

        for t in Team::iter()
        {
            let mut sum_piece = self.iter_piece_team(t).map(|e| e.ai_value()).sum();
            if self.team_data[t].alive_piece_value != sum_piece { return false; }

            for p in self.team_data[t].piece_pos.iter()
            {
                if !self[p.pos].is_also_team(t) 
                { 
                    return false;
                }
            }
        }

        for at in self.iter_idx()
        {
            for (team_idx, v) in self[at].teams_index.piece_index_for_each_team.iter().enumerate()
            {
                let t = Team::from_usize(team_idx);

                //println!("team_idx:{team_idx}, t{t}, v:{v:?}");

                match v
                {
                    Some(idx_non_zero) => 
                    {
                        let idx = TeamPieceIdx::from(*idx_non_zero) as usize - 1;
                        //println!("x:{x}, y:{y}, team:{t}, idx:{idx}");

                        if self.team_data[t].piece_pos[idx].pos != at 
                        { 
                            return false;
                        }
                    },
                    None => 
                    {
                        if self.team_data[t].piece_pos.iter().any(|e| e.pos == at) 
                        { 
                            return false;
                        }
                    },
                }
            }
        }
        true
    }
}




#[derive(Clone, PartialEq, Debug, Default)]
pub struct BoardGameFixedTime
{
    pub data : BoardGameNotStarted,

    /// Number of time
    pub turn : Turn,
    /// Number of actions
    pub nb_actions : Turn,

    //pub actions_and_result : Vec<ActionAndResult>,
    pub actions : Actions,
}
impl Deref for BoardGameFixedTime { type Target=BoardGameNotStarted; fn deref(&self) -> &Self::Target { &self.data } }
impl DerefMut for BoardGameFixedTime { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data } }

impl BoardGameFixedTime 
{
    pub fn is_end_of_the_game(&self) -> bool { self.nb_team_alive <= 1  }
    pub fn end_game_result(&self) -> Option<BoardResult>
    {
        if self.is_end_of_the_game()
        {
            if !self.is_draw
            {
                for (t, data) in self.iter_team_data()
                {
                    if data.alive { return Some(BoardResult::WinnerIs(t)); }
                }
            }
            Some(BoardResult::Draw)
        }else
        {
            None
        }
    }
}

impl BoardGameFixedTime 
{
    pub fn iter_idx(&self) -> impl Iterator<Item=At> { self.size().iter_area() }

    pub fn piece_belong_to_current_team_turn(&self, at : At) -> bool { self[at].teams_index.belong_to(self.current_team) }

    pub fn action_id_to_action(&self, action_id : ActionID) -> Option<&Action> 
    {
        self.actions.iter().find(|e| e.id == action_id)
    }
    /// return true if the action can be played this turn
    pub fn action_id_is_valid(&self, action_id : ActionID) -> bool { self.action_id_to_action(action_id).is_some() }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct MinimaxResult
{
    pub score : Score,
    pub action_id : Option<ActionID>,

    pub stat_nb_action_evaluated : usize,
    pub stat_nb_depth_evaluated  : usize,
}
impl MinimaxResult
{
    pub fn update_stat(&mut self, sub : &Self)
    {
        self.stat_nb_action_evaluated += sub.stat_nb_action_evaluated;
    }
}

impl BoardGameFixedTime 
{
    fn piece_positional_value(&self, at : At) -> Score
    {
        -self.distance_to_promoting_tile(at) as Score
    }

    fn eval_team_position(&self, t : Team, current_depth : usize, maxi_depth : usize) -> MinimaxResult 
    { 
        MinimaxResult { score: self._eval_team_position(t, current_depth, maxi_depth), stat_nb_action_evaluated : 1, ..___() }
    }

    fn _eval_team_position(&self, t : Team, current_depth : usize, maxi_depth : usize) -> Score 
    {
        let rev_depth = (maxi_depth + 1) - current_depth ;
        if !self.team_data[t].alive 
        {
            // The later your defeat arrive, the better it is
            return rev_depth as Score * -10E64;
        }else if self.is_end_of_the_game()
        {
            // Alive and end of the game
            if self.is_draw
            {
                return rev_depth as Score * -10E64;
            }else
            {
                // victory
                return rev_depth as Score * 10E64;
            };
        }

        let mut score = 0 as Score; //self.team_data[t].alive_piece_value as Score * 2.;
        for team in Team::iter()
        {
            // positive

            let piece_score = 
            if self.team_data[team].alive 
            { 
                self.team_data[team].alive_piece_value as Score 
                //* if team == t { 2. } else { 1.0 }
            } 
            else 
            {
                0. as Score 
            };

            let nb_action_score = if team == self.current_team
            {
                self.actions.len() as Score
            }else
            {
                let nb_action = self.actions_for_team_with_anticipation(team, true).len();
                nb_action as Score
            };

            let piece_positionnal_score : Score = self.team_data[team].piece_pos
                .iter().map(|e| self.piece_positional_value(e.pos)).sum();
            
            let local_score = piece_score * 128. + piece_positionnal_score * 4. + nb_action_score;

            // squared for multiplayer in order to minimize each player score equaly
            score += (local_score * local_score) * if t == team { 1. } else { -1. };
        }
        score
    }

    /// maybe can do somethings to avoid allocating a new actions vector each time ?
    pub fn iter_next_state(&self, apply_anticipation : bool) -> impl Iterator<Item = (ActionID, Self)> + '_
    {
        self.actions.iter().map(move |e| (e.id, self.execute_action(&e, apply_anticipation)))
    }

    // Thank to https://www.youtube.com/watch?v=zp3VMe0Jpf8 for the tutorial
    fn _minimax(&self, t : Team, mut current_depth : usize, maxi_depth : usize, mut alpha : Score, mut beta : Score) -> MinimaxResult
    {
        current_depth += 1;
        if current_depth > maxi_depth || self.is_end_of_the_game() { return self.eval_team_position(t, current_depth, maxi_depth); }

        let mut best = MinimaxResult::default();
        best.action_id = Some(self.actions[0].id);

        let apply_anticipation = false;

        if self.current_team == t
        {
            // maximize player score
            best.score = Score::MIN;
            for (action_id, next) in self.iter_next_state(apply_anticipation)
            {
                let cur = next._minimax(t, current_depth, maxi_depth, alpha, beta);
                best.update_stat(&cur);

                if cur.score >  best.score { best.score = cur.score; best.action_id = Some(action_id); }
                
                if cur.score >= beta
                { 
                    best.action_id = Some(action_id);
                    debug_assert!(best.action_id.is_some());
                    return best;
                }
                if cur.score >  alpha { alpha = cur.score; if next.is_end_of_the_game() { break; } }
            }

        }else
        {
            // minimize the player score
            best.score = Score::MAX;
            for (action_id, next) in self.iter_next_state(apply_anticipation)
            {
                let cur = next._minimax(t, current_depth, maxi_depth, alpha, beta);
                best.update_stat(&cur);

                if cur.score < best.score { best.score = cur.score; best.action_id = Some(action_id); }
                
                
                if cur.score <= alpha 
                { 
                    best.action_id = Some(action_id);
                    debug_assert!(best.action_id.is_some());
                    return best;
                }
                if cur.score <  beta { beta = cur.score; if next.is_end_of_the_game() { break; } }
            }
        }

        debug_assert!(best.action_id.is_some());
        return best;
    }

    pub fn ai_best_move(&self) -> ActionID { self.ai_minimax_default().action_id.unwrap() }
    pub fn ai_minimax_default(&self) -> MinimaxResult 
    { 
        let minimax_default_depth = 4;
        self.minimax_custom(minimax_default_depth)
    }

    pub fn minimax_custom(&self, max_depth : usize) -> MinimaxResult
    {
        if max_depth == 0 
        { 
            // return a pseudo random move for depth 0
            return MinimaxResult{ score: Score::MIN, action_id: Some(self.actions[(self.turn as usize * 71) % self.actions.len()].id), stat_nb_action_evaluated: 0, stat_nb_depth_evaluated : 0 } ;
        }
        if self.actions.len() == 1
        {
            return MinimaxResult{ score: Score::MIN, action_id: Some(self.actions[0].id), stat_nb_action_evaluated: 0, stat_nb_depth_evaluated : 0 } ;
        }
        //let mut maxi = Score::MIN;
        let mut r = self._minimax(self.current_team, 0, max_depth, Score::MIN, Score::MAX);
        r.stat_nb_depth_evaluated = max_depth;
        r
    }
}

impl Display for BoardGameFixedTime
{
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult
    {
        let s = self.size();
        writeln!(f, "Turn #{}, {} play (team stat : {})", self.turn, self.current_team, self.team_data)?;

        for _ in 0..s.x*2+3 { write!(f, "=")?; }
        
        writeln!(f)?;
        for y in (0..s.y).rev()
        {
            write!(f, "| ")?;
            for x in 0..s.x
            {
                let idx = at(x, y);
                let c = self.board[idx].display_char();
                if self.turn > 0 && self.was_moved_last_turn_or_this_turn(idx) { write!(f, "{}", MAGENTA_BACKGROUND)?;} else
                {
                    //write!(f, "{}", if (x+y) %2 == 0 { GREY_BACKGROUND } else { BLACK_BACKGROUND })?;
                }
                if c != ' '
                {
                    write!(f, "{}{}", self.board[idx].teams_flags().display_color(), c)?;
                }else
                {
                    write!(f, "{}{}", GREY_FOREGROUND, if (x+y) % 2 == 0 { '.' } else { ' ' })?;
                }
                write!(f, "{} ", COLOR_RESET)?;
            }
            writeln!(f, "| {} {}", y+1, y)?;
        }
        for _ in 0..s.x*2+3 { write!(f, "=")?; }
        writeln!(f)?;

        write!(f, "  ")?;
        for x in 0..s.x { write!(f, "{} ", char::from_u32('a' as u32 + x as u32).unwrap())?; }
        writeln!(f)?;
        write!(f, "  ")?;
        for x in 0..s.x { write!(f, "{} ", char::from_u32('0' as u32 + x as u32).unwrap())?; }
        writeln!(f)?;
        writeln!(f)?;

        writeln!(f, "{} actions :", self.actions.len())?;
        for c in self.actions.iter()
        {
            writeln!(f, "  - {}", c)?;
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct MoveResult
{
    can_move : bool,
    nb_captured : usize,
}
impl MoveResult
{
    pub fn cant_move() -> Self 
    {
        Self { can_move: false, ..___() }
    }
}

impl BoardGameFixedTime
{
    pub fn was_moved_last_turn_or_this_turn(&self, at : At) -> bool
    {
        self[at].last_turn_moved >= self.turn - 1
    }

    pub fn was_moved_last_action_or_this_action(&self, at : At) -> bool
    {
        self[at].last_action_moved >= self.nb_actions - 1
    }
}

impl BoardGameNotStarted
{
    pub fn is_empty_ability_piece(&self, at : At) -> bool { self[at].is_empty_ability() }
}
impl BoardGameNotStarted
{
    pub fn iter_idx_team_current(&self) -> impl Iterator<Item = At> + '_  { self.iter_idx_team(self.current_team) }
    pub fn iter_idx_team(&self, t : Team) -> impl Iterator<Item = At> + '_  { self.team_data[t].piece_pos.iter().map(|e| e.pos) }

    pub fn iter_piece_team_current(&self) -> impl Iterator<Item = &Piece> + '_  { self.iter_piece_team(self.current_team) }
    pub fn iter_piece_team(&self, t : Team) -> impl Iterator<Item = &Piece> + '_  { self.iter_idx_team(t).map(|at| &self[at]) }

    pub fn set_empty_piece(&mut self, at : At)
    {
        let team_flags = self[at].teams_flags();
        for t in Team::iter()
        {
            if t.belong(team_flags)
            {
                self.piece_remove_team(at, t);
            }
        }
        self.piece_set_flags(at, PieceFlags::ZERO);
        custom_assert!(self.integrity_is_ok());
    }


    pub fn piece_add_team_and_set_flags(&mut self, at : At, t : Team, flags : PieceFlags)
    {
        self.piece_add_team(at, t);
        self.piece_set_flags(at, flags);
    }

    pub fn piece_add_teams_flags(&mut self, at : At, t : TeamsFlags) { for t in t.iter_team() { self.piece_add_team(at, t); }}
    pub fn piece_add_team(&mut self, at : At, t : Team)
    {
        // Already in the team
        if self[at].teams_index[t].is_some() { return; }

        if !self.team_data[t].alive { self.team_data[t].alive = true; self.nb_team_alive += 1; }

        let value = self[at].ai_value();

        self.team_data[t].piece_pos.push(TeamPieceInfo { pos: at });
        self.team_data[t].alive_piece_value += value as PieceValue;
       
        self[at].teams_index[t] =  MaybeTeamPieceIdx::Some(NonZeroU8::new(self.team_data[t].piece_pos.len() as u8).unwrap());
        custom_assert!(self.integrity_is_ok());
    }
    pub fn piece_remove_team(&mut self, at : At, t : Team)
    {
        custom_assert!(self.integrity_is_ok());

        // Already in the team
        custom_assert!(self[at].teams_index[t].is_some());
        let value = self[at].ai_value();
        let idx = TeamPieceIdx::from(self[at].teams_index[t].unwrap()) as usize - 1;
        //self.team_data[t].remove_piece(idx, value);

        let (board, piece_pos) = (&mut self.board, &mut self.team_data[t].piece_pos);
        
        if idx == piece_pos.len() - 1
        {
            piece_pos.remove(idx);
        }else
        {
            piece_pos.swap_remove(idx);
            let p = piece_pos[idx].pos;
            //board[at.x as usize][at.y as usize].teams_index[board[p.x as usize][p.y as usize]] = Some(idx);
            board[p].teams_index[t] = Some(TeamPieceIdxNonZero::new((idx + 1) as TeamPieceIdx).unwrap());
        }

        self.team_data[t].alive_piece_value -= value as PieceValue;

        self[at].teams_index[t] = None;
        custom_assert!(self.integrity_is_ok());
    }
    pub fn piece_set_flags(&mut self, at : At, flags : PieceFlags)
    {
        let p = &mut self[at];

        let old_value = p.ai_value();
        p.flags = flags;
        let new_value = p.ai_value();

        if new_value != old_value
        {
            let team_flags = self[at].teams_flags();
            for t in Team::iter()
            {
                if t.belong(team_flags)
                {
                    self.team_data[t].alive_piece_value = self.team_data[t].alive_piece_value - old_value + new_value;
                }
            }
        }
        custom_assert!(self.integrity_is_ok());
    }
}

struct LineOfSightStat
{
    //energy_lose_per_move : Energy,
    //energy_lose_is_traversed : Energy,
    delta : At, 
    max_move : isize,
    can_capture : bool,
}
impl Default for LineOfSightStat
{
    fn default() -> Self {
        Self 
        { 
            //energy_lose_per_move: Default::default(),
            //energy_lose_is_traversed: 1,
            delta: Default::default(),
            max_move: Default::default(),
            can_capture : true,
        }
    }
}

impl BoardGameFixedTime
{
    pub const ORTHO_DELTA : [At; 4] = 
    [
        At::new(0, 1), At::new(1, 0), At::new(0, -1), At::new(-1, 0),
    ];
    pub const DIAG_DELTA  : [At; 4] = 
    [
        At::new(1, 1), At::new(1, -1), At::new(-1, -1), At::new(-1, 1),
    ];
    pub const INF : int = 64;

    /// 2 pieces can be enemy and friend at the same time if they belong to multiple teams
    /// 
    /// true if it is an capturable enemy
    pub fn are_capturable_enemy (&self, src : At, dest : At) -> bool 
    { 
        self.can_be_captured_by_any(dest) && self.are_enemy_but_can_be_not_capturable(src, dest)
    }

    pub fn can_be_captured_by_any (&self, dest : At) -> bool 
    { 
        let dest_flag = self[dest].flags;
        !dest_flag.is_none_flag()
    }

    /// Such as teamless empty tile
    pub fn are_enemy_but_can_be_not_capturable (&self, src : At, dest : At) -> bool 
    { 
        let dest_teams_flags = self[dest].teams_flags();
        let src_teams_flags = self[src].teams_flags();
        src_teams_flags != dest_teams_flags || src_teams_flags.count() > 1
    }

    /// 2 pieces can be enemy and friend at the same time if they belong to multiple teams
    pub fn are_friend (&self, src : At, dest : At) -> bool 
    { 
        !self.are_enemy_but_can_be_not_capturable(src, dest)
        //(self[src].teams_flags() & self[dest].teams_flags()) != TeamsFlags::ZERO 
    }

    fn distance_to_promoting_tile(&self, src : At) -> AtIntType
    {
        if !self[src].can_be_promoted() { return 0; }
        // sometime getting closer to the promoting tile is useless if your moveset prevent you from accessing it
        let mut dist = AtIntType::MAX;
        for t in self[src].iter_team() 
        {
            let team_dir = self.team_direction(t);
            
            if team_dir.x < 0 { dist = dist.min((0 - src.x).abs()); }
            if team_dir.x > 0 { dist = dist.min((self.size().x - 1 - src.x).abs()); }
            if team_dir.y < 0 { dist = dist.min((0 - src.y).abs()); }
            if team_dir.y > 0 { dist = dist.min((self.size().y - 1 - src.y).abs()); }
        }
        dist
    }

    fn is_on_promoting_tile(&self, teams : TeamsFlags, dest : At) -> bool
    {
        for t in teams.iter_team() 
        {
            let team_dir = self.team_direction(t);
            if team_dir.x < 0 && dest.x == 0 { return true; }
            if team_dir.x > 0 && dest.x == self.size().x -1 { return true; }
            if team_dir.y < 0 && dest.y == 0 { return true; }
            if team_dir.y > 0 && dest.y == self.size().y -1 { return true; }
        }
        false
    }

    fn capture_at(&self, action : &mut Action, src : At, dest : At) -> usize
    {
        let mut nb_captured = 1;
        action.push(self, UnitAction::Capture(Some(src), dest));
        let explosif = self.have_relic_at(src, Relic::Explosive);
        
        if explosif
        {
            for dx in [-1, 0, 1]
            {
                for dy in [-1, 0, 1]
                {
                    if dx == 0 && dy == 0 { continue; }
                    let p = dest + at(dx, dy);

                    if self.is_inside(p) && self.are_capturable_enemy(src, p) && !self[p].ability().is_exactly_flag(PieceFlags::AB_CHESS_PAWN)
                    {
                        action.push(self, UnitAction::Capture(Some(src), p));
                        nb_captured += 1;
                    }
                }
            }
        }

        nb_captured
    }

    fn can_move_to_custom(&self, actions : &mut Actions, src : At, dest : At, can_capture : bool, energy_add : Energy) -> MoveResult
    {
        if !self.is_inside(dest) || self.are_friend(src, dest)  { return MoveResult::cant_move(); }
        let mut action = Action::new(ActionID::Move(src, dest), self[src].teams_flags());
        let nb_captured = if self.are_capturable_enemy(src, dest) 
        { 
            if !can_capture { return MoveResult::cant_move() }
            self.capture_at(&mut action, src, dest)
        }  else { 0 };
        action.push(self, UnitAction::Swap(src, dest));
        if src != dest && self[src].can_be_promoted() && self.is_on_promoting_tile(self[src].teams_flags(), dest)
        {
            action.push(self, UnitAction::Promote(dest));
        }
        if energy_add != 0 { action.push(self, UnitAction::EnergyAdd(energy_add)); }
        actions.push(action);
        MoveResult { can_move : true, nb_captured }
    }

    fn can_move_to(&self, actions : &mut Actions, src : At, dest : At) -> MoveResult
    { self.can_move_to_custom(actions, src, dest, true, -1) }

    fn line_of_sight_default(&self, actions : &mut Actions, src : At, delta : At, mut max_move : isize)
    {
        self.line_of_sight(actions, src, LineOfSightStat{  max_move, delta, ..Default::default() }, -1)
    }

    fn line_of_sight(&self, actions : &mut Actions, src : At, mut stat : LineOfSightStat, energy_add : Energy)
    {
        if !stat.delta.have_length() { return; }
        let mut dest = src;

        while stat.max_move >= 1
        {
            stat.max_move -= 1;
            dest += stat.delta;
            let m = self.can_move_to_custom(actions, src, dest, stat.can_capture, energy_add);
            if !m.can_move || m.nb_captured > 0 { return; }
        }
    }

    fn actions_piece_chess_king(&self, actions : &mut Actions, king_src : At, apply_anticipation : bool)
    {
        for d in Self::ORTHO_DELTA { self.line_of_sight_default(actions, king_src, d, 1); }
        for d in Self::DIAG_DELTA  { self.line_of_sight_default(actions, king_src, d, 1); }

        // castle
        if self[king_src].nb_time_moved == Saturating(0)
        {
            for (king_dest, castle_src, castle_dest) in [(king_src.with_x(king_src.x-2), king_src.with_x(0), king_src.with_x(king_src.x-1)), (king_src.with_x(king_src.x+2), king_src.with_x(self.size().x()-1), king_src.with_x(king_src.x+1))]
            {
                if !self.is_inside(king_dest) || !self.is_inside(castle_dest) { continue; }
                // queen can't be here with 0 turn (she have the chess_rook ability)
                if !self[castle_src].is_also_chess_rook() || !self[castle_src].nb_time_moved == Saturating(0) { continue;} 

                let mut can_castle = true;

                let inc = (king_dest-king_src).x.signum();
                let mut pos = king_src;
                pos.x += inc;
                while pos != castle_src
                {
                    // already a piece on the way
                    if !self[pos].is_empty_ability() { can_castle = false; break; }
                    pos.x += inc;
                }

                if !can_castle { continue;}


                let n = self.next_playing_team_after(self[king_src].iter_team().next().unwrap());
                
                //if n == self.current_team { continue; }

                if apply_anticipation
                {
                    // calculate the opponent move it at the last second to check if a tile is under attack
                    let mut opponent_move = self.actions_for_team_with_anticipation(n, false);

                    pos = king_src;
                    while pos != king_dest
                    {
                        pos.x += inc;

                        if opponent_move.iter().any(|a| a.iter().any(|s| match s
                            {
                                UnitAction::Swap(_src, dest) => *dest == pos,
                                _ => false
                            }))
                        {
                            // under attack
                            can_castle = false; break;
                        }
                    }
                }
                
                if !can_castle { continue; }

                let mut action = Action::new(ActionID::Move(king_src, king_dest), self[king_src].teams_flags());
                action.push(self, UnitAction::Swap(king_src, king_dest));
                action.push(self, UnitAction::Swap(castle_src, castle_dest));
                action.push(self, UnitAction::EnergyAdd(-1));
                actions.push(action);
            }
        }
    }
    fn actions_piece_chess_bishop(&self, actions : &mut Actions, src : At) { for d in Self::DIAG_DELTA  { self.line_of_sight_default(actions, src, d, Self::INF); } }
    fn actions_piece_chess_rook  (&self, actions : &mut Actions, src : At) 
    { 
        for d in Self::ORTHO_DELTA  
        { self.line_of_sight_default(actions, src, d, Self::INF); } 
    }
    fn actions_piece_chess_knight(&self, actions : &mut Actions, src : At) 
    {
        // Rotation tricks
        let mut delta = At::new(1, 2);
        for _ in 0..4 { self.can_move_to(actions, src, src + delta); delta = delta.y_rx(); }
        delta.y = -delta.y;
        for _ in 0..4 { self.can_move_to(actions, src, src + delta); delta = delta.y_rx(); }
    }

    fn actions_piece_chess_pawn_dir(&self, actions : &mut Actions, src : At, dir : At)
    {
        let right = dir.y_x();
        let left  = dir.ry_rx();

        let right_dest = src + right + dir;
        let left_dest  = src + left + dir;

        if self.is_inside(right_dest) && self.are_capturable_enemy(src, right_dest) { self.can_move_to(actions, src, right_dest); }
        if self.is_inside(left_dest) && self.are_capturable_enemy(src, left_dest ) { self.can_move_to(actions, src, left_dest ); }

        if self.can_move_to_custom(actions, src, src + dir, false, -1).can_move && !self[src].already_move()
        {
            self.can_move_to_custom(actions, src, src + dir * 2, false, -1);
        }

        // *En passant*
        { 
            let all_en_passant_relative = [left, right];
            for en_passant_rel in all_en_passant_relative
            {
                let en_passant_attack = src + en_passant_rel;
                let en_passant_dest  = en_passant_attack + dir;
                if !self.is_inside(en_passant_attack) || !self.is_inside(en_passant_dest) { continue; }

                if self.are_capturable_enemy(src, en_passant_attack)
                {
                    let pawn = &self[en_passant_attack];
                    if
                         pawn.is_also_chess_pawn() &&
                         self.was_moved_last_turn_or_this_turn(en_passant_attack) &&
                         pawn.distance_travel_total == Saturating(2) && 
                         pawn.nb_time_moved == Saturating(1)
                    { 
                        self.can_move_to(actions, src, en_passant_dest);
                        actions.last_mut().unwrap().push(self, UnitAction::Capture(None, en_passant_attack));
                    }
                }
            }
        }
    }

    fn actions_piece_chess_pawn(&self, actions : &mut Actions, src : At)
    {
        for t in Team::iter()
        {
            if !self[src].is_also_team(t) { continue; }
            self.actions_piece_chess_pawn_dir(actions, src, self.team_direction(t));
        }
    }

    fn actions_piece_dame_pawn_for_team(&self, actions : &mut Actions, src : At, dir : At, can_move_backward : bool, mut only_attack : bool)
    {
        if self.current_nb_action_this_turn > 0
        {
            let this_turn = self[src].last_turn_moved;
            if  this_turn != self.turn { return; }
            only_attack = true;
        }

        let right = dir.y_x();
        let right_up = right + dir;
        let right_right_up_up = right_up * 2;
        let right_up_dest = src + right_up;
        let right_right_up_up_dest = src + right_right_up_up;
        let can_attack_right_up = self.is_inside(right_up_dest) && self.are_capturable_enemy(src, right_up_dest)  && self.is_inside(right_right_up_up_dest) && self[right_right_up_up_dest].is_none_flag();

        let left = dir.ry_rx();
        let left_up = left + dir;
        let left_left_up_up = left_up * 2;
        let left_up_dest = src + left_up;
        let left_left_up_up_dest = src + left_left_up_up;
        let can_attack_left_up = self.is_inside(left_up_dest) && self.are_capturable_enemy(src, left_up_dest)  && self.is_inside(left_left_up_up_dest) && self[left_left_up_up_dest].is_none_flag();

        let right_down = right - dir;
        let right_right_down_down = right_down * 2;
        let right_down_dest = src + right_down;
        let right_right_down_down_dest = src + right_right_down_down;
        let can_attack_right_down = self.is_inside(right_down_dest) && self.are_capturable_enemy(src, right_down_dest)  && self.is_inside(right_right_down_down_dest) && self[right_right_down_down_dest].is_none_flag();

        let left_down = left - dir;
        let left_left_down_down = left_down * 2;
        let left_down_dest = src + left_down;
        let left_left_down_down_dest = src + left_left_down_down;
        let can_attack_left_down = self.is_inside(left_down_dest) && self.are_capturable_enemy(src, left_down_dest)  && self.is_inside(left_left_down_down_dest) && self[left_left_down_down_dest].is_none_flag();

        if can_attack_left_up
        {
            let _ok = self.can_move_to_custom(actions, src , left_left_up_up_dest, false, 0).can_move;
            custom_assert!(_ok);
            actions.last_mut().unwrap().push(self, UnitAction::Capture(None, left_up_dest));
        } else if !only_attack
        {
            self.can_move_to_custom(actions, src, left_up_dest, false, -1);
        }

        if can_attack_right_up
        {
            let _ok = self.can_move_to_custom(actions, src , right_right_up_up_dest, false, 0).can_move;
            custom_assert!(_ok);
            actions.last_mut().unwrap().push(self, UnitAction::Capture(None, right_up_dest));
        }else if !only_attack
        {
            self.can_move_to_custom(actions, src, right_up_dest, false, -1);
        }

        if can_move_backward
        {
            if can_attack_left_down 
            {
                let _ok = self.can_move_to_custom(actions, src , left_left_down_down_dest, false, 0).can_move;
                custom_assert!(_ok);
                actions.last_mut().unwrap().push(self, UnitAction::Capture(None, left_down_dest));
            }else if !only_attack
            {
                self.can_move_to_custom(actions, src, left_down_dest, false, -1);
            }

            if can_attack_right_down
            {
                let _ok = self.can_move_to_custom(actions, src , right_right_down_down_dest, false, 0).can_move;
                custom_assert!(_ok);
                actions.last_mut().unwrap().push(self, UnitAction::Capture(None, right_down_dest));
            }else if !only_attack
            {
                self.can_move_to_custom(actions, src, right_down_dest, false, -1);
            }
        }

        /* 
        if can_attack_left_up || can_attack_right_up || can_attack_left_down || can_attack_right_down
        {
            if can_attack_left_up
            { 
                if self.can_move_to_custom(actions, src , left_left_up_up_dest, false, 0).can_move
                {
                    actions.last_mut().unwrap().push(self, UnitAction::Capture(None, left_up_dest));
                }
            }
            if can_attack_right_up
            { 
                self.can_move_to(actions, src , right_right_up_up_dest);
                actions.last_mut().unwrap().push(self, UnitAction::Capture(None, right_up_dest));
            }
            if can_attack_left_down
            { 
                self.can_move_to(actions, src , left_left_down_down_dest);
                actions.last_mut().unwrap().push(self, UnitAction::Capture(None, left_down_dest));
            }
            if can_attack_right_down
            { 
                self.can_move_to(actions, src , right_right_down_down_dest);
                actions.last_mut().unwrap().push(self, UnitAction::Capture(None, right_down_dest));
            }
        }//else
        */
        {
            /* 
            if can_move_backward
            {
                self.can_move_to_custom(actions, src, left_down_dest, false, -1);
                self.can_move_to_custom(actions, src, right_down_dest, false, -1);
            }
            */

            /* 
            if self.is_inside(left_up_dest) && self[left_up_dest].is_none_flag() { self.can_move_to(actions, src , left_up_dest); }
            if self.is_inside(right_up_dest) && self[right_up_dest].is_none_flag() { self.can_move_to(actions, src , right_up_dest); }
            if can_move_backward
            {
                if self.is_inside(left_down_dest) && self[left_down_dest].is_none_flag() { self.can_move_to(actions, src , left_down_dest); }
                if self.is_inside(right_down_dest) && self[right_down_dest].is_none_flag() { self.can_move_to(actions, src , right_down_dest); }
            }*/
        }
    }


    fn actions_piece_dame_pawn_custom(&self, actions : &mut Actions, src : At, can_move_backward : bool)
    {
        for t in Team::iter()
        {
            if !self[src].is_also_team(t) { continue; }
            self.actions_piece_dame_pawn_for_team(actions, src, self.team_direction(t), can_move_backward, false);
        }
    }

    fn actions_piece_dame_pawn(&self, actions : &mut Actions, src : At) 
    { self.actions_piece_dame_pawn_custom(actions, src, false); }

    fn actions_piece_dame_king(&self, actions : &mut Actions, src : At) 
    { self.actions_piece_dame_pawn_custom(actions, src, true); }

    /// Calculate the action for a given piece regardeless of the turn. Anticipation is not applied
    pub fn actions_piece(&self, actions : &mut Actions, src : At, apply_anticipation : bool)
    {
        if self[src].have_flag(PieceFlags::AB_CHESS_KING  ) { self.actions_piece_chess_king  (actions, src, apply_anticipation); } 
        if self[src].have_flag(PieceFlags::AB_CHESS_KNIGHT) { self.actions_piece_chess_knight(actions, src); } 
        if self[src].have_flag(PieceFlags::AB_CHESS_BISHOP) { self.actions_piece_chess_bishop(actions, src); } 
        if self[src].have_flag(PieceFlags::AB_CHESS_ROOK  ) { self.actions_piece_chess_rook  (actions, src); } 
        if self[src].have_flag(PieceFlags::AB_CHESS_PAWN  ) { self.actions_piece_chess_pawn  (actions, src); } 
        if self[src].have_flag(PieceFlags::AB_DAME_PAWN   ) { self.actions_piece_dame_pawn   (actions, src); } 
        if self[src].have_flag(PieceFlags::AB_DAME_KING   ) { self.actions_piece_dame_king   (actions, src); } 

        if apply_anticipation { self.relic_anticipation_apply_on(actions) }
    }


    pub fn iter_team_data(&self) -> impl Iterator<Item=(Team,&TeamData)> { self.team_data.iter() }
    pub fn current_team_data(&self) -> &TeamData { &self.team_data[self.current_team] }

    /// Possible action for th current turn
    /// Also include illegal action
    pub fn actions(&self) -> Actions { self.actions_for_team(self.current_team) }
    pub fn update_actions(&mut self, apply_anticipation : bool) 
    { 
        self.actions = self.actions();
        
        if apply_anticipation && self.current_team_data().relics.flag_have(Relic::Anticipation)
        {
            self.relic_anticipation_apply();
        }
    }

    /// If by doing this action, you lose in 100 % case with the following action, prevent you from doing the action
    pub fn relic_anticipation_judge_if_action_is_legal(&self, action : &Action) -> bool 
    {
        if !self.team_data[self.current_team].alive { return false; }
        if !action.team.is_also_team(self.current_team) { return true; }

        let after = self.execute_action(action, false);

        for (_, state) in after.iter_next_state(false)
        {
            if !state.team_data[self.current_team].alive 
            {
                return false;
            }
        }
        true
    }
    pub fn relic_anticipation_apply_on(&self, actions : &mut Actions) 
    {
        if !self.team_data[self.current_team].alive { return; }
        actions.retain(|e| self.relic_anticipation_judge_if_action_is_legal(e));
    }


    pub fn clear_pin_for_team(&mut self, t : Team)
    {
        for p in self.iter_mut().filter(|p| p.is_also_team(t))
        {
            p.flags = p.flags.without_flag(PieceFlags::TOTAL_PIN).without_flag(PieceFlags::PARTIAL_PIN);
        }
    }
    pub fn clear_pin_current_team(&mut self) { self.clear_pin_for_team(self.current_team) }

    pub fn concerned_by_relic_anticipation_at(&self, at : At) -> bool
    {
        for t in self[at].iter_team()
        {
            if self.team_data[t].relics.flag_have(Relic::Anticipation) { return true; }
        }
        false
    }

    /// Also calculate the pin
    pub fn relic_anticipation_apply(&mut self) 
    {
        self.clear_pin_current_team();
        let mut idx : Vec<(At,usize)> = self.iter_idx_team_current().map(|e| (e, 0)).collect();
        
        let mut tmp = vec![];
        std::mem::swap(&mut tmp, &mut self.actions);

        for action in tmp.into_iter()
        {
            if self.relic_anticipation_judge_if_action_is_legal(&action) 
            {
                self.actions.push(action);
                continue;
            }
            match action.id
            {
                ActionID::Move(src, _) => 
                { 
                    let i = self[src].teams_index.get_idx(self.current_team).unwrap();
                    idx[i].1 += 1;
                },
            }
        }

        let mut actions = Actions::new();

        for (at, _nb_pinned_move) in idx
        {
            actions.clear();
            self.actions_piece(&mut actions, at, false);
            if _nb_pinned_move != 0 
            {
                let mut f = self[at].flags;
                if _nb_pinned_move == actions.len() 
                {
                    f = f.set_flag(PieceFlags::TOTAL_PIN, true);
                }else
                {
                    f = f.set_flag(PieceFlags::PARTIAL_PIN, true);
                }
                self.piece_set_flags(at, f);
            } 
        }
    }


    pub fn actions_for_team(&self, t : Team) -> Actions { Self::actions_for_team_with_anticipation(self, t, false) }
    pub fn actions_for_team_with_anticipation(&self, t : Team, apply_anticipation : bool) -> Actions { let mut actions = Actions::new(); self.calculate_actions_for_team_with_anticipation(t, &mut actions, apply_anticipation); actions }
    pub fn calculate_actions_for_team(&self, t : Team, actions : &mut Actions) { self.calculate_actions_for_team_with_anticipation(t, actions, false)}
    pub fn calculate_actions_for_team_with_anticipation(&self, t : Team, actions : &mut Actions, apply_anticipation : bool) 
    { 
        for at in self.iter_idx_team(t) { self.actions_piece(actions, at, apply_anticipation); }
        
        // sort by 'best' action to speed up alpha beta pruning
        actions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    }


    /* 
    pub fn clear_results(&mut self)
    {
        for a in self.actions_and_result.iter_mut() { a.score = None; }
    }*/

    fn execute_action(&self, action : &Action, apply_anticipation : bool) -> Self
    {
        let mut s = self.execute_action_without_update(action);
        s.end_of_action(apply_anticipation);
        //custom_assert!((s.is_end_of_the_game() && s.actions.len() == 0) || (!s.is_end_of_the_game() && s.actions.len() != 0));
        s
    }

    fn execute_team_lose(&mut self, t : Team) 
    { 
        // already lose
        if !self.team_data[t].alive { return; }
        self.nb_team_alive -= 1;
        self.team_data[t].alive = false;
    }
    fn execute_teams_flags_lose(&mut self, flags : TeamsFlags)
    {
        for t in flags.iter_team()
        {
            self.execute_team_lose(t);
        }
    }

    fn piece_swap(&mut self, a : At, b : At)
    {
        let delta = b - a;
        let distance_travel_total = delta.length_manhattan() as TileTravelType;

        for (t, idx) in self.data.board[a].teams_index.iter() 
        { 
            self.data.team_data[t].piece_pos[idx].pos = b;
        }
        for (t, idx) in self.data.board[b].teams_index.iter() { self.data.team_data[t].piece_pos[idx].pos = a; }

        let tmp = self[a];
        self[a] = self[b];
        self[a].old_pos = b;

        self[b] = tmp;
        self[b].old_pos = a;

        for p in [a, b]
        {
            self[p].last_turn_moved = self.turn;
            self[p].last_action_moved = self.nb_actions;
            
            self[p].nb_time_moved += 1;
            self[p].distance_travel_total += distance_travel_total;
        }

        custom_assert!(self.integrity_is_ok());
    }

    fn have_relic_at(&self, at : At, relic : Relic) -> bool
    {
        self[at].iter_team().any(|t| self.team_data[t].relics.flag_have(relic))
    }

    fn execute_action_without_update(&self, action : &Action) -> Self
    {
        custom_assert!(!self.is_end_of_the_game());

        let mut s = Self
        {
            data : self.data.clone(),
            turn : self.turn,
            nb_actions : self.nb_actions+1,
            ..___()
        };

        for sub in action.iter().copied()
        {
            match sub
            {
                // Can't use (captured : Team, pieceIdx : usize) instead of (dest) because one piece can belong to multiple team
                UnitAction::Capture(src, dest) => 
                { 
                    if self.current_team_data().relics.flag_have(Relic::Absorb)
                    {
                        // Pov : you are kirby : absorbe the moveset of the piece you capture
                        if let Some(src) = src
                        {
                            s.piece_set_flags(src, self[src].flags | self[dest].ability())
                        }
                    }

                    if s[dest].have_flag(PieceFlags::CROWN) 
                    { 
                        s.execute_teams_flags_lose(s[dest].teams_flags());
                    }
                    s.captured.push(self[dest]);
                    s.set_empty_piece(dest);
                },
                UnitAction::Swap(a, b) => 
                {
                    s.piece_swap(a, b);
                },
                UnitAction::EnergyAdd(how_many) => s.current_nb_energy += how_many,
                UnitAction::Promote(at) => 
                {
                    let f = s[at].promote(self.have_relic_at(at, Relic::Absorb));
                    s.piece_set_flags(at, f);
                },
            }
        }

        //s.turn += 1;

        s
    }
}

impl BoardGameFixedTime
{
    pub fn init_new_turn(&mut self)
    {
        self.current_nb_energy = 1 + if self.current_team_data().relics.flag_have(Relic::MoveTwiceInATurn) { 1 } else { 0 };
        self.current_nb_action_this_turn = 0;
        self.turn += 1;
    }

    pub fn next_playing_team(&self) -> Team { self.next_playing_team_after(self.current_team) }
    pub fn next_playing_team_after(&self, team : Team) -> Team
    {
        // Todo, handle it in a different way can be better
        let mut futur_team = team;
        let mut nb_iter = 0;
        loop
        {
            futur_team = match futur_team
            {
                Team::White   => Team::Black,
                Team::Black    => Team::Yellow,
                Team::Yellow => Team::Green,
                Team::Green  => Team::White,
            };

            if self.team_data[futur_team].alive { break; }
            nb_iter += 1;
            if nb_iter > Team::LENGHT { break; }
        }
        futur_team
    }

    pub fn end_of_action(&mut self, apply_anticipation : bool)
    {
        self.nb_actions +=1;

        if self.current_nb_energy <= 0 
        {
            self.current_team = self.next_playing_team();
            self.init_new_turn();
        }else
        {
            self.current_nb_action_this_turn += 1;
        }

        self.actions.clear();
        if !self.is_end_of_the_game()
        {
            self.update_actions(apply_anticipation);
            if self.actions.len() == 0 
            { 
                if self.current_nb_action_this_turn == 0
                {
                    let current_team = self.current_team;

                    // I don't want any draw. It don't make sens because the kind can't do anythings, and it make ai harder to code
                    /* 
                    
                    // check if the last piece is checked
                    if self.nb_team_alive <= 2
                    {
                        let mut next = self.clone();
                        next.init_new_turn();
                        next.current_team = self.next_playing_team();
                        next.actions.clear();
                        next.update_actions(true);
                        if !next.actions.iter().any(|e| e.iter().any(|s| match &s
                        {
                            UnitAction::Capture(_, dest) => self[*dest].have_flag(PieceFlags::CROWN),
                            _ => false,
                        }))
                        {
                            // stale mate
                            self.is_draw = true;
                            for t in Team::iter()
                            {
                                self.execute_team_lose(t);
                            }
                        }
                    }
                    */
                    self.execute_team_lose(current_team);
                }else
                {
                    self.current_nb_energy = 0;
                    self.end_of_action(apply_anticipation);
                }
                // self._is_end_of_the_game = true;
            }
        }

        // Todo : change it to the 'best' move by default
        // self.ai_current_actions_and_result_idx = 0;
        // self.update_team_score();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToAnalyse
{
    idx : usize,
    action_idx : usize,
}

impl BoardGameFixedTime
{
    pub fn new(data : BoardGameNotStarted) -> Self
    {
        let mut s = Self { data: data, ..Default::default() };
        s.init();
        s.end_of_action(false);
        s
    }

    fn init(&mut self)
    {
        for t in Team::iter()
        {
            self.team_data[t].alive_piece_value = 0;
        }

        for t in Team::iter()
        {
            let mut at_least_one_piece = false;
            for at in self.iter_idx()
            {
                if self[at].is_also_team(t)
                {
                    self.team_data[t].alive_piece_value += self[at].ai_value() as PieceValue;
                    at_least_one_piece = true;
                }
            }

            if at_least_one_piece
            {
                self.team_data[t].is_present = true;
            }
        }

    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct BoardGame
{
    time_line : Vec<BoardGameFixedTime>,
    time_line_idx : usize,
}
impl BoardGame
{
    pub fn new(data : BoardGameNotStarted) -> Self { Self { time_line: vec![BoardGameFixedTime::new(data)], time_line_idx: 0 }}

    pub fn current(&self) -> &BoardGameFixedTime { &self.time_line[self.time_line_idx] }
    pub fn current_mut(&mut self) -> &mut BoardGameFixedTime { &mut self.time_line[self.time_line_idx] }

    pub fn can_undo(&self) -> bool { self.time_line_idx > 0 }
    pub fn undo(&mut self) -> bool { if self.can_undo() { self.time_line_idx -= 1; true } else { false }}

    pub fn can_redo(&self) -> bool { self.time_line_idx < self.time_line.len() -1 }
    pub fn redo(&mut self) -> bool { if self.can_redo() { self.time_line_idx += 1; true } else { false }}
}
impl Deref for BoardGame { type Target=BoardGameFixedTime; fn deref(&self) -> &Self::Target { self.current() }}
impl DerefMut for BoardGame { fn deref_mut(&mut self) -> &mut Self::Target { self.current_mut() }}

impl BoardGame
{
    pub fn console_display(&self) 
    {
        println!();
        println!("{:?}", self.team_data);
        println!();

        println!("Turn {}, playing {}, energy {}", self.time_line.len(), self.current_team, self.current_nb_energy);
        println!("{}", self.current());
    }
    
    fn console_input_line(&self) -> String
    {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) 
        {
            Ok(_) => { input = input.to_lowercase(); },
            Err(e) => { println!("error when reading : {}", e); },
        }
        input
    }

    pub fn console_input_at(&self, it : &mut impl Iterator<Item=char>) -> Result<At, String>
    {
        match it.next() 
        {
            Some(c) => 
            {
                let x = c as AtIntType - 'a' as AtIntType;
                if x >= 0 && x < self.size().x as AtIntType
                {
                    match it.next() 
                    {
                        Some(c) => 
                        {
                            let y = c as AtIntType - '0' as AtIntType;
                            if y >= 1 && y <= self.size().y as AtIntType
                            {
                                Ok(At::new(x, y-1))
                            }else
                            {
                                if y <= 0
                                {
                                    Err("Mising line : 1, 2, 3...".to_owned())
                                }else
                                {
                                    Err("Invalid line number".to_owned())
                                }
                            }
                        }
                        None => Err("Mising line : 1, 2, 3...".to_owned()),
                    }
                }else
                {
                    if x <= 0
                    {
                        Err("Mising column : a, b, c...".to_owned())
                    }else
                    {
                        Err("Invalid column letter".to_owned())
                    }
                }
            }
            None => Err("Mising column : a, b, c...".to_owned()),
        }
    }

    pub fn console_input(&mut self) -> ActionID
    {
        loop
        {
            match self.console_input_from_str(&self.console_input_line())
            {
                Ok(action_id) => 
                {
                    if self.current().actions.iter().find(|e| e.id == action_id).is_some()
                    {
                        return action_id;
                    }else
                    {
                        println!("Can't do that")
                    }
                },
                Err(err) => println!("{}", err),
            }
            self.console_display();
        }
    }

    pub fn console_ai_best_move(&self) -> ActionID
    {
        let start = Instant::now();
        let r = self.ai_minimax_default();
        let elapsed_s = start.elapsed().as_secs_f64();

        println!("Ai : {:>9} actions evaluated in {:>7.3}s with depth {} at Turn {:>2} before choosing to {}. ({:>9} actions/s)", r.stat_nb_action_evaluated, elapsed_s, r.stat_nb_depth_evaluated, self.turn+1, r.action_id.unwrap(), (r.stat_nb_action_evaluated as f64 / elapsed_s) as usize);
        r.action_id.unwrap()
    }
    pub fn console_input_from_str(&mut self, line : &str) -> Result<ActionID,String>
    {
        let chars = line.chars();

        if line.len() <= 2 
        { 
            return Ok(self.console_ai_best_move());
        }

        let mut it = chars.into_iter();

        let src =  self.console_input_at(&mut it).map_err(|e| e + " in move source")?;
        let dest =  self.console_input_at(&mut it).map_err(|e| e + " in move destination")?;

        Ok(ActionID::Move(src, dest))
    }

    /// panic if don't exist
    pub fn get_action_from_action_id(&self, action_id : ActionID) -> &Action
    {
        self.actions.iter().find(|e| e.id == action_id).unwrap()
    }

    pub fn execute(&mut self, action_id : ActionID)
    {
        self.time_line.drain(self.time_line_idx+1..);
        //let cur = self.current_mut();
        //let action_idx = cur.actions_and_result.iter().position(|e| e.action.id == action_id).expect("illegal input");
        
        let result = self.execute_action(self.get_action_from_action_id(action_id), true);

        self.time_line.push(result);
        self.time_line_idx += 1;
    }
}

impl BoardGame
{
    pub fn new_default() -> Self
    {
        // Self::new_chess_custom(4, true)
        // Self::new_chess_custom(2, true)
        //Self::new_checker()
        Self::new_chess()
        
        //Self::new_checker_custom_size(at(8, 8), 3)

        //Self::new_checker_custom_size(at(8, 8), 3)

        /* 
        let mut c = Self::new_checker_custom_size(at(8, 8), 3);
        let lines = include_str!("./contre_kt.txt").lines();
        for v in lines
        {
            let action_id = c.console_input_from_str(v).unwrap();
            c.execute(action_id);
        };
        c*/
    }

    pub fn new_chess() -> Self { Self::new_chess_custom(2, true, Relics::ZERO.with_flag_add(Relic::Anticipation)) }


    const CHESS_BACK_VALUE : [PieceFlags; 8] = [PieceFlags::AB_CHESS_ROOK, PieceFlags::AB_CHESS_KNIGHT, PieceFlags::AB_CHESS_BISHOP, PieceFlags::AB_CHESS_QUEEN, PieceFlags::AB_CHESS_KING.with_flag(PieceFlags::CROWN), PieceFlags::AB_CHESS_BISHOP, PieceFlags::AB_CHESS_KNIGHT, PieceFlags::AB_CHESS_ROOK];

    pub fn new_chess_custom(mut nb_player : usize, with_pawn : bool, relics : Relics) -> Self
    {
        //return Self::new_checker();
        //nb_player = 4;
        let margin = if nb_player > 2 { 3 } else { 0 };
        let s = At::splat(8 + 2 * margin);
        let mut board = BoardGameNotStarted::new_empty(s);
        custom_assert!(board.integrity_is_ok());

        for (idx, p) in Self::CHESS_BACK_VALUE.into_iter().enumerate()
        {
            let i = idx as AtIntType + margin;

            if nb_player >= 1
            {
                board.piece_add_team_and_set_flags(at(i, s.y-1), Team::Black, p);
                if with_pawn { board.piece_add_team_and_set_flags(at(i, s.y-2), Team::Black, PieceFlags::AB_CHESS_PAWN); }
            }

            if nb_player >= 2
            {
                if with_pawn { board.piece_add_team_and_set_flags(at(i, 1), Team::White, PieceFlags::AB_CHESS_PAWN); }
                board.piece_add_team_and_set_flags(at(i, 0), Team::White, p);
            }

            if nb_player >= 3
            {
                board.piece_add_team_and_set_flags(at(0, i), Team::Yellow, p);
                if with_pawn { board.piece_add_team_and_set_flags(at(1, i), Team::Yellow, PieceFlags::AB_CHESS_PAWN); }
            }

            if nb_player >= 4
            {
                board.piece_add_team_and_set_flags(at(s.x-1, i), Team::Green, p);
                if with_pawn { board.piece_add_team_and_set_flags(at(s.x-2, i), Team::Green, PieceFlags::AB_CHESS_PAWN); }
            }
        }
        for p in board.team_data.val.iter_mut()
        {
            p.relics = relics;
        }
        let mut b = BoardGame::new(board);
        b
    }

    pub fn new_checker() -> Self { Self::new_checker_custom_size(at(10, 10), 4) }
    pub fn new_checker_custom_size(s : At, nb_line_of_pawn : AtIntType) -> Self
    {
        let mut board = BoardGameNotStarted::new_empty(s);
        for y in 0..nb_line_of_pawn
        {
            for _x in 0..s.x/2
            {
                let x = 2 * _x;

                board.piece_add_team_and_set_flags(at(x + (y+1) % 2, s.y - 1 - y), Team::Black, PieceFlags::AB_DAME_PAWN);

                board.piece_add_team_and_set_flags(at(x + y % 2, y), Team::White, PieceFlags::AB_DAME_PAWN);
            }
        }
        let mut b = BoardGame::new(board);
        b
    }
}
