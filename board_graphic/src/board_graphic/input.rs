use std::{default, marker::PhantomData};

use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GraphicActionID
{
    #[default]
    Nothings,
    /// UnHover the rest
    Select(Option<At>),
    Hover (Option<At>),
    HoverTeam(Option<Team>),
    HoverTeamToggle,
}

impl GraphicActionID
{
    pub fn is_nothings(&self) -> bool { matches!(self, GraphicActionID::Nothings) }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogicActionID
{
    DoAction(ActionID),
    Undo,
    Redo,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GraphicBoardGameInput
{
    Logic(LogicActionID),
    Graphic(GraphicActionID),
}
impl GraphicBoardGameInput
{
    pub fn is_nothings(&self) -> bool 
    { 
        match self
        {
            GraphicBoardGameInput::Graphic(g) => g.is_nothings(),
            GraphicBoardGameInput::Logic(_) => false,
        }
    }
}
impl Default for GraphicBoardGameInput { fn default() -> Self { GraphicBoardGameInput::Graphic(___()) }}
impl From<ActionID> for GraphicBoardGameInput { fn from(value: ActionID) -> Self { GraphicBoardGameInput::Logic(LogicActionID::DoAction(value)) }}
impl From<GraphicActionID> for GraphicBoardGameInput { fn from(value: GraphicActionID) -> Self { GraphicBoardGameInput::Graphic(value) }}
impl IInput for GraphicBoardGameInput { fn have_side_effect(&self) -> bool { matches!(self, GraphicBoardGameInput::Logic(_)) }}

#[derive(Default, Clone, Debug)]
pub struct GraphicBoardGameInputProvider;
impl GraphicBoardGameInputProvider
{
    fn _input(&mut self, game : &mut GraphicBoardGame, time : GameTime, c : &mut DefaultContext<Glob>) -> GraphicBoardGameInput
    {
        use ActionID::*;
        use GraphicBoardGameInput::*;
        use GraphicActionID::*;
        use LogicActionID::*;

        if c.input.key(KeyCode::U).just_pressed() { return Logic(Undo); }
        if c.input.key(KeyCode::I).just_pressed() { return Logic(Redo); }
        if c.input.key(KeyCode::T).just_pressed() { return Graphic(HoverTeam(Some(game.current_team))); }


        if !game.is_end_of_the_game() && (c.input.key(KeyCode::Space).just_pressed() || c.input.key(KeyCode::Enter).just_pressed() || c.input.key(KeyCode::P).is_pressed())
        {
            let best_move = game.ai_graphic_best_move();
            return Logic(DoAction(best_move));
        }

        game.cam_board_begin(c);
        let mut input = self._get_input(game, time, c);
        game.cam_board_end(c);

        let p = &game.players[game.current_team as usize];
        {
            match p
            {
                PlayerKind::Human => { input },
                PlayerKind::Cpu(level) => 
                {
                    if input.have_side_effect() { input = ___(); }

                    if !game.is_end_of_the_game() && time.elapsed_since_last_input().s() >= 0.4
                    {
                        let action_id = game.minimax_custom((match level
                            {
                                // [0-1]
                                CpuDifficulty::Easy => 
                                {
                                    (game.turn as usize / 2) % 2
                                },
                                // [2-3]
                                CpuDifficulty::Normal => (game.turn as usize / 2) % 2 + 2,
                                // [5]
                                CpuDifficulty::Hard => 5,
                            } + (game.turn - 96).max(0) as usize / 10).min(5)
                        ).action_id.unwrap();
                        Logic(DoAction(action_id))
                    }else { input }
                },
            }
        }
    }

    fn _get_input(&mut self, game : &mut GraphicBoardGame, time : GameTime, c : &mut DefaultContext<Glob>) -> GraphicBoardGameInput
    {
        let mut cursor_pressed = false;
        let mut cursor_pos_vec : Option<Vec2> = None;
        
        for t in c.input.touch_just_pressed()
        {
            cursor_pos_vec = Some(t.position(&mut c.pen.cam).cur());
            cursor_pressed = true;
        }

        if cursor_pos_vec.is_none()
        { 
            cursor_pos_vec = Some(c.input.mouse().position(&mut c.pen.cam).cur());
            // drag & drop
            cursor_pressed = c.input.mouse().press().just_released() && c.input.mouse().delta_from_press(&mut c.pen.cam).length() >= 0.25;
        }

        
        use ActionID::*;
        use GraphicBoardGameInput::*;
        use GraphicActionID::*;
        use LogicActionID::*;

        match cursor_pos_vec
        {
            Some(dest_vec) => 
            {
                let dest_point2 = dest_vec.to_point2();
                let dest =  dest_point2.map(|e| e as AtIntType);

                match (cursor_pressed, game.piece_selector.selected_piece)
                {
                    (true, None) => 
                    {
                        if let Some(src) = game.piece_selector.hovered_piece
                        {
                            let action_id = Move(src, dest);
                            if game.action_id_is_valid(action_id)
                            {
                                return Logic(DoAction(Move(src, dest)));
                            }
                        }
                        return Graphic(Select(Some(dest)));
                    },
                    (true, Some(src)) => 
                    {
                        let action_id = ActionID::Move(src, dest);
                        if game.action_id_is_valid(action_id)
                        {
                            return Logic(DoAction(Move(src, dest)));
                        }
                        return Graphic(Select(Some(dest)));
                    },
                    (false, None) => 
                    {
                        if game.is_inside(dest)
                        {
                            match game.piece_selector.hovered_piece
                            {
                                Some(src) => 
                                {
                                    // Check if the player is following the movement of the current piece
                                    if game.piece_belong_to_current_team_turn(src)
                                    {
                                        for a in game.actions.iter_piece_action(src)
                                        {
                                            for sub_action in a.iter()
                                            {
                                                match sub_action
                                                {
                                                    UnitAction::Capture(_, d) => 
                                                    {
                                                        if dest == *d { return ___(); }
                                                    },
                                                    UnitAction::Swap(s, d) => 
                                                    {
                                                        if dest == *s || dest == *d { return ___(); }
                                                        
                                                        let delta = (*d - *s).to_vec2();

                                                        let radius = if game[dest].have_any_ability() { 0.5 } else { 0.9 };

                                                        let p = 5;
                                                        let mut c = 0.;
                                                        let inc = 1./p.to_real();
                                                        for i in 0..=p
                                                        {
                                                            if (dest_vec - Vec2::HALF - ((*s).to_vec2() + delta * c)).length() <= radius { return ___(); }
                                                            c += inc;
                                                        }
                                                    },
                                                    _ => {},
                                                }
                                            }
                                        }
                                    }else
                                    {
                                        //if !game[dest].have_any_ability() && (dest_vec - Vec2::HALF - dest.to_vec2()).length() <=  0.8 { return ___(); }

                                    }
                                },
                                None => {},
                            }
                            return Graphic(Hover(Some(dest)));
                        }
                    },
                    (false, Some(at)) => 
                    {
                        if game.is_inside(dest) && !game.are_friend(at, dest)
                        {
                            return Graphic(Hover(Some(dest)));
                        }

                    },
                }
            },
            None => {}
        }
        ___()
    }
}

impl IInputProvider<GraphicBoardGame> for GraphicBoardGameInputProvider
{
    fn get_input(&mut self, game : &mut GraphicBoardGame, time : GameTime, c : &mut DefaultContext<Glob>) -> GraphicBoardGameInput {
        self._input(game, time, c)
    }
}
