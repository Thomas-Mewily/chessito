use std::fmt;

use crate::*;

/// A TTF font on the GPU
#[derive(Clone)]
pub struct Font
{
    pub(crate) mq_font : macroquad::text::Font,

    pub credit : &'static str,
}
impl fmt::Debug for Font
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "Font") }
}
impl PartialEq for Font
{
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl GameAsset for Font
{
    fn get_credit(&self) -> String { self.credit.to_owned() }
}

pub type FontSize = u16;
pub type FontFilter = macroquad::texture::FilterMode;
impl Font
{
    pub fn tag_add_credit(mut self, thank_to : &'static str) -> Self 
    { 
        self.credit = thank_to;
        self
    }

    /// Anti Aliasing is on
    pub fn tag_add_aa(mut self) -> Self 
    {
        self.set_filter(FontFilter::Linear);
        self
    }

    /// Pixel Art : anti aliasing is off
    pub fn tag_add_pa(mut self) -> Self 
    {
        self.set_filter(FontFilter::Nearest);
        self
    }

    pub fn populate_font_cache(&mut self, characters: &[char], font_size: FontSize) 
    { self.mq_font.populate_font_cache(characters, font_size as u16); }

    pub fn set_filter(&mut self, filter_mode: FontFilter) 
    { self.mq_font.set_filter(filter_mode); }
}
