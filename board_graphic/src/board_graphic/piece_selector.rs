use std::marker::PhantomData;

use super::*;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct PieceSelector
{
    //hovered_piece  : Vec<Action>,

    pub hovered_piece  : Option<At>,
    pub hovered_team   : Option<Team>,
    pub selected_piece : Option<At>,

    pub avoid_hover_piece_at : Option<At>,

    pub action_piece_to_draw : Actions,
    pub action_team_to_draw : Actions,

    pub hover_piece_time : Time,
    pub hover_team_time : Time,

}

impl GraphicBoardGame
{
    pub fn piece_selector_post_update(&mut self, action_id : GraphicBoardGameInput, t : GameTime, ctx : &mut Context)
    {
        if action_id.is_nothings() { return; }
        match action_id
        {
            GraphicBoardGameInput::Logic(l) => { self.selector_update_team_action_to_draw(); },
            GraphicBoardGameInput::Graphic(_) => {},
        }
    }

    pub fn piece_selector_pre_update(&mut self, action_id : GraphicBoardGameInput, t : GameTime, ctx : &mut Context)
    {
        if action_id.is_nothings() { return; }

        let time = t.total();

        match action_id
        {
            GraphicBoardGameInput::Logic(l) => 
            {
                self.select_piece(None, time);
                self.hover_piece(None, time);

                self.piece_selector.action_piece_to_draw.clear();

                match l
                {
                    LogicActionID::DoAction(a) => 
                    {
                        match a
                        {
                            ActionID::Move(src, dest) => 
                            { 
                                self.piece_selector.avoid_hover_piece_at = Some(dest);
                            },
                        }
                    },
                    _ => { self.piece_selector.avoid_hover_piece_at = None; },
                }
            },
            GraphicBoardGameInput::Graphic(g) => 
            {
                match g
                {
                    GraphicActionID::Select(src) => 
                    { 
                        if src != self.piece_selector.selected_piece
                        {
                            match src
                            {
                                Some(at) => 
                                {
                                    if self.is_inside(at)
                                    {
                                        if self[at].have_flag(PieceFlags::CROWN) { ctx.audio.play(&ctx.globals.assets.sound.board.king.select); }
                                        if self[at].is_also_chess_queen() { ctx.audio.play(&ctx.globals.assets.sound.board.queen.selected); }
                                        else
                                        {
                                            if self[at].is_also_chess_bishop() { ctx.audio.play(&ctx.globals.assets.sound.board.bishop.select); }
                                            if self[at].is_also_chess_rook() { ctx.audio.play(&ctx.globals.assets.sound.board.rook.select); }
                                        }
                                        if self[at].is_also_chess_knight() { ctx.audio.play(&ctx.globals.assets.sound.board.knight.select); }
                                        if self[at].is_also_chess_pawn() { ctx.audio.play(&ctx.globals.assets.sound.board.pawn.select); }
                                    }else
                                    {
                                        ctx.audio.play(&ctx.globals.assets.sound.board.event.unselect);
                                    }
                                },
                                None => 
                                {
                                    ctx.audio.play(&ctx.globals.assets.sound.board.event.unselect);
                                },
                            }
                        }

                        //ctx.audio.play_with_volume(&ctx.globals.assets.sound.select_piece, 0.3);

                        //TODO
                        self.select_piece(src, time);
                    },
                    GraphicActionID::Hover(src) => 
                    { 
                        if src != self.piece_selector.avoid_hover_piece_at && src.map(|e| !self[e].is_empty_ability()).unwrap_or(false) && self.piece_selector.hovered_piece != src 
                        {
                            ctx.audio.play(&ctx.globals.assets.sound.board.event.hover);
                            self.hover_piece(src, time);
                        }
                    },
                    GraphicActionID::Nothings => {},
                    GraphicActionID::HoverTeam(t) => 
                    { 
                        self.hover_team(t, time, ctx);
                    },
                    GraphicActionID::HoverTeamToggle => 
                    {
                        let next_hovered_team = match self.piece_selector.hovered_team
                        {
                            Some(t) => 
                            {
                                let mut next_team = t;
                                for i in 0..Team::LENGHT
                                {
                                    next_team = next_team.next();
                                    if self.team_data[next_team].is(|e| e.is_present) { break; }
                                }
                                if next_team == self.current_team { None } else { Some(next_team) }
                            },
                            None => Some(self.current_team),
                        };
                        self.hover_team(next_hovered_team, time, ctx);
                    },
                }
            },
        }
    }

