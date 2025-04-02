use std::{fmt::{format, Debug}, mem, ops::DerefMut};

use macroquad::prelude;
use crate::*;

pub struct ContextPen
{
    total_draw_rect     : usize,
    total_draw_triangle : usize,

    total_draw_texture  : usize,
    total_draw_text_len : usize,

    text_debug : Vec<String>,

    debug_self : bool,

    background_color : Color,

    white_pixel : Texture2D,

    pub cam : ContextCamera,

    pub fonts : Vec<Font>,

    //cams : Vec<Camera2>,
}

impl Default for ContextPen
{
    fn default() -> Self {
        Self 
        { 
            total_draw_rect: Default::default(),
            total_draw_triangle: Default::default(),
            total_draw_texture: Default::default(),
            total_draw_text_len: Default::default(),
            text_debug: Default::default(),
            debug_self: Default::default(),
            background_color: Default::default(),
            white_pixel: Image::new_white(point2(1, 1)).to_texture2d(),
            cam: Default::default(),
            fonts: Default::default() 
        }
    }
}
// impl Deref for Pen { type Target=CameraManager; fn deref(&self) -> &Self::Target { &self.cam }}
// impl DerefMut for Pen { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.cam }}

impl Debug for ContextPen
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "Pen: {} texture, {} rect, {} triangles, {} txt len", self.total_draw_texture, self.total_draw_rect, self.total_draw_triangle, self.total_draw_text_len) }
}

impl ContextPen
{
    pub fn push_font(&mut self, f : &Font)
    {
        self.fonts.push(f.clone());
    }

    pub fn pop_font(&mut self) { self.fonts.pop().expect("can't pop the empty font stack"); }
    pub fn font(&self) -> Option<&Font> { self.fonts.last() }
}

impl<Glob> ContextEvent<Glob> for ContextPen
{
    fn input_begin(ctx : &mut DefaultContext<Glob>) { ctx.pen.text_debug.clear(); ContextCamera::input_begin(ctx); }
    fn input_end  (ctx : &mut DefaultContext<Glob>) { ContextCamera::input_end(ctx);   }

    fn update_begin(ctx : &mut DefaultContext<Glob>) { ContextCamera::update_begin(ctx); }
    fn update_end  (ctx : &mut DefaultContext<Glob>) { ContextCamera::update_end(ctx); }

    fn draw_begin(ctx : &mut DefaultContext<Glob>)
    {
        ctx.pen.total_draw_rect     = 0;
        ctx.pen.total_draw_texture  = 0;
        ctx.pen.total_draw_triangle = 0;
        ctx.pen.total_draw_text_len = 0;
        ctx.pen.debug_self = false;
        ctx.pen.clear_background(ctx.pen.background_color);

        ContextCamera::draw_begin(ctx);
        ctx.pen.cam.push();
    }
    fn draw_end(ctx : &mut DefaultContext<Glob>) 
    { 
        ctx.pen.cam.pop();

        let mut text_debug_height = Self::print_all_debug_txt(ctx);
        if ctx.pen.debug_self
        {
            ctx.pen.print_debug_txt(&format!("{:?}", ctx.pen), &mut text_debug_height);
        }

        assert!(ctx.pen.fonts.is_empty(), "pen : forgot to pop a font");
        ContextCamera::draw_end(ctx);
    }
}

impl ContextPen
{
    pub fn clear_background(&mut self, c : Color) 
    {
        macroquad::prelude::clear_background(c.to_lib());
    }

    pub fn set_window_background_color(&mut self, window_background_color : Color) { self.background_color = window_background_color; }
    /// fill the current backgound
    pub fn fill_world_background(&mut self, background_color : Color) 
    { 
        let c = self.cam.current();
        self.rectangle(c.world.pos, c.world.size, VEC2_ZERO, zero(), background_color);
    }
    
    //pub fn step(&mut self, step : Vec2) { self.transform.pos += step; }

    pub fn rectangle
    (
        &mut self, 
        pos : Vec2,
        size : Vec2,
        center_coef: Vec2,
        angle : Angle,
        color : Color
    )
    {
        self.total_draw_rect += 1;
        //self._texture(&self.white_pixel, pos, size, center_coef, DrawTexture::___().with_angle(angle).with_color(color));
        prelude::draw_rectangle_ex(pos.x as f32, pos.y as f32, size.x as f32, size.y as f32, macroquad::shapes::DrawRectangleParams{ offset: center_coef.to_lib(), rotation: angle.radian() as f32, color: color.to_lib() })
    }


