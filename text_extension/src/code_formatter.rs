use std::{fmt::{Display, Debug, Formatter, Result}, ops::{Deref, DerefMut}};

use math::map_on_scalar;
use util::IterEndPeek;

use crate::console_color::*;

#[derive(Clone, Default)]
pub struct CodeFormatterOption
{
    nb_open_parenthesis : usize,
    nb_open_accolade    : usize,
    nb_open_bracket     : usize,
    indent : usize,
}

pub struct CodeFormatter<'a,'b>
{
    f:  &'a mut Formatter<'b>,
    option : CodeFormatterOption,
}
impl<'a,'b> Deref for CodeFormatter<'a,'b> { type Target=CodeFormatterOption; fn deref(&self) -> &Self::Target { &self.option }}
impl<'a,'b> DerefMut for CodeFormatter<'a,'b> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.option }}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OpenCloseKind
{
    None,
    Brackets,
    Parenthesis,
    Accolade,
}

pub trait CodeDebug
{
    fn fmt_code_type_name(&self, _f : &mut CodeFormatter) -> Result { Ok(()) }

    fn open_close_kind(&self) -> OpenCloseKind { OpenCloseKind::Accolade }
    fn can_write_inline(&self) -> bool { false }
    fn fmt_code_value(&self, f : &mut CodeFormatter) -> Result;

    fn fmt_code(&self, f : &mut CodeFormatter) -> Result
    {
        let is_big = !self.can_write_inline();
        let open_close_kind = self.open_close_kind();

        self.fmt_code_type_name(f)?;


        if is_big
        { 
            f.push_ident();
            //f.new_line()?;
            //f.push_ident();
        }
        f.open(open_close_kind)?;

        self.fmt_code_value(f)?;
        if is_big
        { 
            //f.pop_ident();
            f.pop_ident_nl()?;
            //f.pop_ident_nl()?;
        }
        f.close(open_close_kind)?;

        Ok(())
    }

    fn fmt_code_from_debug(&self, f : &mut Formatter) -> Result { self.fmt_code(&mut CodeFormatter::new(f))}
}

macro_rules! impl_debug_code_for {
    ($name : ty) => {
        impl CodeDebug for $name 
        { 
            fn can_write_inline(&self) -> bool { true }
            fn fmt_code_value(&self, f : &mut CodeFormatter) -> Result { write!(f.f, "{}", self) }
            fn open_close_kind(&self) -> OpenCloseKind { OpenCloseKind::None }
        }
    };
}

map_on_scalar!(impl_debug_code_for);

impl_debug_code_for!(str);
impl_debug_code_for!(String);
impl_debug_code_for!(bool);

impl CodeDebug for () 
{ 
    fn can_write_inline(&self) -> bool { true }
    fn open_close_kind(&self) -> OpenCloseKind { OpenCloseKind::Parenthesis }
    fn fmt_code_value(&self, _ : &mut CodeFormatter) -> Result { Ok(()) }
}

impl<T : CodeDebug> CodeDebug for Option<T>
{
    fn open_close_kind(&self) -> OpenCloseKind { OpenCloseKind::None }
    fn can_write_inline(&self) -> bool { true }
    fn fmt_code_value(&self, f : &mut CodeFormatter) -> Result {
        match &self
        {
            Some(v) => { f.keyword("Some")?; f.open_parenthesis()?; f.write(v)?; f.close_parenthesis() },
            None => { f.keyword("None") },
        }
    }
}

impl<T : CodeDebug> CodeDebug for Vec<T>
{
    fn open_close_kind(&self) -> OpenCloseKind { OpenCloseKind::Brackets }
    fn can_write_inline(&self) -> bool 
    { 
        self.is_empty() || (self.len() <= 3 && self.first().unwrap().can_write_inline())
    }

    fn fmt_code_value(&self, f : &mut CodeFormatter) -> Result 
    {
        if !self.can_write_inline()
        {
            f.new_line()?;
        }
        let mut it = self.iter().peekable();
        while let Some(t) = it.next()
        {
            f.write(t)?;
            if it.is_not_last() { f.collection_sep()?; }
        }
        //if !self.can_write_inline() { f.new_line()?; }

        Ok(())
    }
}

impl<'a, 'b> CodeFormatter<'a, 'b>
{
    pub fn new(f : &'a mut Formatter<'b>) -> Self { Self { f, option: Default::default() }}

