use std::{borrow::Borrow, default, fmt::Debug, ops::{Deref, DerefMut}};
use macroquad::{math::Vec2, prelude, window::screen_width};
use crate::*;

pub type Error = macroquad::Error;

pub trait GameAsset
{
    
    fn get_credits(&self, credits : &mut Vec<String>) 
    { 
        let c = self.get_credit();
        if !c.is_empty() { credits.push(c) } else { credits.push("???".to_owned()); }
    }
    fn get_credit(&self) -> String;
}

#[derive(Default, Debug)]
pub struct ContextAssetManager 
{
    cached_char : Vec<char>,
}

impl ContextAssetManager
{
    pub const FONT_DEFAULT_SIZE : FontSize = 96;

    pub fn new() -> Self 
    { 
        let nb_char = 127;
        let mut cached_char = Vec::with_capacity(nb_char);

        for i in 0..=nb_char
        { 
            if let Some(c) = std::char::from_u32(i as u32) 
            {
                cached_char.push(c);
            }
        }

        Self { cached_char }
    }

    pub async fn load_pure_texture2d(&mut self, path : &str) -> PureTexture2D { self.try_load_pure_texture2d(path).await.unwrap() }
    pub async fn try_load_pure_texture2d(&mut self, path : &str) -> Result<PureTexture2D, Error> 
    { 
        macroquad::prelude::load_texture(path).await.map(|val| PureTexture2D { val })
    }

    pub async fn load_texture2d(&mut self, path : &str) -> Texture2D { self.try_load_texture2d(path).await.unwrap() }
    pub async fn try_load_texture2d(&mut self, path : &str) -> Result<Texture2D, Error> 
    { 
        self.try_load_pure_texture2d(path).await.map(|texture| Texture2D { pure_texture: texture, meta : ___()})
    }



    pub async fn load_font(&mut self, path : &str) -> Font { self.try_load_font(path).await.unwrap() }
    pub async fn try_load_font(&mut self, path : &str) -> Result<Font, Error> 
    { 
        let t = macroquad::prelude::load_ttf_font(path).await;
        let font = t.map(|r| 
            { 
                let mut f = Font { mq_font: r, credit : "???" }; 
                f.populate_font_cache(self.cached_char.as_slice(), Self::FONT_DEFAULT_SIZE);
                f
            }
        );
        font
    }

    pub async fn load_sound(&mut self, path : &str) -> Sfx { self.try_load_sound(path).await.unwrap() }
    pub async fn try_load_sound(&mut self, path : &str) -> Result<Sfx, Error> { self.try_load_audio(path).await.map(Sfx::sfx) }

    pub async fn load_audio(&mut self, path : &str) -> Audio { self.try_load_audio(path).await.unwrap() }
    pub async fn try_load_audio(&mut self, path : &str) -> Result<Audio, Error> { macroquad::audio::load_sound(path).await }
}
