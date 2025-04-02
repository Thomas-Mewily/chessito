use std::{default, fmt::{format, Debug}, mem, ops::DerefMut};

use macroquad::{audio::{AudioContext, PlaySoundParams}, prelude};
use crate::*;

pub type Music = Sound;
/// Sound Effect
pub type Sfx = Sound;

pub type Audio = macroquad::audio::Sound;

#[derive(Clone)]
pub struct Sound
{
    pub(crate) audio : Audio,
    pub(crate) kind  : SoundKind,
    pub(crate) volume_coef : real,

    pub(crate) credit : &'static str,
}

impl PartialEq for Sound
{
    // Todo : wait for macroquad to implement equality for audio...
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind // && self.audio == other.audio && 
    }
}

impl GameAsset for Sound
{
    fn get_credit(&self) -> String { self.credit.to_owned() }
}

impl Debug for Sound
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{{sound ???, {:?}}}", self.kind) }
}
impl Sound
{
    pub fn new(audio : Audio, kind  : SoundKind) -> Self { Self { audio, kind, volume_coef : 1., credit: "???" }}

    pub fn sfx(audio : Audio) -> Self { Self::new(audio, SoundKind::Sfx) }
    pub fn music(audio : Audio) -> Self { Self::new(audio, SoundKind::Music) }

    pub fn is_music(&self) -> bool { matches!(self.kind, SoundKind::Music) }
    pub fn is_sfx(&self) -> bool { matches!(self.kind, SoundKind::Sfx) }

    pub fn kind(&mut self) -> SoundKind { self.kind }
    pub fn set_kind(&mut self, kind : SoundKind) -> &mut Self
    {
        self.kind = kind;
        self
    }

    pub fn volume_coef(&self) -> real { self.volume_coef }
    pub fn set_volume_coef(&mut self, volume_coef : real) -> &mut Self { self.volume_coef = volume_coef; self }


    pub fn tag_add_sfx(mut self) -> Self {
        self.set_kind(SoundKind::Sfx);
        self
    }

    pub fn tag_add_music(mut self) -> Self {
        self.set_kind(SoundKind::Music);
        self
    }

    pub fn tag_add_credit(mut self, thank_to : &'static str) -> Self { self.credit = thank_to; self }

    pub fn tag_add_volume(mut self, percentage : u16) -> Self 
    { 
        self.volume_coef = percentage.to_real() / 100.;
        self 
    }

}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SoundKind
{
    Sfx,
    Music,
}

/// Based on MacroQuad PlaySoundParams
#[derive(Clone, Copy, PartialEq)]
pub struct SoundParams 
{
    pub looped : bool,
    pub volume : real,
}
impl Default for SoundParams
{
    fn default() -> Self { Self { looped: false, volume: 1. } }
}
impl SoundParams
{
    pub fn new() -> Self { ___() }

    //pub fn sfx() -> Self { Self::___() }
    /// Looped by default
    //pub fn music() -> Self { Self::___().and_loop_it() }

    pub fn and_loop_it(self) -> Self { self.with_loop(true) }
    pub fn with_loop(mut self, looped : bool) -> Self { self.looped = looped; self }
    pub fn with_volume(mut self, volume : real) -> Self { self.volume = volume; self }
}