    pub fn debug<T : Debug>(&mut self, t : &T) -> Result { write!(self.f, "{:?}", t) } 
    pub fn debug_ln<T : Debug>(&mut self, t : &T) -> Result { self.debug(t)?; self.new_line() } 
    pub fn debug_colored<T : Debug, C : CodeDebug + ?Sized>(&mut self, t : &T, c : &C) -> Result 
    { 
        self.color(c)?;
        write!(self.f, "{:?}", t)?;
        self.color_reset()
    } 
    
    pub fn display<T : Display>(&mut self, t : &T) -> Result { write!(self.f, "{}", t) } 
    pub fn display_ln<T : Display>(&mut self, t : &T) -> Result { self.display(t)?; self.new_line() } 
    pub fn display_colored<T : Display, C : CodeDebug + ?Sized>(&mut self, t : &T, c : &C) -> Result 
    { 
        self.color(c)?;
        write!(self.f, "{}", t)?;
        self.color_reset()
    } 

    pub fn write<T : CodeDebug + ?Sized>(&mut self, t : &T) -> Result 
    { 
        t.fmt_code(self)
    }

    pub fn writeln<T : CodeDebug + ?Sized>(&mut self, t : &T) -> Result { self.write(t)?; self.new_line() }
    pub fn new_line(&mut self) -> Result 
    { 
        writeln!(self.f)?;
        for _ in 0..self.indent
        {
            self.write_colored("|", GREY_FOREGROUND)?;
            self.space()?;
            self.space()?;
        }
        Ok(())
    }

    pub fn write_colored<T : CodeDebug + ?Sized, C : CodeDebug + ?Sized>(&mut self, t : &T, c : &C) -> Result 
    { 
        self.color(c)?;
        self.write(t)?;
        self.color_reset()?;
        Ok(())
    }
    pub fn writeln_colored<T : CodeDebug + ?Sized, C : CodeDebug + ?Sized>(&mut self, t : &T, c : &C) -> Result 
    { 
        self.write_colored(t, c)?;
        self.new_line()?;
        Ok(())
    }

    pub fn keyword(&mut self, keyword : &str) -> Result { self.write_colored(keyword, MAGENTA_FOREGROUND) }
    pub fn error(&mut self, error : &str) -> Result { self.write_colored(error, RED_FOREGROUND) }

    pub fn punctuation(&mut self, keyword : &str) -> Result { self.write_colored(keyword, GREY_FOREGROUND) }

    pub fn collection_sep(&mut self) -> Result { self.punctuation(",")?; self.space() }
    pub fn instruction_end(&mut self) -> Result { self.punctuation(";") }

    pub fn type_name(&mut self, keyword : &str) -> Result { self.write_colored(keyword, CYAN_FOREGROUND) }
    pub fn field(&mut self, field : &str) -> Result { self.write_colored(field, GREEN_FOREGROUND)?; self.field_value_sep() }
    pub fn field_value_sep(&mut self) -> Result { self.space()?; self.punctuation(":")?; self.space() }
    pub fn field_value_end(&mut self) -> Result { self.collection_sep() }
    pub fn number<T : CodeDebug>(&mut self, value : T) -> Result { self.write_colored(&value, WHITE_FOREGROUND) }
    
    pub fn field_value<T : CodeDebug>(&mut self, field : &str, value : &T) -> Result 
    { 
        self.new_line()?;
        self.field(field)?;
        if !value.can_write_inline() { self.new_line()?; }
        self.write(value)?;
        Ok(())
    }
    pub fn field_debug_value<T : Debug>(&mut self, field : &str, value : &T) -> Result 
    {
        self.new_line()?;
        self.field(field)?;
        self.debug(value)?;
        Ok(())
    }


    pub fn color<C : CodeDebug + ?Sized>(&mut self, c : &C) -> Result 
    { 
        self.write(c)
    }
    pub fn color_reset(&mut self) -> Result { self.color(COLOR_RESET) }

    pub fn space(&mut self) -> Result { self.write(" ") }

