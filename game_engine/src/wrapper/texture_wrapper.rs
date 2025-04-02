use std::ops::DerefMut;

use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PureTexture2D
{
    pub val : macroquad::texture::Texture2D
}
impl PureTexture2D
{
    pub(crate) fn new(val : macroquad::texture::Texture2D) -> Self
    {
        Self { val }
    }

    pub fn weak_clone(&self) -> Self
    { Self { val: self.val.clone() } }

    pub fn size(&self) -> Vec2
    { self.val.size().to_engine() }

    pub fn width(&self) -> real
    { self.size().x }

    pub fn height(&self) -> real
    { self.size().y }

    pub fn set_filter(&mut self, filter_mode : Texture2DFilterMode) -> &mut Self
    { self.val.set_filter(filter_mode); self }

    pub fn to_image(&self) -> Image { self.to_image_with_meta(___()) }
    pub fn to_image_with_meta(&self, meta : TextureOrImageMeta) -> Image
    { Image::new_from_pure_texture(self, meta) }

    /// Anti Aliasing is on
    pub fn tag_add_aa(mut self) -> Self {
        self.set_filter(Texture2DFilterMode::Linear);
        self
    }

    /// Pixel Art : anti aliasing is off
    pub fn tag_add_pa(mut self) -> Self {
        self.set_filter(Texture2DFilterMode::Nearest);
        self
    }
}

pub type Texture2DFilterMode = macroquad::texture::FilterMode;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct TextureOrImageMeta
{
    /// the size of each tile in the texture
    px_size     : C2<u16>,

    /// margin for each axis of the tile
    px_margin : C2<u16>,

    credit : &'static str,
}
impl TextureOrImageMeta
{
    fn tile_with_margin_x(&self) -> usize
    { (self.px_size.x + 2 * self.px_margin.x) as usize }

    fn tile_with_margin_y(&self) -> usize
    { (self.px_size.y + 2 * self.px_margin.y) as usize }

    fn tile_with_margin(&self) -> Point2
    { Point2::new(self.tile_with_margin_x() as isize, self.tile_with_margin_y() as isize) }

    pub fn px_size_with_margin(&self) -> Point2 { self.tile_with_margin() }
    pub fn px_size(&self) -> Point2 { self.px_size.map(|e| e as int) }
    pub fn px_margin(&self) -> Point2 { self.px_margin.map(|e| e as int) }

    pub fn nb_tile(&self, size : Point2) -> Point2 { size / self.px_size_with_margin() }

    pub fn sheet_idx_to_point2(&self, idx : usize, size : Point2) -> Point2 
    {
        let nb = self.nb_tile(size).x;
        Point2::new(idx as isize % nb, idx as isize / nb)
    }

    pub fn sheet_point2_to_idx(&self, idx : Point2, size : Point2) -> usize 
    { (idx.y * size.y + idx.x) as usize }

    pub fn sheet_rect_from_idx(&self, idx : usize, size : Point2) -> Rect2 
    { self.sheet_rect_from_point2(self.sheet_idx_to_point2(idx, size)) }
    pub fn sheet_rect_from_point2(&self, idx : Point2) -> Rect2 
    {
        Rect2::new(self.px_margin.to_vec2() + (self.px_size.to_vec2() + self.px_margin.to_vec2() * 2.) * idx.to_vec2(), self.px_size.to_vec2())
    }

    pub fn tag_add_px(mut self, px_size : impl Into<Point2>) -> Self 
    {
        let v: Point2 = px_size.into();
        self.px_size = v.map(|e| e as u16);
        self
    }

    pub fn tag_add_margin(mut self, margin : impl Into<Point2>) -> Self 
    {
        let v: Point2 = margin.into();
        self.px_margin = v.map(|e| e as u16);
        self
    }

