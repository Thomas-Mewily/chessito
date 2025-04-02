use super::*;

pub trait BoardUiExtension<Owner : UiOwner>
{
    fn board_button(&mut self, icon : impl ToUiIcon<Glob>, input : Owner::Input, ctx : &mut Context) -> &mut Self;
    
    fn board_icon_button(&mut self, icon : impl ToUiIcon<Glob>, input : Owner::Input, ctx : &mut Context) -> &mut Self;
    fn board_icon(&mut self, icon : impl ToUiIcon<Glob>, ctx : &mut Context) -> &mut Self;

    fn board_input_pressed(&mut self, input : Owner::Input) -> &mut Self;

    fn board_relic_button(&mut self, icon : impl ToUiIcon<Glob>, input : <Owner>::Input, ctx : &mut Context) -> &mut Self;
}

impl<Owner : UiOwner> BoardUiExtension<Owner> for UiElement<Owner>
{


    fn board_button(&mut self, icon : impl ToUiIcon<Glob>, input : Owner::Input, ctx : &mut Context) -> &mut Self {
        self
            .add_icon(icon.to_ui_icon(ctx))
            .set_icon_margin(0.125.ui_min().splat2())
            .add_nine_slice(GameUiNineSlice::Button.to_ui_nine_slice(ctx))
            .board_input_pressed(input)
    }

    
    fn board_icon_button(&mut self, icon : impl ToUiIcon<Glob>, input : <Owner>::Input, ctx : &mut Context) -> &mut Self {
        self.board_icon(icon, ctx).board_input_pressed(input)
    }

        
    fn board_relic_button(&mut self, icon : impl ToUiIcon<Glob>, input : <Owner>::Input, ctx : &mut Context) -> &mut Self {
        self
            .add_nine_slice(GameUiNineSlice::Relics.to_ui_nine_slice(ctx))
            .add_icon(icon.to_ui_icon(ctx))
            .set_icon_margin(0.125.ui_min().splat2())
            .board_input_pressed(input)
    }

    fn board_input_pressed(&mut self, input : Owner::Input) -> &mut Self
    {
        self.add_input_released(input)
    }

    
    fn board_icon(&mut self, icon : impl ToUiIcon<Glob>, ctx : &mut Context) -> &mut Self {
        self
            .add_nine_slice(GameUiNineSlice::IconButton.to_ui_nine_slice(ctx))
            .add_icon(icon.to_ui_icon(ctx))
            .set_icon_margin(0.125.ui_min().splat2())
    }
        
    // .add_nine_slice(GameUiNineSlice::IconButton.to_ui_nine_slice(ctx))


}


#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoardIcon
{
    Leave = 0,
    Option,
    Refuse,
    SfxOn,

    More,
    Stat,
    Accept,
    SfxOff,

    Undo,
    Redo,
    Learn,
    MusicOn,

    Prev,
    Next,
    Help,
    MusicOff,

    Pause,
    Home,
    RobotEasy,
    Slash,

    Left,
    Right,
    RobotNormal,
    Human,
    
    Up,
    Down,
    RobotHard,
    Lightning,

    Board,
    Puzzle,
    Versus,

    RelicAnticipation,
    RelicExplosif,
    RelicDuck,
    RelicAbsorb,
    RelicMoveTwice,
}

impl ToUiSprite<Glob> for BoardIcon
{
    fn to_ui_sprite(self, ctx : &DefaultContext<Glob>) -> UiSprite 
    { 
        match self
        {
            BoardIcon::Board  => { UiSprite::new(&ctx.globals.assets.img.ui.icon.board) },
            BoardIcon::Puzzle => { UiSprite::new(&ctx.globals.assets.img.ui.icon.puzzle_piece) },
            BoardIcon::Versus => { UiSprite::new(&ctx.globals.assets.img.ui.icon.versus) },

            BoardIcon::RelicAnticipation | BoardIcon::RelicExplosif | BoardIcon::RelicAbsorb | BoardIcon::RelicDuck | BoardIcon::RelicMoveTwice => 
            { UiSprite::new_animated_from_idx(&ctx.globals.assets.img.relics, self as usize - Self::RelicAnticipation as usize) },

            _ => { UiSprite::new_animated_from_idx(&ctx.globals.assets.img.ui.button, self as usize) }
        }
    }
}
impl ToUiIcon<Glob> for BoardIcon
{
    fn to_ui_icon(self, ctx : &DefaultContext<Glob>) -> UiIcon 
    {
        let mut icon = UiIcon::new(Some(self.to_ui_sprite(ctx)));
        icon.set_margin(0.125.ui_min().splat2());
        icon
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameUiNineSlice
{
    Button = 0,
    IconButton,
    Page,
    Relics,
}

impl ToUiNineSlice<Glob> for GameUiNineSlice
{
    fn to_ui_nine_slice(self, ctx : &mut DefaultContext<Glob>) -> UiNineSlice 
    {
        let texture = &ctx.globals.assets.img.ui.nine_slice;
        let mut s = UiNineSlice::new_from_texture_point2(texture, point2(0, self as isize * 3));

        match self
        {
            GameUiNineSlice::IconButton => 
            {
                s.set_angle_size(0.0250.ui_window_min().splat2());
            },
            _ => {},
        }
        s
    }   
}

impl ToUiSprite<Glob> for Relic
{
    fn to_ui_sprite(self, ctx : &DefaultContext<Glob>) -> UiSprite 
    { UiSprite::new_animated_from_idx(&ctx.globals.assets.img.relics, self as usize) }
}