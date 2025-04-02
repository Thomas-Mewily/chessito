use std::{default, ops::{Div, Mul}};

use super::*;


pub enum PieceGraphicRep
{
    Cburmeet,
    DejaView,
    Chantal,
}

impl GraphicBoardGame
{
    //pub fn hud_top_margin
    /*
    pub fn ui_hud_height(&self) -> UiNumber { Coef::ui_axis(0.075) }
    */
    pub fn cam_hud_top_begin(&self, c : &mut Context) 
    {
        c.pen.cam.push();
        //c.pen.cam.set_world(rect2(0., 0., 16., 1.));
        //c.pen.cam.add_margin_bot(Coef::ui_axis(0.925));
            //c.pen.cam.glue_top(self.ui_hud_height());
        //c.pen.cam.glue_right(Coef::ui_y_axis(0.075));
        //c.pen.cam.glue_to_top();
        //c.pen.cam.current().
        //c.pen.cam.set_world_target(Some(Rect2::new(Vec2::ZERO, vec2(10., 1.))));
        //c.pen.cam.add_margin_down(Coef::ui_min(0.10));
        //c.pen.cam.add_margin_top(Coef::ui_min(0.10));
        c.pen.cam.apply();
    } 

    pub fn cam_hud_top_end(&self, c : &mut Context) 
    {
        c.pen.cam.pop();
    }

    pub fn cam_board_begin(&self, c : &mut Context) 
    {
        //self.cam_hud_top_begin(c);
        
        //let m = (Angle::from_turn(c.tick_time.total.s()/3.).cos().abs() / 5.).ui_min();
        //c.pen.cam.push_bot();

        let cam = &mut c.pen.cam;

        cam.push();

            //cam.add_margin_top(self.ui_hud_height());
        //c.pen.cam.add_margin(Coef::ui_min(0.05));
        //c.pen.cam.add_margin_left_px(10.);
        //c.pen.cam.current_mut().unused_coef_center.x = 0.;
        //c.pen.cam.add_margin_left_px(10.);
        //c.pen.cam.add_margin_top(0.5.ui_min()).set_area(Rect2::new(Vec2::ZERO, self.size())).apply();
        /* 
        c.pen.cam.add_margin_top(m)
            .add_margin_bot(m)
            .set_area(Rect2::new(Vec2::ZERO, self.size())).apply();
        */
        //c.pen.cam.add_margin(Coef::ui_min(0.05));
        //c.pen.cam.add_margin_top_px(40.);
        //c.pen.cam.add_margin_bot_px(40.);
        //c.pen.cam.add_margin_right_px(40.);
        //c.pen.cam.apply();

        //c.pen.cam.push();
        //c.pen.cam.add_margin(Coef::ui_min(0.05));

        //c.pen.cam.current_mut().container_px.pos.y += 20.;

        cam.set_world(self.board_rect_with_edge());

        //cam.set_rect_px(self.board_rect_with_edge());
        cam.apply();
        //c.pen.cam.add_margin_bot((Angle::from_turn(c.tick_time.total.s()/3.).cos().abs() * 0.25).ui_min()).set_area(Rect2::new(Vec2::ZERO, self.size())).apply();
    }

    pub fn cam_board_end(&self, c : &mut Context) 
    {
        c.pen.cam.pop();
        //self.cam_hud_top_end(c);
        //c.pen.cam.pop();
    }

    #[cfg(feature = "chantal_design")]   
    pub fn color_tile_dark(&self) -> Color { Color::from_rgb_hex(0x8592AF) }
    #[cfg(feature = "chantal_design")]   
    pub fn color_tile_light(&self) -> Color { Color::from_rgb_hex(0xE9EBDE) }


    #[cfg(not(feature = "chantal_design"))]
    pub fn color_tile_dark(&self) -> Color { Color::from_rgb_hex(0x007F15) }
    #[cfg(not(feature = "chantal_design"))]
    pub fn color_tile_light(&self) -> Color { Color::from_rgb_hex(0x64BC25) }

    pub fn tile_color_at(&self, at : At) -> Color { if at.length_manhattan() % 2 == 0 { self.color_tile_dark() } else { self.color_tile_light() }}