    pub fn tag_add_credit(&mut self, thank_to : &'static str) -> &mut Self 
    {
        self.credit = thank_to;
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Texture2D
{
    pub(crate) pure_texture : PureTexture2D,
    /// rect_size + margin
    pub(crate) meta : TextureOrImageMeta,
}
impl Deref for Texture2D
{
    type Target=PureTexture2D;
    fn deref(&self) -> &Self::Target { &self.pure_texture }
}
impl DerefMut for Texture2D
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.pure_texture }
}

impl GameAsset for Texture2D
{
    fn get_credit(&self) -> String { self.meta.credit.to_owned() }
}

impl Texture2D
{
    pub fn weak_clone(&self) -> Self
    {
        Self 
        { 
            pure_texture: self.pure_texture.weak_clone(),
            meta : self.meta,
        }
    }

    pub fn to_image(&self) -> Image { self.pure_texture.to_image_with_meta(self.meta) }

    /// Anti Aliasing is on
    pub fn tag_add_aa(mut self) -> Self 
    {
        self.pure_texture = self.pure_texture.tag_add_aa();
        self
    }

    /// Pixel Art : anti aliasing is off
    pub fn tag_add_pa(mut self) -> Self 
    {
        self.pure_texture = self.pure_texture.tag_add_pa();
        self
    }

    /// the size of each tile in the texture
    pub fn tag_add_px(mut self, val : impl Into<Point2>) -> Self {
        self.meta = self.meta.tag_add_px(val);
        self
    }

    /// the size of each tile in the texture
    pub fn tag_add_margin(mut self, val : impl Into<Point2>) -> Self {
        self.meta = self.meta.tag_add_margin(val);
        self
    }

    pub fn tag_add_credit(mut self, thank_to : &'static str) -> Self { self.meta.tag_add_credit(thank_to); self }

    pub fn sheet_rect_from_point2(&self, idx : Point2) -> Rect2 { self.meta.sheet_rect_from_point2(idx) }
    pub fn sheet_rect_from_idx(&self, idx : usize) -> Rect2  { self.meta.sheet_rect_from_idx(idx, self.size().to_point2()) }
    pub fn sheet_idx_to_point2(&self, idx : usize) -> Point2 { self.meta.sheet_idx_to_point2(idx, self.size().to_point2()) }
    pub fn sheet_point2_to_idx(&self, idx : Point2) -> usize { self.meta.sheet_point2_to_idx(idx, self.size().to_point2()) }
    pub fn nb_tile(&self) -> Point2 { self.meta.nb_tile(self.size().to_point2()) }

    pub fn sheet_px_size_with_margin(&self) -> Point2 { self.meta.px_size_with_margin() }
    pub fn sheet_px_size(&self) -> Point2 { self.meta.px_size() }
    pub fn sheet_px_margin(&self) -> Point2 { self.meta.px_margin() }

}

/// Copied from macroquad 
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DrawTexture 
{
    pub color : Color,

    /// Part of texture to draw. If None - draw the whole texture.
    /// Good use example: drawing an image from texture atlas.
    /// Is None by default
    pub source: Option<Rect2>,

    /// Rotation in radians
    pub angle: Angle,

    // Todo : missed opportunity to use C2<bool>

    /// Mirror on the `X / Y` axis
    pub flip : Bool2,

    /// Rotate around this point.
    /// When `None`, rotate around the texture's center.
    /// When `Some`, the coordinates are in screen-space.
    /// E.g. pivot (0,0) rotates around the top left corner of the screen, not of the
    /// texture.
    pub pivot: Option<Vec2>,
}
impl DrawTexture
{
    pub fn source(source : Rect2) -> Self 
    {
        Self::default().with_source(Some(source))
    }

    pub fn with_source(mut self, source : Option<Rect2>) -> Self { self.source = source; self }

    pub fn with_flip(mut self, flip : Bool2) -> Self { self.flip = flip; self }
    pub fn with_flip_x(mut self, flip_x : bool) -> Self { self.flip.x = flip_x; self }
    pub fn with_flip_y(mut self, flip_y : bool) -> Self { self.flip.y = flip_y; self }

    pub fn with_angle(mut self, angle : Angle) -> Self { self.angle = angle; self }

    pub fn with_pivot(mut self, pivot : Option<Vec2>) -> Self { self.pivot = pivot; self }
    pub fn with_color(mut self, color : Color) -> Self { self.color = color; self }
}