    fn selector_update_piece_action_to_draw(&mut self) 
    {
        self.piece_selector.action_piece_to_draw.clear();
        if let Some(selected) = self.piece_selector.selected_piece
        {
            let concerned_by_anticipation = self.concerned_by_relic_anticipation_at(selected);
            self.back_end.actions_piece(&mut self.piece_selector.action_piece_to_draw, selected, concerned_by_anticipation);
            //self.piece_selector.action_to_draw.append(&mut self.iter_piece_action(selected).map(|a| a.clone()).collect());
        }
        if let Some(hover) = self.piece_selector.hovered_piece
        {
            if self.piece_selector.hovered_piece != self.piece_selector.avoid_hover_piece_at
            {
                let concerned_by_anticipation = self.concerned_by_relic_anticipation_at(hover);
                self.back_end.actions_piece(&mut self.piece_selector.action_piece_to_draw, hover, concerned_by_anticipation);
                //self.piece_selector.action_to_draw.append(self.actions_piece(action_id))
                //self.piece_selector.action_to_draw.append(&mut  self.iter_piece_action(hover).map(|a| a.clone()).collect());
            }
        }
    }

    fn select_piece(&mut self, mut src : Option<At>, time : Time)
    {
        if let Some(s) = src
        {
            if !self.is_inside(s) || self[s].is_empty_ability() { src = None; }
        }

        if self.piece_selector.selected_piece.is_some() != src.is_some() && self.piece_selector.hovered_piece != src
        {
            self.piece_selector.hover_piece_time = time;
        }
        self._hover_piece(None);


        self.piece_selector.selected_piece = src;
        //if reset_timer { self.hover_time = time.total; }
        self.selector_update_piece_action_to_draw();
    }


    fn _hover_piece(&mut self, src : Option<At>)
    {
        self.piece_selector.hovered_piece = src;
        self.selector_update_piece_action_to_draw();
    }

    fn hover_team(&mut self, team : Option<Team>, time : Time, ctx : &mut Context)
    {
        if team != self.piece_selector.hovered_team && team.is_some()
        {
            ctx.audio.play(&ctx.globals.assets.sound.board.event.hover_team);
            self.piece_selector.hover_team_time = time;
        }
        self.piece_selector.hovered_piece = None;
        self.piece_selector.hovered_team = team;
        self.selector_update_piece_action_to_draw();
        self.selector_update_team_action_to_draw();
    }
    fn selector_update_team_action_to_draw(&mut self) 
    {
        self.piece_selector.action_team_to_draw.clear();
        match self.piece_selector.hovered_team
        {
            Some(t) => 
            {
                for hover in self.back_end.iter_idx_team(t)
                {
                    let concerned_by_anticipation = self.concerned_by_relic_anticipation_at(hover);
                    self.back_end.actions_piece(&mut self.piece_selector.action_team_to_draw, hover, concerned_by_anticipation);
                }
            },
            None => {},
        }
    }
    
    fn hover_piece(&mut self, mut src : Option<At>, time : Time)
    {
        if let Some(s) = src
        {
            if !self.is_inside(s) || self[s].is_empty_ability() { src = None; }
        }
        if src == self.piece_selector.avoid_hover_piece_at
        {
            src = None;
        }else
        {
            self.piece_selector.avoid_hover_piece_at = None;
        }
 
        if self.piece_selector.hovered_piece.is_some() != src.is_some() && self.piece_selector.selected_piece.is_none()
        {
            self.piece_selector.hover_piece_time = time;
            //self.piece_selector.hovered_team = None;
        }
        self._hover_piece(src);
    }

}


impl GraphicBoardGame
{
    pub fn scale_time_effect(&self, time : Time, selector_time : Time) -> real
    {
        ((time - selector_time).s().max(0.)/0.25).powf(2.).min(1.)
    }

    pub fn color_tile_effect(&self, time : Time, selector_time : Time, c : Color, _p : At) -> Color
    {
        let d = (time-selector_time).s() / 4.;
        let t = Angle::from_turn(d).sin();
        let sign = t.signum();
        let v = t.abs().powf(3.) * sign;
        Color::from_vec3_coef(c.to_vec3_coef() * (1. - v.abs() * 0.22))
    }

