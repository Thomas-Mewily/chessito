use std::ops::DerefMut;

use crate::*;


pub trait ToSprite<G>
{
    fn to_sprite(self, ctx : &DefaultContext<G>) -> Sprite;
}
impl<G> ToSprite<G> for Sprite { fn to_sprite(self, _ctx : &DefaultContext<G>) -> Sprite { self }}

#[derive(Debug, PartialEq)]
pub struct Sprite
{
    pub texture : Texture2D,
    pub params  : DrawTexture,
}
impl Deref for Sprite
{
    type Target=DrawTexture;
    fn deref(&self) -> &Self::Target {
        &self.params
    }
}
impl DerefMut for Sprite
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.params
    }
}
impl Clone for Sprite
{
    fn clone(&self) -> Self {
        Self { texture: self.texture.weak_clone(), params : self.params.clone() }
    }
}
impl Sprite
{
    pub fn new(texture : &Texture2D) -> Self { Self::new_with_param(texture, ___()) }
    pub fn new_with_param(texture : &Texture2D, params : DrawTexture) -> Self
    {
        Self { texture: texture.weak_clone(), params }
    }

    pub fn size(&self) -> Vec2
    {
        match self.params.source
        {
            Some(src) => src.size,
            None => { self.texture.size() },
        }
    }
}
impl Sprite
{
    pub fn draw<G>(&self, px : Rect2, ctx : &mut DefaultContext<G>)
    {
        ctx.pen.texture(&self.texture, px.pos, px.size, Vec2::ZERO, self.params);
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct NineSlice<T = Sprite>
{
    //                           this column is
    //                             stretched
    //                                 |
    // 9 slice in action :            \|/
    //                                 V
    //*====================/**/==================/**/=====================**/
    /**/ pub top_left : T, /**/ pub top_mid : T, /**/ pub top_right : T, /**/
    //*====================/**/==================/**/=====================**/
    /**/ pub left     : T, /**/   pub mid : T,   /**/     pub right : T, /**   <-  This line is also stretched */  
    //*====================/**/==================/**/=====================**/
    /**/ pub bot_left : T, /**/ pub bot_mid : T, /**/ pub bot_right : T, /**/
    //*====================/**/==================/**/=====================**/
}
impl<T> NineSlice<T>
{
    pub fn new_full(top_left : T, top_mid : T, top_right : T, left : T, mid : T, right : T, bot_left : T, bot_mid : T, bot_right : T) -> Self
    {
        Self { top_left, top_mid, top_right, left, mid, right, bot_left, bot_mid, bot_right }
    }

    pub fn map<F>(&mut self, f : F) where F : Fn(&mut T)
    {
        f(&mut self.top_left); f(&mut self.top_mid); f(&mut self.top_right);
        f(&mut self.left    ); f(  &mut self.mid  ); f(  &mut self.right  );
        f(&mut self.bot_left); f(&mut self.bot_mid); f(&mut self.bot_right);
    }
}

impl NineSlice<Sprite>
{
    /// 1 pixel margin
    pub fn new(texture : &Texture2D, size_without_margin : Vec2, idx : Point2) -> Self
    {
        let margin = Vec2::ONE;
        let size_with_margin = size_without_margin + margin * 2.;
        let pos = idx.to_vec2() * size_with_margin;
        let pos_with_margin = pos + margin;
        Self::new_custom(texture, size_without_margin, pos_with_margin, margin)
    }

    pub fn new_custom(texture : &Texture2D, size_without_margin : Vec2, pos_with_margin : Vec2, margin : Vec2) -> Self
    {
        let size_with_margin = size_without_margin + margin * 2.;

        Self
        {
            top_left: Sprite::new_with_param(&texture, DrawTexture::source(Rect2::new(pos_with_margin + size_with_margin * vec2(0., 0.), size_without_margin))), 
            top_mid: Sprite::new_with_param(&texture, DrawTexture::source(Rect2::new(pos_with_margin + size_with_margin * vec2(1., 0.), size_without_margin))), 
            top_right: Sprite::new_with_param(&texture, DrawTexture::source(Rect2::new(pos_with_margin + size_with_margin * vec2(2., 0.), size_without_margin))), 
            left: Sprite::new_with_param(&texture, DrawTexture::source(Rect2::new(pos_with_margin + size_with_margin * vec2(0., 1.), size_without_margin))), 
            mid: Sprite::new_with_param(&texture, DrawTexture::source(Rect2::new(pos_with_margin + size_with_margin * vec2(1., 1.), size_without_margin))), 
            right: Sprite::new_with_param(&texture, DrawTexture::source(Rect2::new(pos_with_margin + size_with_margin * vec2(2., 1.), size_without_margin))), 
            bot_left: Sprite::new_with_param(&texture, DrawTexture::source(Rect2::new(pos_with_margin + size_with_margin * vec2(0., 2.), size_without_margin))), 
            bot_mid: Sprite::new_with_param(&texture, DrawTexture::source(Rect2::new(pos_with_margin + size_with_margin * vec2(1., 2.), size_without_margin))), 
            bot_right: Sprite::new_with_param(&texture, DrawTexture::source(Rect2::new(pos_with_margin+ size_with_margin * vec2(2., 2.), size_without_margin))), 
        }
    }
}