    pub fn draw_tile(&self, pos : At, color : Color, pen : &mut ContextPen) { self.draw_tile_with_size(pos, color, Vec2::ONE, pen); }
    pub fn draw_tile_with_size(&self, pos : At , color : Color, size : Vec2, pen : &mut ContextPen)
    {
        pen.rectangle(pos.to_vec() + Vec2::HALF, size, Vec2::HALF, zero(), color);
    }

    // Todo : change the color over time
    pub fn team_flags_to_color(&self, t : TeamsFlags, c : &mut Context) -> Color
    {
        let nb_team = t.count();
        if nb_team == 0 { return Color::BLACK; }
        let coef = 1. / nb_team.to_real();

        let mut color = Color::ZERO;
        for c in t.iter_team().map(|t| self.team_to_color(t, c))
        {
            color = c;
            //color += c * coef;
        }
        color
    }
    pub fn team_to_color(&self, t : Team, c : &mut Context) -> Color
    {
        match t
        {
            Team::White  => Color::from_rgb_hex(0x99EEFF),
            Team::Black  => Color::from_rgb_hex(0xFF99AB),
            Team::Yellow => Color::from_rgb_hex(0xFFE566),
            Team::Green  => Color::from_rgb_hex(0x99FF9F),
        }
    }

    
    #[cfg(not(feature = "chantal_design"))]
    pub fn edge(&self) -> real { 0.15 }

    #[cfg(feature = "chantal_design")]   
    pub fn edge(&self) -> real { 0.3 }

    pub fn board_rect_with_edge(&self) -> Rect2
    {
        let edge_vec = Vec2::splat(self.edge());
        Rect2::new(-edge_vec, self.size().to_vec2() + edge_vec * 2.)
        //Rect2::new(Vec2::ZERO, self.size().to_vec2() + edge_vec * 2.)
    }

    pub fn draw_board_tile(&self, ctx : &mut Context) 
    {
        let s = self.board_rect_with_edge();
        ctx.pen.rectangle(s.pos, s.size, Vec2::ZERO, zero(), self.board_edge_color);

        for p in self.size().iter_area()
        {
            self.draw_tile(p, self.tile_color_at(p), &mut ctx.pen);
        }
    }

    pub fn draw_board_piece_color(&self, c : &mut Context) 
    {
        /* 
        for p in self.size().iter_area()
        {
            
            let t : Vec<Color> = self[p].iter_team().map(|t| Self::team_to_color(t, c)).collect();

            if t.len() == 0 { continue; }

            let mut x_begin = 0.0;
            let x_size = 1. / t.len().to_real();
            for color in t.iter().copied()
            {
                //let color = color.with_a(0.25);
                c.pen.rectangle(p.to_vec() + vec2(x_begin, 0.), vec2(x_size, 1.), Vec2::ZERO, zero(), color);
                x_begin += x_size;
            } 
        }*/
    }

