#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{mem, ops::{Deref, DerefMut}};

use board::*;
use game_engine::DefaultContext;
use game_engine::*;
use ui::*;

pub mod generated;
pub use generated::*;

pub mod board_graphic;
pub use board_graphic::*;

pub mod menu;
pub use menu::*;

pub mod game_ui;
pub use game_ui::*;

macro_asset_loader::import_asset!("./board_graphic/assets", "./board_graphic/src/generated/generated_assets.rs");

use macroquad::{material::Material, texture::load_image};

pub struct Glob
{
    pub assets : AssetAssets,
    pub shader : Option<Material>,
}

pub type Context = DefaultContext<Glob>;

pub trait ContextAssetExtension
{
    fn assets(&self) -> &AssetAssets;
    fn assets_mut(&mut self) -> &mut AssetAssets;
}
impl ContextAssetExtension for Context
{
    fn assets(&self) -> &AssetAssets { &self.globals.assets }
    fn assets_mut(&mut self) -> &mut AssetAssets { &mut self.globals.assets }
}


fn window_conf() -> macroquad::window::Conf {


    //use crate::miniquad::conf::Icon;
    //let v = macroquad::telemetry::textures_count();

    macroquad::window::Conf {
        window_title: "ChessIto 1.0".to_owned(),
        fullscreen: false,
        high_dpi: true,
        window_resizable : true,
        icon : Some(get_icon()),
        ..Default::default()
    }
}


fn load_img(bytes: &'static [u8]) -> macroquad::texture::Image {
    return macroquad::texture::Image::from_file_with_format(bytes, Some(macroquad::prelude::ImageFormat::Png)).unwrap();
}

fn populate_array(img: macroquad::texture::Image, array: &mut [u8]) {
    let mut index: usize = 0;
    for pixel in img.get_image_data() {
        for value in pixel.iter() {
                array[index] = *value;
                index += 1;
        }
    }
}

// Thank to @ranmuran on discord on the MacroQuad server : https://discord.com/channels/710177966440579103/710180051349405746/1093173735734915092
pub fn get_icon() -> macroquad::miniquad::conf::Icon {
    /*
    let mut array_small: [u8; 1024] = [0; 1024];
    let mut array_medium: [u8; 4096] = [0; 4096];
    let mut array_big: [u8; 16384] = [0; 16384];
    */

    let mut array_small: [u8; 16*16*4] = [0; 16*16*4];
    let mut array_medium: [u8; 32*32*4] = [0; 32*32*4];
    let mut array_big: [u8; 64*64*4] = [0; 64*64*4];

    populate_array(load_img(include_bytes!(r"../assets/img/icon/icon_16px.png")), &mut array_small);
    populate_array(load_img(include_bytes!(r"../assets/img/icon/icon_32px.png")), &mut array_medium);
    populate_array(load_img(include_bytes!(r"../assets/img/icon/icon_64px.png")), &mut array_big);

    macroquad::miniquad::conf::Icon {
        small: array_small,
        medium: array_medium,
        big: array_big,
    }
}


#[macroquad::main(window_conf)]
async fn main()
{
    //println!("sizeof(UiElement) = {}", std::mem::size_of::<UiElement<Menu>>());



    /* 
    let tc = Time::from_s(42.);
    let a = tc;
    let b = tc;
    let c = tc;

    let v1 = UiVec::ZERO;
    let v2 = UiVec::ZERO;
    let v3 = v1 + v2;

    let r = UiRect::new(zero(), zero());
    let r2 = r * 0.9;
    let r3 = r2 * 0.9;
    let r4 = r2 + r3;

    let t = Rect2::ZERO;
    let t1 = t;
    let t2 = t;
    let t3 = t1 + t2;

    let v = 2.0.splat2();
    let l = v.length();

    let v2 = false.splat2();
    let l2 = v2.ry_x();
    */
    
    //println!("sizeof(piece logique) = {} octets", mem::size_of::<Piece<()>>());
    //println!("sizeof(piece grasize of the boardphique) = {} octets", mem::size_of::<Piece<G>>());
    
    let mut assets_manager = ContextAssetManager::default();

    let assets = AssetAssets::load(&mut assets_manager).await;

    //let t3 = assets.hud.icon.
    /* 
    let mut v = vec![];
    for i in 0..1024
    {
        v.push(assets.piece.cburnett.weak_clone())
    }
    */

    #[allow(unused_assignments)]
    let mut shader = Some({
        
        use macroquad::prelude::*;
        use miniquad::*;

        let vertex_shader = include_str!("./shader/vertex.glsl");
        let fragment_shader = include_str!("./shader/fragment.glsl");
    
        /*
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Zero,
                BlendFactor::Value(BlendValue::DestinationColor),
            )),

            
            color_blend: Some(BlendState::new(
                Equation::Subtract,
                BlendFactor::One,
                BlendFactor::Value(BlendValue::DestinationColor),
            )),

            color_blend: Some(BlendState::new(
                Equation::ReverseSubtract,
                BlendFactor::One,
                BlendFactor::Value(BlendValue::DestinationColor),
            )),


                        color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceColor),
                BlendFactor::OneMinusValue(BlendValue::DestinationAlpha),
            )
            ),
        */

        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,

            
            color_blend : Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
            )),

            /* 
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
            ),*/

            /* 
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceColor),
                //BlendFactor: Value(BlendValue::DestinationAlpha),
                BlendFactor::Value(BlendValue::DestinationAlpha),
            )),*/

            /* 
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::SourceAlphaSaturate,
            )),*/
            /* 
            alpha_blend: Some(BlendState::new(
                Equation::Subtract,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::One,
            )
            ),*/
            /*
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
            ),*/
            /* 
            alpha_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::)
            ),*/

            //color_write : (false, false, true, true),

            /*

                        color_blend : 
            Some(BlendState::new(
                Equation::Add,
                BlendFactor:: Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
            )
            ,

            alpha_blend:
            Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
            ),*/

            ..Default::default()
        };
    
        load_material(
            ShaderSource::Glsl {
                vertex: &vertex_shader,
                fragment: &fragment_shader,
            },
            MaterialParams {
                pipeline_params,
                ..Default::default()
            },
        )
        .unwrap()
    });

    
    shader = None;
    
    #[allow(unused_mut)]
    let mut globals = Glob { assets, shader };

    let mut ctx: DefaultContext<Glob> = Context::new(globals, assets_manager);


    ctx.audio.play_with_params(&ctx.globals.assets.sound.stream_loops_2023_11_29, SoundParams::new().and_loop_it().with_volume(0.3));

    ctx.pen.set_window_background_color(Color::from_rgb_hex(0x210033));
    
    //ctx.audio.play(&ctx.globals.assets.sound.zapsplat_fantasy_swell_mysterious_mystery_dark_eerie_92353);
    //ctx.audio.sfx(&ctx.globals.assets.sound.zapsplat_cartoon_impact_slip_trip_crash_90966);
    
    //let the_game = GraphicBoardGame::new(BoardGame::new_default());
    let the_game = Menu::new(&mut ctx);

    let mut g = GameRunner::new(the_game, &mut ctx);
    g.run(&mut ctx).await;
}