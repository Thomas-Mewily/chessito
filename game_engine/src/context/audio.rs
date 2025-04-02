use std::{default, fmt::{format, Debug}, mem, ops::DerefMut};

use macroquad::{audio::{AudioContext, PlaySoundParams}, prelude};
use crate::*;

#[derive(Debug)]
pub struct ContextAudio
{
    music_coef : Coef,
    sfx_coef : Coef,
}
impl Default for ContextAudio
{
    fn default() -> Self 
    {
        Self { music_coef: 1., sfx_coef: 1. }
    }
}

impl ContextAudio
{
    pub fn music_coef(&self) -> Coef { self.music_coef }
    pub fn set_music_coef(&mut self, music_coef : Coef) -> &mut Self { self.music_coef = music_coef; self }

    pub fn sfx_coef(&self) -> Coef { self.sfx_coef }
    pub fn set_sfx_coef(&mut self, sfx_coef : Coef) -> &mut Self { self.sfx_coef = sfx_coef; self }

    
    pub fn stop(&mut self, sound: &Sound) { macroquad::audio::stop_sound(&sound.audio); }
    pub fn set_volume(&mut self, sound: &Sound, volume: real) { macroquad::audio::set_sound_volume(&sound.audio, volume as f32) }

    pub fn play(&mut self, sound : &Sound) { self.play_with_params(sound, SoundParams::new()); }
    pub fn play_with_volume(&mut self, sound : &Sound, volume : real) { self.play_with_params(sound, SoundParams::new().with_volume(volume)); }
    pub fn play_with_params(&mut self, sound : &Sound, params : SoundParams)
    {
        // Todo : can change the volume if the audio is a sound effect or the music
        macroquad::audio::play_sound(&sound.audio, PlaySoundParams
            {
                looped: params.looped,
                volume: 
                (
                    params.volume * sound.volume_coef * match sound.kind
                    {
                        SoundKind::Sfx   => self.sfx_coef,
                        SoundKind::Music => self.music_coef,
                    }
                ) as f32,
            }
        );
    }
}

pub trait SoundExtension
{
    fn play(&self, audio : &mut ContextAudio);
    fn play_with_volume(&self, volume : real, audio : &mut ContextAudio);
    fn play_with_params(&self, params : SoundParams, audio : &mut ContextAudio);
}

impl SoundExtension for Sound
{
    fn play(&self, audio : &mut ContextAudio) 
    { audio.play(self) }
    
    fn play_with_volume(&self, volume : real, audio : &mut ContextAudio) 
    { audio.play_with_volume(self, volume) }
    
    fn play_with_params(&self, params : SoundParams, audio : &mut ContextAudio)
    { audio.play_with_params(self, params) }
}