use super::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum CpuDifficulty
{
    #[default]
    Easy,
    Normal,
    Hard,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PlayerKind
{
    #[default]
    Human,
    Cpu(CpuDifficulty),
}
impl PlayerKind
{
    pub fn is_human(&self) -> bool { matches!(self, Self::Human) }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ParticleBase
{
    pub spawn : Time,
    pub pos   : Vec2,
}
#[derive(PartialEq, Clone, Debug)]
pub struct ParticlePieceCaptured
{
    pub base : ParticleBase,
    pub piece : Piece,
    pub speed : Vec2,
    pub size  : Vec2,
}
impl Deref for ParticlePieceCaptured
{
    type Target=ParticleBase;
    fn deref(&self) -> &Self::Target { &self.base }
}
impl DerefMut for ParticlePieceCaptured
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.base }
}


#[derive(PartialEq, Clone, Debug)]
pub struct GraphicBoardGame
{
    pub back_end : BoardGame,
    pub piece_selector : PieceSelector,

    pub players : [PlayerKind; Team::LENGHT],

    pub board_edge_color : Color,

    pub particles_captured_pieces : Vec<ParticlePieceCaptured>,
    pub particles_tile_explosion    : Vec<ParticleBase>,

    pub ai : GraphicBoardAi,
}

impl Deref for GraphicBoardGame { type Target = BoardGame; fn deref(&self) -> &Self::Target { &self.back_end }}
impl DerefMut for GraphicBoardGame { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.back_end }}


impl GraphicBoardGame
{
    pub fn new(back_end : BoardGame, players : [PlayerKind; Team::LENGHT]) -> Self 
    { 
        Self 
        { 
            back_end,
            piece_selector: ___(),
            ai : ___(),
            players,
            board_edge_color: 
            {
                let mut c = Color::from_rgb_hex(0x007F5B);
                let mut hard = false;

                for p in players.iter()
                {
                    match p
                    {
                        PlayerKind::Cpu(lvl) => 
                        {
                            match lvl
                            {
                                CpuDifficulty::Normal => { if !hard { c = Color::from_rgb_hex(0x006E9E) }},
                                CpuDifficulty::Hard => { hard = true; c = Color::from_rgb_hex(0xA50000) },
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                c
            },
            particles_captured_pieces: vec![],
            particles_tile_explosion: vec![], 
        }
    }
        
}

impl IGame for GraphicBoardGame
{
    type Input=GraphicBoardGameInput;
    type InputProvider=GraphicBoardGameInputProvider;
    type Result=BoardResult;
    type Global=Glob;

    async fn update(&mut self, input : Self::Input, time : GameTime, ctx : &mut DefaultContext<Glob>) -> Option<Self::Result> 
    { self._update(input, time, ctx) }

    fn draw(&self, time : GameTime, ctx : &mut DefaultContext<Glob>)
    { self._draw(time, ctx); }
}