    pub fn straight_line(&mut self, src : impl Into<Vec2>, dest : impl Into<Vec2>, tickness : real, color : Color)
    {
        let src  : Vec2 = src.into();  
        let dest : Vec2 = dest.into();

        let direction = dest-src;

        //self.rectangle(src, vec2(tickness, direction.length()), vec2(0.5, 0.), direction.angle(), color);
        self.rectangle(src, vec2(tickness, direction.length()), vec2(0.5, 0.), direction.angle() - Angle::RIGHT, color);
    } 
}

impl ContextPen
{
    pub fn texture 
    (
        &mut self, 
        texture : &Texture2D,
        pos : Vec2,
        size : Vec2,
        center_coef : Vec2,
        params : DrawTexture
    )
    {
        self.total_draw_texture += 1;
        self._texture(texture, pos, size, center_coef, params);
    }

    fn _texture 
    (
        &self, 
        texture : &Texture2D,
        mut pos : Vec2,
        size : Vec2,
        mut center_coef : Vec2,
        params : DrawTexture
    )
    {
        if params.flip.x { center_coef.x = 1. - center_coef.x; }
        if params.flip.y { center_coef.y = 1. - center_coef.y; }

        pos -= center_coef * size;

        prelude::draw_texture_ex(&texture.pure_texture.val, pos.x as f32, pos.y as f32, params.color.to_lib(),macroquad::texture::DrawTextureParams 
        { 
            dest_size: Some(size.to_lib()),
            source: params.source.map(|r| r.to_lib()),
            rotation: params.angle.radian() as f32, 
            flip_x:  params.flip.x,
            flip_y: !params.flip.y,
            pivot: params.pivot.map(|p| p.to_lib()),
        })
    }
}


impl ContextPen
{
    pub fn triangle(&mut self, p1 : impl Into<Vec2>, p2 : impl Into<Vec2>, p3 : impl Into<Vec2>, color : Color)
    { 
        self.total_draw_triangle += 1;
        prelude::draw_triangle(Into::<Vec2>::into(p1).to_lib(), Into::<Vec2>::into(p2).to_lib(), Into::<Vec2>::into(p3).to_lib(), color.to_lib())
    }

    pub fn circle(&mut self, pos : impl Into<Vec2>, radius : real, color : Color) { self.ellipse(pos, Vec2::splat(radius), ANGLE_ZERO, color) }

    pub fn ellipse(&mut self, pos : impl Into<Vec2>, radius : impl Into<Vec2>, angle: Angle, color: Color) 
    {
        let mut nb_triangle = 48;

        nb_triangle = nb_triangle.max(3);

        let mut rotation = angle;
        let rotation_inc = ANGLE_FULL / nb_triangle;

        let center = pos.into();
        let r      = radius.into();

        let mut rotation_point = center + rotation.to_vec2_normalized() * r;


        for _ in 0..nb_triangle
        {
            rotation += rotation_inc;
            let old_angle_point = rotation_point;
            rotation_point = center + rotation.to_vec2_normalized() * r;
            self.triangle(center, old_angle_point, rotation_point, color);
        }
    }
}


/// Copied from macroquad 
#[derive(Debug, Clone, Default)]
pub struct DrawFont<'a>
{
    pub font: Option<&'a Font>,
    pub angle: Angle,
}

