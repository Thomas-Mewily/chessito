use std::{default, fmt::{Debug, Display, Formatter, Result}, ops::*};
use crate::*;

pub trait TimeExtension
{
    fn ms(self) -> Time;
    fn s(self) -> Time;
    fn mins(self) -> Time;
}
impl<T:ToReal> TimeExtension for T
{
    fn ms(self) -> Time { Time::from_ms(self.to_real()) }
    fn s(self) -> Time { Time::from_s(self.to_real()) }
    fn mins(self) -> Time { Time::from_min(self.to_real()) }
}

pub type Tick = Time<int>;

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct Time<T : Unit + DefaultDeltaTime=real> { _tick : T }

impl<T : Unit + DefaultDeltaTime> Time<T>
{
    pub const fn _from_ticks(ticks : T) -> Self { Self { _tick: ticks } }
    pub const fn _ticks(self) -> T { self._tick }
}

impl<T : Unit + DefaultDeltaTime> Unit for Time<T> {}
impl<T : Unit + DefaultDeltaTime> NumberCompare for Time<T> {}
impl<T : Unit + DefaultDeltaTime> PartialOrdComparison for Time<T> {}

impl<T : Unit + DefaultDeltaTime> UnitArithmetic<Self> for Time<T> {}
impl<T : Unit + DefaultDeltaTime> HaveZero for Time<T> { const ZERO : Self = Self::_from_ticks(T::ZERO); }
impl<T : Unit + DefaultDeltaTime> HaveOne  for Time<T> { const ONE  : Self = Self::_from_ticks(T::ONE ); }

impl<T : Unit + DefaultDeltaTime> Absolute for Time<T> { fn absolute(self) -> Self { Self::_from_ticks(self._tick.absolute()) }}
impl<T : Unit + DefaultDeltaTime> SmallestIncrement for Time<T>  { const SMALL_INC  : Self = Self::_from_ticks(T::SMALL_INC); }


impl<T : FloatingNumber + DefaultDeltaTime> Time<T>
{
    pub fn from_min(ms : T) -> Self { Self { _tick: ms } }
    pub fn from_ms (ms : T) -> Self { Self { _tick: ms } }
    pub fn from_s  (s  : T) -> Self { Self { _tick: s * T::from_usize(1000) } }

    /// minutes
    pub fn mins(self) -> T { self._tick / T::from_usize(60 * 1000) }
    pub fn whole_mins(self) -> uint { self.mins().to_uint() }

    /// milli seconds
    pub fn ms(self) -> T { self._tick }
    pub fn whole_ms(self) -> uint { self.ms().to_uint() }

    /// seconds
    pub fn s(self) -> T { self._tick / T::from_usize(1000) }
    pub fn whole_s(self) -> uint { self.s().to_uint() }
}

impl<T : Integer + DefaultDeltaTime> Time<T>
{
    pub fn from_turn(turn : T) -> Self { Self::_from_ticks(turn) }
    pub fn turn(self) -> T { self._tick }
}



impl<T : Unit + DefaultDeltaTime> AddAssign<Time<T>> for Time<T> { fn add_assign(&mut self, rhs: Time<T>) { self._tick.add_assign(rhs._tick) }}
impl<T : Unit + DefaultDeltaTime> Add<Time<T>> for Time<T>
{
    type Output=Time<T>;
    fn add(self, rhs: Time<T>) -> Self::Output {
        Self::_from_ticks(self._tick + rhs._tick)
    }
}

impl<T : Unit + DefaultDeltaTime> SubAssign<Time<T>> for Time<T> { fn sub_assign(&mut self, rhs: Time<T>) { self._tick.sub_assign(rhs._tick) }}
impl<T : Unit + DefaultDeltaTime> Sub<Time<T>> for Time<T>
{
    type Output=Time<T>;
    fn sub(self, rhs: Time<T>) -> Self::Output {
        Self::_from_ticks(self._tick - rhs._tick)
    }
}