    /// Darker color
    pub fn color_last_moved_src(&self, at : At) -> Color
    {
        Color::from_rgb_hex(0xFFD400)
        /* 
        if at.length_manhattan() % 2 == 0 
        {
            Color::from_rgb_hex(0x114433)
        }else
        {
            Color::from_rgb_hex(0x57663B)
        }*/
    }

    /// Lighter color
    pub fn color_last_moved_dest(&self, at : At) -> Color
    {
        self.color_last_moved_src(at)
        /* 
        if at.length_manhattan() % 2 == 0 
        {
            Color::from_rgb_hex(0x3F7A60)
            //Color::from_rgb_hex(0x23664A)
        }else
        {
            Color::from_rgb_hex(0x829954)
        }*/
    }

    pub fn draw_selector(&self, time : Time, ctx : &mut Context)
    {
        let (input, pen) = (&mut ctx.input, &mut ctx.pen);

        let mut color_capture = Color::from_rgb_hex(0xFF4C6A);
        let mut color_dest = Color::from_rgb_hex(0x00BFFF);

        if self.is_end_of_the_game()
        {
            color_capture = color_capture.lerp(Color::BLACK, 0.5);
            color_dest    = color_dest.lerp(Color::BLACK, 0.5);
        }

        let color_select = color_capture;

        let color_select_enemy: Colored<f32> = color_capture;
        let color_dest_enemy = color_capture;


        let intro_time = 0.5;


        //self.draw_selected_tile(at, WHITE, time, pen);
        //let p = &self[at];

        // Last Moved Piece Tile
        if self.turn > 0
        {
            for at in self.iter_idx().filter(|e| self.was_moved_last_turn_or_this_turn(*e) && !self[*e].is_empty_ability())
            {
                self.draw_tile(self[at].old_pos, self.color_last_moved_src(self[at].old_pos), pen);
            }

            for at in self.iter_idx().filter(|e| self.was_moved_last_turn_or_this_turn(*e) && !self[*e].is_empty_ability())
            {
                self.draw_tile(at, self.color_last_moved_dest(at), pen);
            }
        }

        #[cfg(feature = "chantal_design")]
        {
            // Board coordinate
            let font_scale = 0.4;
            for x in 0..self.size().x
            {
                let n = format!("{}", char::from_u32('A' as u32 + x as u32).unwrap());
                //pen.text(&*n, vec2(x.to_real(), self.size().y.to_real()), 0.5, Vec2::new(0., 1.), BLACK, ___());        
                pen.text(&*n, vec2(x.to_real() + 0.5, 0.), font_scale, Vec2::new(0.5, 1.), Color::BLACK, ___());        
            }
            for y in 0..self.size().x
            {
                let n = format!("{}", char::from_u32('1' as u32 + y as u32).unwrap());
                pen.text(&*n, vec2(self.size().x.to_real(), y.to_real()+0.5), font_scale, Vec2::new(0., 0.5), Color::BLACK, ___());        
            }
        }
        #[cfg(not(feature = "chantal_design"))]
        {
            // Board coordinate
            let font_scale = 0.4;
            for x in 0..self.size().x
            {
                let n = format!("{}", char::from_u32('A' as u32 + x as u32).unwrap());
                //pen.text(&*n, vec2(x.to_real(), self.size().y.to_real()), 0.5, Vec2::new(0., 1.), BLACK, ___());        
                pen.text(&*n, vec2(x.to_real(), 0.), font_scale, Vec2::new(0., 0.), Color::BLACK, ___());        
            }
            for y in 0..self.size().x
            {
                let n = format!("{}", char::from_u32('1' as u32 + y as u32).unwrap());
                pen.text(&*n, vec2(self.size().x.to_real(), y.to_real()+1.), font_scale, Vec2::new(1., 1.), Color::BLACK, ___());        
            }
        }




        let time_effect = self.scale_time_effect(time, self.piece_selector.hover_piece_time);

        // Selected Tile
        if let Some(at) = self.piece_selector.selected_piece
        {
            let (color_select, color_dest) = if self.piece_belong_to_current_team_turn(at) { (color_select, color_dest) } else { (color_select_enemy, color_dest_enemy) };
            pen.circle(at.to_vec() + Vec2::HALF, 0.5 * time_effect, color_dest);
        }

        // Hovered Tile
        for at in self.piece_selector.hovered_piece.iter().copied().filter(|a| Some(*a) != self.piece_selector.selected_piece)
        {
            let (color_select, color_dest) = if self.piece_belong_to_current_team_turn(at) { (color_select, color_dest) } else { (color_select_enemy, color_dest_enemy) };
            pen.rectangle(at.to_vec() + Vec2::HALF, Vec2::ONE * time_effect, Vec2::HALF,  zero(), color_dest);
        }

        for (idx, (actions, selector_time)) in [(&self.piece_selector.action_team_to_draw, self.piece_selector.hover_team_time), (&self.piece_selector.action_piece_to_draw, self.piece_selector.hover_piece_time)].into_iter().enumerate()
        {
            let line_tickness = 1. / if idx == 0 { 10. } else { 4. };
            let move_dest_radius = 1. / if idx == 0 { 10. } else { 4. };

            let time_effect = self.scale_time_effect(time, selector_time);

            // En passant
            for m in actions.iter().flat_map(|e| e.iter())
            {
                match m
                {
                    UnitAction::Capture(src, dest) /*if src.is_none()*/ => self.draw_tile(*dest, color_capture, pen),
                    _ => {}
                };
            }

            // Attacking line
            for s in actions.iter().filter(|m| m.iter().any(|e| e.is_capture()))
            {
                let (color_select, color_dest) = if self.current_team.belong(s.team) { (color_select, color_dest) } else { (color_select_enemy, color_dest_enemy) };

                for s in s.iter()
                {
                    match s
                    {
                        UnitAction::Swap(src, dest) => 
                        {
                            let c = self.color_tile_effect(time, selector_time, color_capture, *dest);
                            pen.straight_line(src.to_vec2() + Vec2::HALF, dest.to_vec2() + Vec2::HALF, line_tickness * time_effect, c);
                            //pen.circle(m.src.to_vec2() + Vec2::HALF, line_tickness * time_effect,c);
                        },
                        _ => {}
                    };
                }
            }

            // Non attacking line
            for s in actions.iter().filter(|m| !m.iter().any(|e| e.is_capture()))
            {
                let (color_select, color_dest) = if self.current_team.belong(s.team) { (color_select, color_dest) } else { (color_select_enemy, color_dest_enemy) };

                for s in s.iter()
                {
                    match s
                    {
                        UnitAction::Swap(src, dest) => 
                        {
                            let c = self.color_tile_effect(time, selector_time, color_dest, *dest);
                            pen.straight_line(src.to_vec2() + Vec2::HALF, dest.to_vec2() + Vec2::HALF, line_tickness * time_effect, c);
                            //pen.circle(m.src.to_vec2() + Vec2::HALF, line_tickness * time_effect,c);
                        },
                        _ => {}
                    };
                }
            }

            // Circle Pos
            for s in actions.iter()
            {
                let (color_select, color_dest) = if self.current_team.belong(s.team) { (color_select, color_dest) } else { (color_select_enemy, color_dest_enemy) };

                match &s.id
                {
                    ActionID::Move(src, dest) => 
                    {
                        let mut highlight_capture = 0.;

                        let color =  if self.are_capturable_enemy(*src, *dest) { highlight_capture += 50.; color_capture } else { color_dest };
                        let dest_pos = dest.to_vec2() + Vec2::HALF;
                        let c = self.color_tile_effect(time, selector_time, color, *dest);
                        //pen.circle(dest_pos, line_tickness / 2. * 1.1, if dest.dest.lenght_manhattan() % 2 == 0 { BLACK } else { WHITE });
        
                        let co = (*dest- *src).absolute().max_element().to_real();
        
                        let sinus = (Angle::from_turn((selector_time - time).s()/2.+co/(2.5*2.))).sin();
                        let t = (sinus * 10. + highlight_capture) / 200.;
                        
                        
                        //pen.circle(dest_pos, line_tickness / 2. * 1.75, c);
                        //pen.circle(dest_pos, line_tickness / 2. * (1.75 - t + 1.), c);
                        pen.circle(dest_pos, (move_dest_radius + t) * time_effect, c.lerp(Color::BLACK, 0.3).with_a(1.));
                    },
                }

                /* 
                for s in s.iter()
                {
                    match s
                    {
                        UnitAction::Swap(src, dest) => 
                        {

                        },
                        _ => {}
                    };
                }*/
            }
        }
    }
}