impl<'a> DrawFont<'a>
{
    pub fn with_font(self, font : &'a Font) -> Self { self.with_optionnal_font(Some(font)) }
    pub fn with_optionnal_font(mut self, font : Option<&'a Font>) -> Self { self.font = font; self }
    pub fn with_angle(mut self, angle : Angle) -> Self { self.angle = angle; self }
}


/* 
#[derive(Debug, Clone, Copy)]
pub struct TextSize {
    pub size   : Vec2,
    pub offset : Vec2,
}*/

impl ContextPen
{
    pub fn mesure_text(&self, text : &str, font : Option<&Font>, font_scale : real) -> Vec2 
    { self.mesure_text_with_fontsize(text, font, ContextAssetManager::FONT_DEFAULT_SIZE, font_scale/ContextAssetManager::FONT_DEFAULT_SIZE.to_real()) }
    pub fn mesure_text_with_fontsize(&self, text : &str, font : Option<&Font>, font_size : FontSize, font_scale : real) -> Vec2
    {
        let dim = prelude::measure_text(&text, font.map(|e| &e.mq_font), font_size as u16, font_scale as f32);
        let (width, mut height) = (dim.width as real, dim.height as real);
        if !height.is_finite() 
        {
            height = font_size as real * font_scale;
        }
        vec2(width, height)
    }

    pub fn text(&mut self, text : &str, pos : Vec2, font_scale : real, center : Vec2, color: Color, params : DrawFont)
    {
        self._text(text, pos, font_scale, center, color, params)
    }



    fn _text<'a>(&mut self, text : &str, mut pos : Vec2, mut font_scale : real, center : Vec2, color: Color, params : DrawFont)
    {
        self.total_draw_text_len += text.len();

        let font_size  = ContextAssetManager::FONT_DEFAULT_SIZE;
        font_scale /= font_size as real;

        let f = if params.font.is_none() { self.fonts.last() } else { params.font };

        let dim = self.mesure_text_with_fontsize(text, f, font_size, font_scale);

        pos -= dim * center;
        
        prelude::draw_text_ex(&text, pos.x as f32, pos.y as f32, 
            macroquad::text::TextParams
            {
                font_scale: -font_scale as f32,
                font_size : font_size,
                font_scale_aspect : -1.,
                color : color.to_lib(),
                font : f.map(|e| &e.mq_font),
                rotation : params.angle.radian() as f32
            }
        );
    }
}


impl ContextPen
{
    pub fn debug_self(&mut self) { self.debug_self = true; }

    pub fn debug<E : Debug>(&mut self, e : &E) { self.debug_text(format!("{:?}", e)) }
    pub fn debug_text(&mut self, txt : String) { self.text_debug.push(txt); }
}



impl ContextPen
{
    fn print_debug_txt(&mut self, text : &str, text_debug_height : &mut real)
    {
        
        let font_size  = 16;
        let font_scale : real = 1.;
        let (_width, height) = self.mesure_text_with_fontsize(text, None, font_size, font_scale).into();

        if height == 0. { return; }

        let shadow_offset_px = 1.;

        // shadow
        //prelude::draw_text_ex(&text, shadow_offset_px, *text_debug_height - height - shadow_offset_px, macroquad::text::TextParams{font_scale: -font_scale, font_size : font_size, font_scale_aspect : -1., color: MColorMod::BLACK.into(), ..Default::default()});
        //prelude::draw_text_ex(&text, 0., *text_debug_height - height - shadow_offset_px, macroquad::text::TextParams{font_scale: -font_scale, font_size : font_size, font_scale_aspect : -1., color: MColorMod::BLACK.into(), ..Default::default()});
        //prelude::draw_text_ex(&text, shadow_offset_px, *text_debug_height - height, macroquad::text::TextParams{font_scale: -font_scale, font_size : font_size, font_scale_aspect : -1., color: MColorMod::BLACK.into(), ..Default::default()});
        //self.text(text, vec2(shadow_offset_px, *text_debug_height - height - shadow_offset_px), font_scale * (font_size as real), BLACK);
        //self.text(text, vec2(-shadow_offset_px, *text_debug_height - height + shadow_offset_px), font_scale * (font_size as real), BLACK);
        //self.text(text, vec2(-shadow_offset_px, *text_debug_height - height - shadow_offset_px), font_scale * (font_size as real), BLACK);
        //self.text(text, vec2(shadow_offset_px, *text_debug_height - height + shadow_offset_px), font_scale * (font_size as real), BLACK);
        self._text(text, vec2(shadow_offset_px, *text_debug_height - height), font_scale * (font_size as real), Vec2::ZERO, Color::BLACK, ___());
        self._text(text, vec2(-shadow_offset_px, *text_debug_height - height), font_scale * (font_size as real), Vec2::ZERO, Color::BLACK, ___());
        self._text(text, vec2(0., *text_debug_height - height -shadow_offset_px), font_scale * (font_size as real), Vec2::ZERO, Color::BLACK, ___());
        self._text(text, vec2(0., *text_debug_height - height +shadow_offset_px), font_scale * (font_size as real), Vec2::ZERO, Color::BLACK, ___());
        
        // text
        self._text(text, vec2(0., *text_debug_height - height), font_scale * (font_size as real), Vec2::ZERO, Color::WHITE, ___());
        *text_debug_height -= height*1.;
    }

    fn print_all_debug_txt<Glob>(c : &mut DefaultContext<Glob>) -> real
    {
        let mut text_debug_height = c.pen.cam.window_size_px().y;
        for text in mem::replace(&mut c.pen.text_debug, vec![]).into_iter()
        {
            //println!("{}", text);
            c.pen.print_debug_txt(&text, &mut text_debug_height);
        }
        text_debug_height
    }
}