impl<T : Unit + DefaultDeltaTime> Div<Time<T>> for Time<T> where T : Div<T,Output = T>
{
    type Output=T;
    fn div(self, rhs: Time<T>) -> Self::Output {
        self._tick / rhs._tick
    }
}

impl<T : Unit + DefaultDeltaTime + Number> Mul<T> for Time<T>
{
    type Output=Time<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Time::_from_ticks(self._tick * rhs)
    }
}

impl<T : Unit + DefaultDeltaTime + Number> Rem<T> for Time<T>
{
    type Output=Time<T>;
    fn rem(self, rhs: T) -> Self::Output {
        Self::_from_ticks(self._tick % rhs)
    }
}









/// Define a definition for a smallest increment for a type. `1` for integer, 0.0 for float.
pub trait DefaultDeltaTime
{ 
    const DEFAULT_DELTA_INV  : Self; 
    fn default_delta_time() -> Self;
}
macro_rules! impl_default_delta_int {
    ($primitive_name: ty) => 
    { 
        impl DefaultDeltaTime for $primitive_name 
        { 
            const DEFAULT_DELTA_INV : Self = Self::ONE;
            fn default_delta_time() -> Self { Self::ONE }
        }
    };
}
map_on_integer!(impl_default_delta_int);
macro_rules! impl_default_delta_float {
    ($primitive_name: ty) => 
    { 
        impl DefaultDeltaTime for $primitive_name 
        { 
            const DEFAULT_DELTA_INV : Self = 60.;
            fn default_delta_time() -> Self { 1000. / Self::DEFAULT_DELTA_INV }
        }
    };
}
map_on_non_zero_floating!(impl_default_delta_float);
impl DefaultDeltaTime for f0 { const DEFAULT_DELTA_INV  : Self = Self; fn default_delta_time() -> Self { Self } }

impl<T : Unit + DefaultDeltaTime> DefaultDeltaTime for Time<T> 
{ 
    const DEFAULT_DELTA_INV  : Self = Self::_from_ticks(T::DEFAULT_DELTA_INV); 
    fn default_delta_time() -> Self { Self::_from_ticks(T::default_delta_time()) }
}


/* 
impl<S : ToReal> MulAssign<S> for TimeClock { fn mul_assign(&mut self, rhs: S) { self._tick.mul_assign(rhs.to_real()) }}
impl<S : ToReal> Mul<S> for TimeClock
{
    type Output=TimeClock;
    fn mul(self, rhs: S) -> Self::Output {
        Self::from_ms(self._tick * rhs.to_real())
    }
}

impl<S : ToReal> Div<S> for TimeClock
{
    type Output=TimeClock;
    fn div(self, rhs: S) -> Self::Output {
        Self::from_ms(self._tick / rhs.to_real())
    }
}



impl Debug for TimeClock
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{:.3?} ms", self.ms()) }
}

impl TimeClock
{
    pub fn fmt_total(self, precision : Option<PrecisionSnap>, postfix : &'static str) -> DisplayGameTime
    { DisplayGameTime { time:self, precision, postfix }}
    
    pub fn fmt_ms(self) -> DisplayGameTime { self.fmt_total( None, " ms") }
}
impl Display for TimeClock { fn fmt(&self, f: &mut Formatter<'_>) -> Result { self.fmt_ms().fmt(f) }}

#[derive(Clone)]
pub struct PrecisionSnap
{
    div : real,
    mul : real,
}

#[derive(Clone)]
pub struct DisplayGameTime{ time : TimeClock, precision : Option<PrecisionSnap>, postfix : &'static str }

impl Display for DisplayGameTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result 
    {
        write!(f, "{}{}",
            match &self.precision {
                Some(p) => (self.time.ms() / p.div).round()*p.mul,
                None => self.time.ms(),
            }, 
        self.postfix)
    }
}*/