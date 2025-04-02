use std::time::{Duration, Instant};

use super::*;

#[derive(Clone, Debug, Default)]
pub struct GraphicBoardAi
{
    //pub nb_action_force_brut : usize,
    pub result : MinimaxResult,
}
impl PartialEq for GraphicBoardAi { fn eq(&self, other: &Self) -> bool { true }}

impl GraphicBoardGame
{
    pub fn ai_graphic_best_move(&mut self) -> ActionID
    {
        self.ai.result = self.ai_minimax_default();
        self.ai.result.action_id.unwrap()
    }
}
