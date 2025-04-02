use std::ops::DerefMut;

use crate::*;

#[derive(Clone)]
pub struct Image
{
    img : macroquad::texture::Image,
    meta : TextureOrImageMeta,
}
impl Image
{
    pub fn new_from_texture(texture : &Texture2D) -> Image { Self::new_from_pure_texture(&texture.pure_texture, texture.meta) }
    pub fn new_from_pure_texture(texture : &PureTexture2D, meta : TextureOrImageMeta) -> Image
    {
        Image { img: texture.val.get_texture_data(), meta }
    }

    pub fn new_transparent(size : Point2) -> Image  { Self::new_with_chanel(size, u8::MIN) }
    pub fn new_white(size : Point2) -> Image  { Self::new_with_chanel(size, u8::MAX) }
    pub fn new_with_chanel(size : Point2, all_chanel : u8) -> Image
    {
        Self
        {
            img : macroquad::texture::Image
            {
                bytes: vec![all_chanel; (size.area() * 4) as usize],
                width: size.x as u16,
                height: size.y as u16,
            },
            meta : ___(),
        }
    }

    pub fn size  (&self) -> Point2 { point2(self.img.width as int, self.img.height as int)}
    pub fn width (&self) -> isize  { self.size().x }
    pub fn height(&self) -> isize  { self.size().y }

    pub fn sub_image(&self, rect : Rect2I) -> Image
    {
        Image
        {
            meta : self.meta,
            img : self.img.sub_image(Rect2::new(rect.pos.to_vec2(), rect.size.to_vec2()).to_lib())
        }
    }

    pub fn get_color(&self, pos : Point2) -> Color
    {
        self.img.get_pixel(pos.x as u32, pos.y as u32).to_engine()
    }

    pub fn get_pixel(&self, pos : Point2) -> Pixel
    {
        self.img.get_image_data()[(pos.y * self.width() + pos.x) as usize].into()
    }

    pub fn set_color(&mut self, pos : Point2, color : Color) -> &mut Self
    {
        self.img.set_pixel(pos.x as u32, pos.y as u32, color.to_lib());
        self
    }

    pub fn save_as_png(&self, path : &str)
    {
        self.img.export_png(path);
    }

    pub fn to_texture2d(&self) -> Texture2D
    {
        Texture2D
        {
            pure_texture: PureTexture2D::new(macroquad::texture::Texture2D::from_image(&self.img)),
            meta: self.meta,
        }
    }
}