    pub fn display_piece_at(&self, at : At, pos : Vec2, time : Time, ctx : &mut Context) { self.display_piece(self[at], pos, one(), ___(), time, ctx);}
    pub fn display_piece(&self, p : Piece, pos : Vec2, size : Vec2, angle : Angle, time : Time, ctx : &mut Context)
    {

        #[cfg(feature = "chantal_design")]   
        let piece_display_kind = PieceGraphicRep::Chantal;

        #[cfg(not(feature = "chantal_design"))]
        let piece_display_kind = PieceGraphicRep::Cburmeet;

        let mut textures_y : [isize; 8] = [0; 8];
        let mut textures_len = 0;

        // Todo : handle different move set
        if p.is_also_chess_pawn  () { textures_y[textures_len] = 6; textures_len+=1; }
        if p.is_also_chess_knight() { textures_y[textures_len] = 4; textures_len+=1; }
        // let this one here because it override rook and bishop
        if p.is_also_chess_queen () { textures_y[textures_len] = 2; textures_len+=1; }
        else
        {
            if p.is_also_chess_bishop() { textures_y[textures_len] = 5; textures_len+=1; }
            if p.is_also_chess_rook  () { textures_y[textures_len] = 3; textures_len+=1; }
        }

        if p.is_also_dame_pawn   () { textures_y[textures_len] = 7; textures_len+=1; }
        if p.is_also_dame_king   () { textures_y[textures_len] = 8; textures_len+=1; }

        if p.is_also_chess_king  () { textures_y[textures_len] = 1; textures_len+=1; }

        let mut texture_y : isize = 1;
        let mut texture_y_next = 1;

        if textures_len == 0 { return; }

        let t = time.s() * 1.2;
        let coef = Easing::quart_in_out().apply(t % 1.);
        let rcoef = 1. - coef;

        if !p.have_flag(PieceFlags::CROWN)
        {
            let time_isize = t as isize;
            texture_y_next = textures_y[(time_isize+1) as usize % textures_len];
            texture_y = textures_y[time_isize as usize % textures_len];
        }

        let mut texture_x = 0;
        if p.is_also_team(Team::White) { texture_x+=1; }
        if p.is_also_team(Team::Black) { texture_x+=2; }

        let texture = match piece_display_kind
        {
            PieceGraphicRep::Cburmeet  => &ctx.globals.assets.img.piece.cburnett,
            PieceGraphicRep::DejaView  => &ctx.globals.assets.img.piece.deja_view,
            PieceGraphicRep::Chantal => &ctx.globals.assets.img.piece.chantal,
        };

        let mut rec1 = texture.sheet_rect_from_point2(point2(texture_x, texture_y));

        let color = Color::WHITE; // self.team_flags_to_color(self[at].teams_flags(), c);

        let pos = pos.to_vec() + vec2(0.5, 0.5);
        let center = vec2(0.5, 0.5);

        let in_game_size : Vec2 = size;


        if textures_len == 1
        {
            ctx.pen.texture(texture, pos, in_game_size, center, DrawTexture::default().with_source(Some(rec1)).with_color(color).with_angle(angle));
        }else
        {
            let mut rec2 = texture.sheet_rect_from_point2(point2(texture_x, texture_y_next));

            rec1.add_margin_right(rec1.width() * coef);
            rec2.add_margin_left(rec2.width() * rcoef);

            let s1 = in_game_size - vec2(coef*in_game_size.x, 0.);
            ctx.pen.texture(texture, pos - vec2((in_game_size.x-s1.x) * 0.5, 0.), s1, center, DrawTexture::default().with_source(Some(rec1)).with_color(color));
            
            let s2 = in_game_size - vec2(rcoef*in_game_size.x, 0.);
            ctx.pen.texture(texture, pos + vec2((in_game_size.x-s2.x) * 0.5, 0.), s2, center, DrawTexture::default().with_source(Some(rec2)).with_color(color));
        }

        //ctx.pen.texture(texture, pos, in_game_size, center, DrawTexture::default().with_source(Some(rec2)).with_color(color).with_angle(angle));
    
        if p.flags.have_flag(PieceFlags::PARTIAL_PIN)
        {
            let r = texture.sheet_rect_from_point2(point2(7, 1));
            ctx.pen.texture(texture, pos, in_game_size, center, DrawTexture::default().with_source(Some(r)).with_color(color).with_angle(angle));
        }
        if p.flags.have_flag(PieceFlags::TOTAL_PIN)
        {
            let r = texture.sheet_rect_from_point2(point2(7, 2));
            ctx.pen.texture(texture, pos, in_game_size, center, DrawTexture::default().with_source(Some(r)).with_color(color).with_angle(angle));
        }
    }