    const PARENTHESIS_ZEBRA_COLORING : &'static [&'static str] = &[YELLOW_FOREGROUND, GREEN_FOREGROUND, CYAN_FOREGROUND, MAGENTA_FOREGROUND];
    fn apply_parenthesis_color(&mut self) -> Result { self.color(Self::PARENTHESIS_ZEBRA_COLORING[self.nb_open_parenthesis % Self::PARENTHESIS_ZEBRA_COLORING.len()]) }
    pub fn open_parenthesis(&mut self) -> Result { self.apply_parenthesis_color()?; self.write("(")?; self.color_reset()?; self.nb_open_parenthesis+=1; Ok(()) }
    pub fn close_parenthesis(&mut self) -> Result { self.nb_open_parenthesis-=1;self.apply_parenthesis_color()?; self.write(")")?; self.color_reset() }
    pub fn open_close_parenthesis(&mut self) -> Result { self.open_close(OpenCloseKind::Parenthesis) }

    // I used the french word "accolade", it just sound better than "curly brackets"
    const ACCOLADE_ZEBRA_COLORING : &'static [&'static str] = &[YELLOW_FOREGROUND, GREEN_FOREGROUND, CYAN_FOREGROUND, MAGENTA_FOREGROUND];
    fn apply_accolade_color(&mut self) -> Result { self.color(Self::ACCOLADE_ZEBRA_COLORING[self.nb_open_accolade % Self::ACCOLADE_ZEBRA_COLORING.len()]) }
    pub fn open_accolade(&mut self) -> Result { self.apply_accolade_color()?; self.write("{")?; self.color_reset()?; self.nb_open_accolade+=1; Ok(()) }
    pub fn close_accolade(&mut self) -> Result { self.nb_open_accolade-=1;self.apply_accolade_color()?; self.write("}")?; self.color_reset() }
    pub fn open_close_accolade(&mut self) -> Result { self.open_close(OpenCloseKind::Accolade) }
    
    pub fn ident_nl_open_accolade (&mut self) -> Result { self.ident_nl_open (OpenCloseKind::Accolade) }
    pub fn ident_nl_close_accolade(&mut self) -> Result { self.ident_nl_close(OpenCloseKind::Accolade) }

    pub fn push_ident_nl (&mut self) -> Result { self.push_ident(); self.new_line()  }
    pub fn pop_ident_nl(&mut self) -> Result { self.pop_ident(); self.new_line() }


    const BRACKET_ZEBRA_COLORING : &'static [&'static str] = &[YELLOW_FOREGROUND, GREEN_FOREGROUND, CYAN_FOREGROUND, MAGENTA_FOREGROUND];
    fn apply_bracket_color(&mut self) -> Result { self.color(Self::BRACKET_ZEBRA_COLORING[self.nb_open_bracket % Self::BRACKET_ZEBRA_COLORING.len()]) }
    pub fn open_bracket(&mut self) -> Result { self.apply_bracket_color()?; self.write("[")?; self.color_reset()?; self.nb_open_bracket+=1; Ok(()) }
    pub fn close_bracket(&mut self) -> Result { self.nb_open_bracket-=1;self.apply_bracket_color()?; self.write("]")?; self.color_reset() }
    pub fn open_close_bracket(&mut self) -> Result { self.open_close(OpenCloseKind::Brackets) }

    pub fn push_ident(&mut self) { self.indent += 1; }
    pub fn pop_ident (&mut self) { self.indent -= 1; }


    pub fn ident_nl_open (&mut self, kind : OpenCloseKind) -> Result { self.open(kind)?; self.push_ident_nl()  }
    pub fn ident_nl_close(&mut self, kind : OpenCloseKind) -> Result { self.pop_ident_nl()?; self.close(kind) }

    pub fn open_close(&mut self, kind : OpenCloseKind) -> Result { self.open(kind)?; self.close(kind) }

    pub fn open(&mut self, kind : OpenCloseKind) -> Result
    {
        match kind
        {
            OpenCloseKind::None => Ok(()),
            OpenCloseKind::Brackets => self.open_bracket(),
            OpenCloseKind::Parenthesis => self.open_parenthesis(),
            OpenCloseKind::Accolade => self.open_accolade(),
        }
    }

    pub fn close(&mut self, kind : OpenCloseKind) -> Result
    {
        match kind
        {
            OpenCloseKind::None => Ok(()),
            OpenCloseKind::Brackets => self.close_bracket(),
            OpenCloseKind::Parenthesis => self.close_parenthesis(),
            OpenCloseKind::Accolade => self.close_accolade(),
        }
    }
}