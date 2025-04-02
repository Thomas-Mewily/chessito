use std::default;

use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum MenuInput
{
    #[default]
    Nothings,
    GoHome,
    ClassicParam,
    ClassicChangePlayer(usize),

    CreditOpen,
    ClosePage,

    Puzzle,
    InGame(GraphicBoardGameInput),

    ToggleMusic,
    ToggleSfx,

    ClassicCancel,
    ClassicStart,

    ToggleRelicForTeams(Relic)
}
impl IInput for MenuInput 
{
    fn have_side_effect(&self) -> bool {
        match self
        {
            MenuInput::Nothings => false,
            MenuInput::InGame(g) => g.have_side_effect(),
            _ => true,
        }
    }

    fn combine(&mut self, other : Self) {
        if !self.have_side_effect() && !matches!(self, MenuInput::InGame(_))
        { *self = other; return; }
    }
}
impl MenuInput
{
    pub fn as_in_game(self) -> GraphicBoardGameInput { self.try_as_in_game().unwrap() }
    pub fn try_as_in_game(self) -> Option<GraphicBoardGameInput>
    {
        match self
        {
            MenuInput::InGame(g) => Some(g),
            _ => None,
        }
    }
}

/* 
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct MenuInputProvider;
impl IInputProvider<Menu> for MenuInputProvider
{
    fn get_input(&mut self, game : &mut Menu, time : DrawTime, ctx : &mut DefaultContext<Glob>) -> MenuInput 
    {
        game.ui.get_input_ui(game, self, time, ctx)
    }
}*/

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum MenuUiName
{
    Splash,

    Puzzle,
    
    TitleScreen,
    Credits,
    CreditsClose,
    CreditsText,
    MoreGameHere,

    TitleTop,
    TitleBotClassicOrPuzzle,
    TitleBotClassicParam,
    ClassicParam,
    ClassicParamPlayer(usize),
    ClassicCancel,
    ClassicStart,

    ToggleMusic,
    ToggleSfx,


    BoardHud,

    Home,
    SeeAlTeamPath,
    Undo,
    Redo,

    Learn,

    Board,
    CapturedPiece,
    Relic(Relic)
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum MenuUiPageName
{
    TitleScreen,
    OptionsCredits,
    Learn,
    InGame,
    Splash,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuScene
{
    #[default]
    TitleScreen,
    InGame,
}

impl UiOwner for Menu
{
    type UiName=MenuUiName;
    type UiPageName=MenuUiPageName;

    fn ui(&self) -> &UiManager<Self> { &self.ui }
    fn ui_mut(&mut self) -> &mut UiManager<Self> { &mut self.ui }

    fn draw_ui_element(&self, name : &MenuUiName, ctx : &mut DefaultContext<Glob>) 
    {
        match name
        {
            MenuUiName::Board => self.board.draw(ctx),
            MenuUiName::CapturedPiece => self.board.game.draw_captured_piece_side(self.board.game_time, ctx),
            _ => {}
        }
    }
    
    fn get_input_ui_element(&mut self, input_provider : &mut Self::InputProvider, name : Self::UiName, ctx : &mut DefaultContext<Self::Global>) -> Option<Self::Input>
    {
        match name
        {
            MenuUiName::Board => 
            {
                let v = self.board.input(ctx);
                //if v.is_nothings() { None } else {  }
                Some(MenuInput::InGame(v))
                /*
                //println!("{:?}", v);
                if v.is_nothings()
                {
                    None
                }else
                {
                }*/
            },
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Menu
{
    ui    : UiManager<Self>,

    board : GameRunner<GraphicBoardGame>,

    //scene : MenuScene,
}

impl Menu
{
    pub fn new(ctx : &mut Context) -> Self 
    {
        let mut players = [PlayerKind::Cpu(CpuDifficulty::Easy); Team::LENGHT];
        players[0] = PlayerKind::Human;

        let mut s = Self 
        { 
            ui : ___(),
            board : GameRunner::new(GraphicBoardGame::new(BoardGame::new_chess(), players), ctx), 
            //scene : ___()
        };
        //s.go_home(___(), ctx);
        s.go_splash(___(), ctx);
        s
    }

    fn ui_init_pop_up(&mut self, page_name : MenuUiPageName, time : Time, ctx : &mut Context)
    {
        self.ui.push_page(page_name);
        self.ui_init_page(ctx);

        self.ui.add_lambda()
            .add_color(Color::BLACK.with_a(0.75));

        
        self.ui.pos_push_x4_margin_left_right_top_bot(0.1.ui_window_min());

        let p = self.ui.pos;

        for i in 0..4
        {
            self.ui.pos_pop_rest();
            self.ui.add_lambda().add_input_released(MenuInput::ClosePage).input_sound_clear();
            self.ui.last_pop();
        }

        self.ui.proto_mut().pos = p;
        

        let slice = GameUiNineSlice::Page.to_ui_nine_slice(ctx);
        let slice_angle = slice.angle_size();

        self.ui.add_lambda()
            .add_nine_slice(slice);

        self.ui.pos_push_x4_margin_left_right_top_bot_vec(slice_angle);
        
        //let m = self.default_margin();
        //self.ui.pos_push_glue_top((1.ui_min() - m * 3.)/4.);
        self.ui.pos_push_glue_top(1.ui_min() / 4.);

        self.ui.add_lambda()
            .add_split_on_x();
    }

    pub fn pop_up_add_top_action(&mut self, icon : impl ToUiIcon<Glob>, name : MenuUiName, input : MenuInput, ctx : &mut Context)
    {
        self.ui
            .add_named(name)
            .board_icon_button(icon, input, ctx)
            .in_split_default();

        self.ui.last_pop();
    }

    pub fn music_icon(&self, ctx : &mut Context) -> BoardIcon
    {
        if ctx.audio.music_coef() >= 0.5 { BoardIcon::MusicOn } else { BoardIcon::MusicOff }
    }

    pub fn sfx_icon(&self, ctx : &mut Context) -> BoardIcon
    {
        if ctx.audio.sfx_coef() >= 0.5 { BoardIcon::SfxOn } else { BoardIcon::SfxOff }
    }

    pub fn go_to_option_credits(&mut self, time : Time, ctx : &mut Context)
    {
        self.ui_init_pop_up(MenuUiPageName::OptionsCredits, time, ctx);

        self.pop_up_add_top_action(BoardIcon::Home, MenuUiName::Home, MenuInput::GoHome, ctx);

        self.pop_up_add_top_action(self.music_icon(ctx), MenuUiName::ToggleMusic, MenuInput::ToggleMusic, ctx);
        self.pop_up_add_top_action(self.sfx_icon(ctx), MenuUiName::ToggleSfx, MenuInput::ToggleSfx, ctx);

        self.pop_up_add_top_action(BoardIcon::Refuse, MenuUiName::CreditsClose, MenuInput::ClosePage, ctx);

        self.ui.pos_pop_rest();



        let credits = self.credits(ctx);
        self.ui.add_lambda()
            //.add_color(Color::RED)
            //.add_color(Color::BLACK.with_a(0.25))
            //.add_nine_slice(GameUiNineSlice::IconButton.to_ui_nine_slice(ctx))
            .add_text(credits)
            .set_text_font(Some(ctx.globals.assets.img.ui.font.stanberry.clone()));



        self.ui.build(ctx);
    }

    pub fn go_splash(&mut self, time : Time, ctx : &mut Context)
    {
        self.ui.push_page(MenuUiPageName::Splash);
        self.ui_init_page(ctx);

        self.ui.add_named(MenuUiName::Splash)
            .add_icon(UiIcon::new_from_texture(&ctx.globals.assets.img.ui.splash))
            .board_input_pressed(MenuInput::GoHome);

    
        self.ui[MenuUiName::Splash].pos.set_src_relative_to_dest(UiRect::new(ui_vec2(zero(), 1.ui_max()), zero())).set_duration(0.75.s());

        self.ui_init_page(ctx);
    }

    pub fn go_in_game(&mut self, time : Time, ctx : &mut Context)
    {
        self.ui.push_page(MenuUiPageName::InGame);
        self.ui_init_page(ctx);

        self.board = GameRunner::new(GraphicBoardGame::new(BoardGame::new_chess_custom(2, true, self.board.game.team_data[Team::White].relics), self.board.game.players), ctx);
        //self.scene = MenuScene::InGame;

        ctx.audio.play(&ctx.globals.assets.sound.board.event.start);

        self.ui.add_lambda()
            .add_split_on_max()
            .add_split_sfx(&ctx.assets().sound.ui.split)
            .set_split_margin(zero())
            ;
        {
            let hud_size = 0.2.ui_min(); //(1.ui_window_min() - self.default_margin() * 3.)/4.;
            let wood_texture = UiIcon::new_from_texture(&ctx.globals.assets.img.ui.hud_top).with_fill_mode(UiIconFillMode::Stretch);

            self.ui.add_named(MenuUiName::BoardHud)
                .in_split_unit(hud_size)
                .set_icon(wood_texture.clone())
                .in_split_set_z_idx(0.)
                .add_split_on_max();
            {
                self.ui.add_named(MenuUiName::Credits)
                    .in_split_default()
                    .board_icon_button(BoardIcon::More, MenuInput::CreditOpen, ctx)
                ;
                self.ui.last_pop();

                self.ui.add_named(MenuUiName::SeeAlTeamPath)
                .in_split_default()
                .board_icon_button(game_ui::BoardIcon::Lightning, MenuInput::InGame(GraphicBoardGameInput::Graphic(GraphicActionID::HoverTeamToggle)), ctx)
                ;
                self.ui.last_pop();

                self.ui.add_named(MenuUiName::Undo)
                    .in_split_default()
                    .board_icon_button(BoardIcon::Undo, MenuInput::InGame(GraphicBoardGameInput::Logic(LogicActionID::Undo)), ctx)
                ;
                self.ui.last_pop();

                self.ui.add_named(MenuUiName::Redo)
                    .in_split_default()
                    .board_icon_button(BoardIcon::Redo, MenuInput::InGame(GraphicBoardGameInput::Logic(LogicActionID::Redo)), ctx)
                ;
                self.ui.last_pop();

                /* 
                self.ui.add_named(MenuUiName::Learn)
                .in_split_default()
                .board_icon_button(BoardIcon::Learn, MenuInput::InGame(GraphicBoardGameInput::Logic(LogicActionID::Redo)), ctx)
                ;
                self.ui.last_pop();
                */

                // .add_sprite(Relic::Anticipation.to_ui_sprite(ctx))
            }
            self.ui.last_pop();

            self.ui.add_named(MenuUiName::Board)
                .add_interactivity()
                .in_split_default()
                .in_split_set_z_idx(1.)
                .input.clear_sound()
            ;
            self.ui.last_pop();

            self.ui.add_named(MenuUiName::CapturedPiece)
                .in_split_unit(hud_size)
                .set_icon(wood_texture.clone())
                .in_split_set_z_idx(0.)
            ;
            self.ui.last_pop();
        }
        self.ui.last_pop();


        //self.ui.last_pop();
        //self.ui.pos_pop_rest();


        self.ui.build(ctx);
    }

    pub fn default_margin(&self) -> UiUnit { 0.025.ui_window_min() }

    pub fn ui_init_page(&mut self, ctx : &mut Context)
    {
        self.ui.proto_mut().pos.set_duration(1.5.s()).set_ease(Easing::robot());
        let default_margin = self.default_margin();
        self.ui.proto_mut().split_on.set_margin(default_margin);

        self.ui.proto_mut().input.set_sound(
            UiElementInputSound::___()
            .add_hover_in(&ctx.assets().sound.ui.hover_in)
            .add_pressed(&ctx.assets().sound.ui.press)
        );
        //.add_sfx_hover_out(&ctx.assets().sound.ui_hover_in)
    }

    pub fn go_home(&mut self, time : Time, ctx : &mut Context)
    {
        self.ui.push_page(MenuUiPageName::TitleScreen);
        self.ui_init_page(ctx);
        //self.ui.clear();
        //self.scene = MenuScene::TitleScreen;

        self.ui.proto_push();



        let default_margin = self.default_margin();
        self.ui.pos_push_x4_margin_left_right_top_bot(default_margin);
        

        let button_nine_slice =  GameUiNineSlice::Button.to_ui_nine_slice(ctx);

        let top_coef = 1./4.;
        let bot_coef = 1. - top_coef;

        {
            self.ui.add_named(MenuUiName::TitleTop).add_split_on_y();

            {
                //self.ui.pos_push_glue_top(0.25.ui_axis());
                self.ui.add_lambda().add_split_on_x().add_color(Color::from_rgb_hex(0x009129)).in_split_weight(top_coef);
                {
                    let unit_button = 0.15.ui_window_min();

                    self.ui.add_named(MenuUiName::Credits)
                        .board_icon_button(BoardIcon::More, MenuInput::CreditOpen, ctx)
                        .in_split_unit(unit_button)
                        ;
                    self.ui.last_pop();


                    self.ui.add_named(MenuUiName::TitleScreen)
                        .add_sprite(ctx.globals.assets.img.ui.title.to_ui_sprite(ctx))
                        .in_split_weight(4.)
                        ;
                    self.ui.last_pop();

                    self.ui.add_lambda()
                        //.add_split_config_square()
                        //.add_split_config_weight(1.)
                        .in_split_unit(unit_button)
                        ;
                    self.ui.last_pop();
                }
                self.ui.pos_pop_rest();
            }

            self.ui.add_lambda().in_split_weight(bot_coef);
            {
                {
                    self.ui.add_named(MenuUiName::TitleBotClassicParam)
                        .add_split_on_min()
                        ;
                    {
                        self.ui.add_lambda().in_split_default().add_color(Color::from_rgb_hex(0xFF6A00));
                        {
                            self.ui.add_named(MenuUiName::ClassicParamPlayer(0))
                                .in_split_weight(1.)
                                .board_icon_button(BoardIcon::Human, MenuInput::ClassicChangePlayer(0), ctx)
                            ;
                            self.ui_update_player(0, ctx);
                            self.ui.last_pop();


                            self.ui.add_lambda()
                                .in_split_weight(1.)
                                .add_sprite(UiSprite::new(&ctx.globals.assets.img.ui.icon.versus))
                                ;
                            self.ui.last_pop();
                            

                            self.ui.add_named(MenuUiName::ClassicParamPlayer(1))
                                .in_split_weight(1.)
                                .board_icon_button(BoardIcon::Human, MenuInput::ClassicChangePlayer(1), ctx)
                            ;
                            self.ui_update_player(1, ctx);
                            self.ui.last_pop();
                        }
                        self.ui.last_pop();


                        let have_relic = true;
                        if have_relic
                        {
                            self.ui.add_lambda().in_split_weight(0.5).add_color(Color::from_rgb_hex(0xAF2AA4)).add_color(Color::from_rgb_hex(0xC600AC));
                            for r in [Relic::Anticipation, Relic::MoveTwiceInATurn, Relic::Explosive, Relic::Absorb]
                            {
                                self.ui.add_named(MenuUiName::Relic(r))
                                    .in_split_square()
                                    .board_relic_button(
                                        match r
                                        {
                                            Relic::Anticipation => BoardIcon::RelicAnticipation,
                                            Relic::Explosive => BoardIcon::RelicExplosif,
                                            Relic::DuckButDifferent => BoardIcon::Slash,
                                            Relic::Absorb => BoardIcon::RelicAbsorb,
                                            Relic::MoveTwiceInATurn => BoardIcon::RelicMoveTwice,
                                        }
                                        , MenuInput::ToggleRelicForTeams(r), ctx
                                    );
                                self.ui.last_pop();
                                self.ui_update_relic(r);
                            }
                            self.ui.last_pop();
                        }

                        self.ui.add_lambda().in_split_default().add_split_on_max().add_color(Color::from_rgb_hex(0x3096DB));
                        {
                            self.ui.add_named(MenuUiName::ClassicCancel)
                                .in_split_weight(1.)
                                .board_button(BoardIcon::Refuse, MenuInput::ClassicCancel, ctx)
                            ;
                            self.ui.last_pop();

                            self.ui.add_named(MenuUiName::ClassicStart)
                                .in_split_weight(3.)
                                .board_button(BoardIcon::Accept, MenuInput::ClassicStart, ctx)
                            ;
                            self.ui.last_pop();
                        }
                        self.ui.last_pop();
                    }
                    self.ui.last_pop();
                }

                //self.ui.pos_push_margin_top(default_margin);
                self.ui.add_named(MenuUiName::TitleBotClassicOrPuzzle)
                    .add_split_on_max()
                    .add_split_sfx(&ctx.assets().sound.ui.split);
                {
                    self.ui.add_named(MenuUiName::ClassicParam)
                        .in_split_with(UiSplit::new_with_weight(7.))
                        .board_button(BoardIcon::Board, MenuInput::ClassicParam, ctx)
                        .add_split_on_y()
                    ;
                    self.ui.last_pop();
        
                    /*
                    self.ui.add_named(MenuUiName::Puzzle)
                        .in_split_with(UiSplit::new_with_weight(3.))
                        .board_button(BoardIcon::Puzzle, MenuInput::Puzzle, ctx)
                    ;
                    self.ui.last_pop();*/
                    
                }
                self.ui.last_pop();
                
            }
            self.ui.last_pop();
        }

        self.ui.build(ctx);


                        
        self.ui[MenuUiName::TitleScreen].pos.set_src_relative_to_dest(UiRect::new(ui_vec2(zero(), top_coef.ui_window_max()), zero())).set_duration(0.75.s());
        self.ui[MenuUiName::Credits].pos.set_src_relative_to_dest(UiRect::new(ui_vec2(-0.5.ui_window_min(), zero()), zero())).set_duration(3.s());
        self.ui[MenuUiName::ClassicParam].pos.set_src_relative_to_dest(UiRect::new(ui_vec2(zero(), -bot_coef.ui_window_max()), zero()));
        //self.ui[MenuUiName::Puzzle].pos.set_src_relative_to_dest(UiRect::new(ui_vec2(zero(), -bot_coef.ui_window_max()), zero()));
    
        self.ui[MenuUiName::TitleBotClassicParam].desactivate();
    }

    fn ui_update_relic(&mut self, r : Relic)
    {
        let is_on = self.board.game.team_data[Team::White].relics.flag_have(r);
        self.ui[MenuUiName::Relic(r)].icon.sprite.as_mut().map(|e| e.map(|s| 
            { s.color = if is_on { Color::WHITE } else { Color::BLACK }}
        ));

    }

    fn ui_update_player(&mut self, idx : usize, ctx: &mut Context)
    {
        let (icon, color) = match self.board.game.players[idx]
        {
            PlayerKind::Human => (BoardIcon::Human, Color::from_rgb_hex(0xFF9F19)),
            PlayerKind::Cpu(lvl) => 
            {
                match lvl
                {
                    CpuDifficulty::Easy   => (BoardIcon::RobotEasy, Color::from_rgb_hex(0x52A542)),
                    CpuDifficulty::Normal => (BoardIcon::RobotNormal, Color::from_rgb_hex(0x50A3C4)),
                    CpuDifficulty::Hard   => (BoardIcon::RobotHard, Color::from_rgb_hex(0xD3324D)),
                }
            }
        };

        let sprite = icon.to_ui_sprite(ctx);

        self.ui[MenuUiName::ClassicParamPlayer(idx)].add_sprite(sprite).add_color(color);
    }

    async fn _update(&mut self, input : MenuInput, time : GameTime, ctx : &mut Context) -> Option<bool> 
    {
        self.ui.update_ui(time, ctx);

        match input
        {
            MenuInput::ClassicParam => 
            { 
                self.ui[MenuUiName::TitleBotClassicOrPuzzle].desactivate();
                self.ui[MenuUiName::TitleBotClassicParam].activate();
            },
            MenuInput::ClassicCancel => 
            { 
                self.ui[MenuUiName::TitleBotClassicOrPuzzle].activate();
                self.ui[MenuUiName::TitleBotClassicParam].desactivate();
            },

            MenuInput::ClassicChangePlayer(idx) =>
            {
                self.board.game.players[idx] = match self.board.game.players[idx]
                {
                    PlayerKind::Human  => PlayerKind::Cpu(CpuDifficulty::Easy),
                    PlayerKind::Cpu(lvl) => 
                    {
                        match lvl
                        {
                            CpuDifficulty::Easy => PlayerKind::Cpu(CpuDifficulty::Normal),
                            CpuDifficulty::Normal => PlayerKind::Cpu(CpuDifficulty::Hard),
                            CpuDifficulty::Hard => PlayerKind::Human,
                        }
                    },
                };
                self.ui_update_player(idx, ctx);
            }
            MenuInput::InGame(input) => 
            {
                self.board.update(input, time.delta(), ctx).await;
            },

            MenuInput::ClassicStart => 
            { 
                self.ui.pop_page();
                self.go_in_game(time.total(), ctx);
            },
            MenuInput::GoHome => 
            { 
                self.ui.clear_pages();
                self.go_home(time.total(), ctx);
            },

            MenuInput::ToggleRelicForTeams(r) =>
            {
                for (t, data) in self.board.game.team_data.iter_mut()
                {
                    data.relics.flag_toggle(r);
                }

                self.ui_update_relic(r);
            }

            MenuInput::ToggleMusic => 
            { 
                ctx.audio.set_music_coef(if ctx.audio.music_coef() >= 0.5 { 0. } else { 1. });
                let icon = self.music_icon(ctx).to_ui_icon(ctx);
                self.ui[MenuUiName::ToggleMusic].set_icon(icon);

                ctx.audio.set_volume(&ctx.globals.assets.sound.stream_loops_2023_11_29, ctx.audio.music_coef());
            },

            MenuInput::ToggleSfx => 
            {
                ctx.audio.set_sfx_coef(if ctx.audio.sfx_coef() >= 0.5 { 0. } else { 1. });

                if ctx.audio.sfx_coef() >= 0.5 { ctx.audio.play(&ctx.globals.assets.sound.board.bishop.select);}
                let icon = self.sfx_icon(ctx).to_ui_icon(ctx);
                self.ui[MenuUiName::ToggleSfx].set_icon(icon);
            },

            MenuInput::ClosePage =>
            {
                self.ui.pop_page();
            }

            MenuInput::CreditOpen => 
            {
                self.go_to_option_credits(time.total(), ctx);
            }

            _ => {}
        }

        None
    }

    fn _draw(&self, time : GameTime, ctx : &mut Context)
    {
        if let Some(shader) = &ctx.globals.shader
        {
            macroquad::material::gl_use_material(shader);
        }

        self.ui.draw(self, ctx);

        macroquad::material::gl_use_default_material();

        /* 
        for i in 0..30
        {
            ctx.pen.debug_text(format!("{:?}", quad_timestamp::timestamp_utc()));
        }
        */
    }

    pub fn credits(&self, ctx : &mut Context) -> String
    {
        //"hello world\nhi ok long text\nit work\n123456789 I know how to count wow".to_owned()
        let mut credits = include_str!("../../credit.txt").to_owned();

        let mut assets_credits = vec![];
        ctx.globals.assets.get_credits(&mut assets_credits);

        for c in assets_credits
        {
            credits.push_str(&c);
            credits.push('\n');
        }

        credits.push_str("\n");

        credits
    }
}

impl IGame for Menu
{
    type Input=MenuInput;
    type InputProvider=InputProvidedByUi;

    type Result=bool;
    type Global=Glob;

    async fn update(&mut self, input : Self::Input, time : GameTime, ctx : &mut DefaultContext<Glob>) -> Option<Self::Result> 
    { self._update(input, time, ctx).await }

    fn draw(&self, time : GameTime, ctx : &mut DefaultContext<Glob>)
    {  self._draw(time, ctx); }
}
