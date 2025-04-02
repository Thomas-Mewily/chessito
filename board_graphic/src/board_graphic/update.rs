use std::default;

use super::*;

impl GraphicBoardGame
{
    pub fn _update(&mut self, input : GraphicBoardGameInput, time : GameTime, ctx : &mut DefaultContext<Glob>) -> Option<BoardResult>
    {
        self.pre_update(input, time, ctx);

        let result = match input
        {
            GraphicBoardGameInput::Logic(l) => self.update_logic(l, time, ctx),
            GraphicBoardGameInput::Graphic(g) => { self.update_graphic(g, time, ctx); None },
        };

        self.post_update(input, time, ctx);

        self.update_particle(time);

        result
    }

    pub fn pre_update(&mut self, input : GraphicBoardGameInput, time : GameTime, ctx : &mut DefaultContext<Glob>)
    {
        self.piece_selector_pre_update(input, time, ctx);
    }

    pub fn post_update(&mut self, input : GraphicBoardGameInput, time : GameTime, ctx : &mut DefaultContext<Glob>)
    {
        self.piece_selector_post_update(input, time, ctx);
    }


    pub fn execute(&mut self, action_id : ActionID)
    {
        //self.clear_pin();
        self.back_end.execute(action_id);
        //self.relic_perception_apply();
    }

    pub fn update_logic(&mut self, action_id : LogicActionID, time : GameTime, ctx : &mut DefaultContext<Glob>) -> Option<BoardResult>
    {
        match action_id
        {
            LogicActionID::DoAction(action_id) => 
            {
                assert!(self.action_id_is_valid(action_id), "illegal action {}", action_id);

                let action = self.get_action_from_action_id(action_id).clone();

                // playing some sound
                for a in action.iter()
                {
                    match a
                    {
                        UnitAction::Capture(src, dest) => 
                        {
                            let piece = self[*dest];
                            if piece.is_also_chess_king() { ctx.audio.play(&ctx.globals.assets.sound.board.king.captured); }
                            if piece.is_also_chess_queen() { ctx.audio.play(&ctx.globals.assets.sound.board.queen.captured); }
                            else
                            {
                                if piece.is_also_chess_rook() { ctx.audio.play(&ctx.globals.assets.sound.board.rook.captured); }
                                if piece.is_also_chess_bishop() { ctx.audio.play(&ctx.globals.assets.sound.board.bishop.captured); }
                            }
                            if piece.is_also_chess_pawn() { ctx.audio.play(&ctx.globals.assets.sound.board.pawn.captured); }
                            if piece.is_also_chess_knight() { ctx.audio.play(&ctx.globals.assets.sound.board.knight.captured); }
                        
                            self.particles_captured_pieces.push(self.get_piece_captured_particle(*src, *dest, time));
                            self.particles_tile_explosion.push(ParticleBase { spawn: time.total(), pos: dest.to_vec() + Vec2::HALF });
                        },
                        UnitAction::Swap(src, dest) => 
                        {
                            let piece = &self[*src];
                            if piece.is_also_chess_king() { ctx.audio.play(&ctx.globals.assets.sound.board.king.moving); }
                            if piece.is_also_chess_queen() { ctx.audio.play(&ctx.globals.assets.sound.board.queen.moving); }
                            else
                            {
                                if piece.is_also_chess_rook() { ctx.audio.play(&ctx.globals.assets.sound.board.rook.moving); }
                                if piece.is_also_chess_bishop() { ctx.audio.play(&ctx.globals.assets.sound.board.bishop.moving); }
                            }
                            if piece.is_also_chess_pawn() { ctx.audio.play(&ctx.globals.assets.sound.board.pawn.moving); }
                            if piece.is_also_chess_knight() { ctx.audio.play(&ctx.globals.assets.sound.board.knight.moving); }
                        },
                        UnitAction::EnergyAdd(_) => {},
                        UnitAction::Promote(_) => 
                        {
                            ctx.audio.play(&ctx.globals.assets.sound.board.promotion);
                        },
                    }
                }

                
                self.execute(action_id);
            },
            LogicActionID::Undo => 
            { 
                ctx.audio.play(&ctx.globals.assets.sound.ui.undo);
                self.undo();
                for i in 0..Team::LENGHT - 1
                {
                    if !self.players[self.current_team as usize].is_human()
                    {
                        self.undo();
                    }
                }
            },
            LogicActionID::Redo => 
            { 
                ctx.audio.play(&ctx.globals.assets.sound.ui.redo);
                self.redo();
                for i in 0..Team::LENGHT - 1
                {
                    if !self.players[self.current_team as usize].is_human()
                    {
                        self.redo();
                    }
                }
            },
        }

        let r = self.end_game_result();
        if let Some(val) = &r
        {
            match val
            {
                BoardResult::WinnerIs(team) => 
                {
                    match self.players[*team as usize]
                    {
                        PlayerKind::Human => { ctx.audio.play(&ctx.globals.assets.sound.board.event.victory); },
                        PlayerKind::Cpu(ordi) => { ctx.audio.play(&ctx.globals.assets.sound.board.event.defeat); },
                    }
                },
                BoardResult::Draw => { ctx.audio.play(&ctx.globals.assets.sound.board.event.draw); },
            }
        }
        r
    }

    pub fn update_graphic(&mut self, action_id : GraphicActionID, time : GameTime, ctx : &mut DefaultContext<Glob>)
    {

    }
}