    pub fn draw_piece_position(&self, at : At, time : GameTime, c : &mut Context) -> Vec2
    {
        let new = at.to_vec2();
        let mut pos = if self.was_moved_last_action_or_this_action(at)
        {
            let old = self[at].old_pos.to_vec2();
            let tile_per_second = 10.;
            let coef = tile_per_second * time.elapsed_since_last_input().s() / (new-old).length();

            old.ease(new, coef, Easing::cubic_out())
        }else
        {
            new
        };


        if self.is_end_of_the_game()
        {
            let offset_coef = (new.x * 1. + new.y + self[at].flags.0.to_real()) / 8.0 % 2.;
            //let offset_coef = (self[at].flags.0.to_real()) / 8.0 % 2.;
            
            let mut won_animation = false;
            if !self.is_draw
            {
                for t in self[at].iter_team()
                {
                    won_animation |= self.team_data[t].alive;
                }
            }

            if won_animation
            {
                pos.y += ((time.elapsed_since_last_input().s() - offset_coef).max(0.) * 1.).turn().sin().abs() * 0.4;
                return pos;
            }

            // lose or draw
            pos.x += ((time.elapsed_since_last_input().s() - offset_coef).max(0.) * 3.).turn().sin() * 0.025;
        }

        pos

    } 

    pub fn draw_board_piece(&self, time : GameTime, c : &mut Context) 
    {
        for y in (0..self.size().y).rev()
        {
            for x in 0..self.size().x
            {
                let draw_pos = at(x,y);
                self.display_piece_at(draw_pos, self.draw_piece_position(draw_pos, time, c), time.total(), c);

                /* 
                let dest = p.to_vec();
                let src = self[p].draw.gpos;
                let delta = dest - src;
                */
                //self.display_piece(p, src + delta * time.coef, c);
            }
        }
    }

    pub fn draw_end_message(&self, time : GameTime, ctx : &mut DefaultContext<Glob>)
    {
        if !self.is_end_of_the_game() { return;}

        let middle_pos = self.size().to_vec2() / 2.;

        let (msg, mut color) = if self.is_draw
        {
            ("Draw !".to_owned(), Color::CYAN)
        }else
        {
            let (mut msg, mut color) = ("".to_owned(), ColorHSL::new_hsl((time.total().s() / 8.) % 1., 1., 0.5).to_rgb());

            for t in Team::iter()
            {
                if self.team_data[t].is_present && !self.team_data[t].alive && self.players[t as usize].is_human()
                {
                    msg = format!("Defeat !");
                    color = Color::RED;
                }
            }

            for t in Team::iter()
            {
                if self.team_data[t].alive && self.players[t as usize].is_human()
                {
                    msg = format!("{} Victory !", t);
                }
            }

            (msg, color)
        };
        color.a = 0.5 + time.elapsed_since_last_input().s().div(6.).turn().sin().abs().powf(2.)*0.5;
        ctx.pen.text(&msg, middle_pos, 1., half(), color, ___());
    }


    pub fn get_piece_captured_particle(&self, src : Option<At>, dest : At, time : GameTime) -> ParticlePieceCaptured
    {
        let v = if let Some(src) = src { dest.to_vec2() - src.to_vec2() } else { vec2((dest.x().to_real()-self.size().x.to_real()/2.).signum() * 3., 2.) };
        ParticlePieceCaptured { piece : self[dest], speed: v * 1., base : ParticleBase { pos: dest.to_vec2(), spawn: time.total() }, size : one() }
    }

    pub fn particle_captured_life_time() -> Time { 10.s() }
    pub fn particle_explosion_life_time() -> Time { 0.75.s() }
    
    pub fn update_particle(&mut self, time : GameTime)
    {
        let particle_life_time = Self::particle_captured_life_time();
        self.particles_captured_pieces.retain(|e| (time.total()-e.spawn) <= particle_life_time);

        for c in self.particles_captured_pieces.iter_mut()
        {
            let t = time.total()-c.spawn;
            if t >= particle_life_time { continue; }

            let coef = t.s() / particle_life_time.s();

            let src : Vec2 = 1.0.splat2();
            let dest : Vec2 = 3.0.splat2();
            c.size = src.ease(dest, coef*2., Easing::quart_out());

            if t.s() >= 0.2
            {
                c.base.pos += c.speed * time.delta().s();
                c.speed *= (2.0 as real).powf(time.delta().s());
                c.speed.y -= time.delta().s() * 3.;
                
                //c.speed = c.speed * c.speed.map(|e| e.powf(time.delta().s()));
            }
        }

        self.particles_tile_explosion.retain(|e| (time.total()-e.spawn) <= Self::particle_explosion_life_time());

    }

    pub fn draw_tile_explosion_particle(&self, time : GameTime, ctx : &mut DefaultContext<Glob>)
    {
        for p in self.particles_tile_explosion.iter()
        {
            let time_coef = (time.total()-p.spawn) / Self::particle_explosion_life_time();

            let radius = 0.6.ease(0., time_coef, Easing::back_in_out());

            ctx.pen.circle(p.pos, radius, Color::from_rgb_hex(0xFF4C6A));
        }
    }

    pub fn draw_captured_piece_particle(&self, time : GameTime, ctx : &mut DefaultContext<Glob>)
    {
        for c in self.particles_captured_pieces.iter()
        {
            self.display_piece(c.piece, c.pos, c.size, (time.total()-c.spawn).s().div(2.).turn(), time.total(), ctx);
        }
    }

    pub fn draw_captured_piece_side(&self, time : GameTime, ctx : &mut DefaultContext<Glob>)
    {
        let nb_captured = self.captured.len();
        if nb_captured == 0 { return; }

        let nb_captured_real = nb_captured.to_real();

        let cam = &mut ctx.pen.cam;

        //cam.push();
        //cam.set_world(Rect2::new_zero_pos(vec2(8., 0.)));

        //cam.apply();

        let area = cam.parent_size_px();
        

        let (delta, mut pos) = if area.x >= area.y
        {
            let offset = area.x / nb_captured_real * 0.5;
            let size_x = area.x - if nb_captured == 1 { 0. } else { offset * 2. };
            (vec2(size_x / (nb_captured_real - 1.0).max(1.), 0.), vec2(offset, area.y * 0.5))
        }else
        {
            let offset = area.y / nb_captured_real * 0.5;
            let size_y = area.y - if nb_captured == 1 { 0. } else { offset * 2. };
            (vec2(0., size_y / (nb_captured_real - 1.0).max(1.)), vec2(area.x * 0.5, offset))
        };

        let size = delta.max_element().min(area.min_element().mul(0.75)).splat2();
        //let size = area.min_element().mul(0.75).splat2();

        pos += cam.parent_pos_px();

        for (idx, captured) in self.captured.iter().enumerate()
        {
            self.display_piece(*captured, pos, size, zero(), time.total(), ctx);
            pos += delta;
        }

        //ctx.pen.cam.pop();
    }

    pub fn _draw(&self, time : GameTime, ctx : &mut DefaultContext<Glob>)
    {
        /* 
        self.cam_hud_top_begin(c);
        c.pen.fill_world_background(CYAN);
        self.cam_hud_top_end(c);
        */


        self.cam_board_begin(ctx);
        {
            ctx.pen.push_font(&ctx.globals.assets.img.ui.font.stanberry);
            self.draw_board_tile(ctx);
            self.draw_selector(time.total(), ctx);
            //self.draw_board_piece_color(c);
            self.draw_tile_explosion_particle(time, ctx);
            self.draw_board_piece(time, ctx);
            self.draw_end_message(time, ctx);
            self.draw_captured_piece_particle(time, ctx);

            ctx.pen.pop_font();
        }
        self.cam_board_end(ctx);


        /* 
        for lines in include_str!("../../credit.txt").lines()
        {
            c.pen.debug_text(lines.to_owned());
        }
        c.pen.debug(&c.perf);
        c.pen.debug(&c.input);
        c.pen.debug(&c.cam().window_size_px());
        //c.pen.debug(&c.input.key_pressed().collect::<Vec<&KeyCode>>());
        //c.pen.debug(&c.input.keyboard.pressed);
        c.pen.debug_text(format!("{} just pressed", c.input.touch_just_pressed().count()));
        c.pen.debug(&self.ai);

        //c.pen.nine_slice_at(&c.globals.assets.hud.nine_slice, )

        c.pen.debug_text(format!("white piece value {}, black piece value {}", self.team_data[Team::Blue].alive_piece_value, self.team_data[Team::Red].alive_piece_value));
        c.pen.debug_text(format!("{:?}", self.end_game_result()));*/
    }
}