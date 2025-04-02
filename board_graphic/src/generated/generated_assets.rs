///! This code was generated automatically by macro_game_asset
///! DO NOT EDIT IT BECAUSE IT WILL BE ERASED
///! Thank, Thomas Mewily

use game_engine::context::*;
use game_engine::wrapper::*;

pub struct AssetAssets {
   pub img : AssetImgId2,
   pub sound : AssetSoundId14,
   
}


impl AssetAssets {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         img : AssetImgId2::load(a).await,
         sound : AssetSoundId14::load(a).await,
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("img :".to_owned()); self.img.get_credits(s);
      s.push("sound :".to_owned()); self.sound.get_credits(s);
      
      }
   
}


pub struct AssetImgId2 {
   pub icon : AssetIconId4,
   pub piece : AssetPieceId6,
   pub relics : Texture2D,
   pub ui : AssetUiId8,
   
}


impl AssetImgId2 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         icon : AssetIconId4::load(a).await,
         piece : AssetPieceId6::load(a).await,
         relics : a.load_texture2d(r"./board_graphic/assets\img\relics.px(256).margin(1).credits(Mewily).png").await.tag_add_px(256).tag_add_margin(1).tag_add_credit("(Mewily)"),
         ui : AssetUiId8::load(a).await,
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("icon :".to_owned()); self.icon.get_credits(s);
      s.push("piece :".to_owned()); self.piece.get_credits(s);
      s.push("relics :".to_owned()); self.relics.get_credits(s);
      s.push("ui :".to_owned()); self.ui.get_credits(s);
      
      }
   
}


pub struct AssetIconId4 {
   pub icon_16px : Texture2D,
   pub icon_32px : Texture2D,
   pub icon_64px : Texture2D,
   
}


impl AssetIconId4 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         icon_16px : a.load_texture2d(r"./board_graphic/assets\img\icon\icon_16px.png").await,
         icon_32px : a.load_texture2d(r"./board_graphic/assets\img\icon\icon_32px.png").await,
         icon_64px : a.load_texture2d(r"./board_graphic/assets\img\icon\icon_64px.png").await,
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("icon_16px :".to_owned()); self.icon_16px.get_credits(s);
      s.push("icon_32px :".to_owned()); self.icon_32px.get_credits(s);
      s.push("icon_64px :".to_owned()); self.icon_64px.get_credits(s);
      
      }
   
}


pub struct AssetPieceId6 {
   pub cburnett : Texture2D,
   pub chantal : Texture2D,
   pub deja_view : Texture2D,
   
}


impl AssetPieceId6 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         cburnett : a.load_texture2d(r"./board_graphic/assets\img\piece\cburnett.credits(Cburnett).px(128).png").await.tag_add_credit("(Cburnett)").tag_add_px(128),
         chantal : a.load_texture2d(r"./board_graphic/assets\img\piece\chantal.credits(Chantal_Helm).px(128).png").await.tag_add_credit("(Chantal_Helm)").tag_add_px(128),
         deja_view : a.load_texture2d(r"./board_graphic/assets\img\piece\deja_view.credits(deja_view itch io chess piece).png").await.tag_add_credit("(deja_view itch io chess piece)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("cburnett :".to_owned()); self.cburnett.get_credits(s);
      s.push("chantal :".to_owned()); self.chantal.get_credits(s);
      s.push("deja_view :".to_owned()); self.deja_view.get_credits(s);
      
      }
   
}


pub struct AssetUiId8 {
   pub button : Texture2D,
   pub font : AssetFontId10,
   pub hud_top : Texture2D,
   pub icon : AssetIconId12,
   pub nine_slice : Texture2D,
   pub splash : Texture2D,
   pub title : Texture2D,
   
}


impl AssetUiId8 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         button : a.load_texture2d(r"./board_graphic/assets\img\ui\button.pa.px(32).margin(1).credits(Mewily).png").await.tag_add_pa().tag_add_px(32).tag_add_margin(1).tag_add_credit("(Mewily)"),
         font : AssetFontId10::load(a).await,
         hud_top : a.load_texture2d(r"./board_graphic/assets\img\ui\hud_top.credits(William Warby Oak Texture).png").await.tag_add_credit("(William Warby Oak Texture)"),
         icon : AssetIconId12::load(a).await,
         nine_slice : a.load_texture2d(r"./board_graphic/assets\img\ui\nine_slice.px(128).margin(1).credits(Mewily).png").await.tag_add_px(128).tag_add_margin(1).tag_add_credit("(Mewily)"),
         splash : a.load_texture2d(r"./board_graphic/assets\img\ui\splash.pa.png").await.tag_add_pa(),
         title : a.load_texture2d(r"./board_graphic/assets\img\ui\title.credits(Mewily and Cburnett).png").await.tag_add_credit("(Mewily and Cburnett)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("button :".to_owned()); self.button.get_credits(s);
      s.push("font :".to_owned()); self.font.get_credits(s);
      s.push("hud_top :".to_owned()); self.hud_top.get_credits(s);
      s.push("icon :".to_owned()); self.icon.get_credits(s);
      s.push("nine_slice :".to_owned()); self.nine_slice.get_credits(s);
      s.push("splash :".to_owned()); self.splash.get_credits(s);
      s.push("title :".to_owned()); self.title.get_credits(s);
      
      }
   
}


pub struct AssetFontId10 {
   pub stanberry : Font,
   
}


impl AssetFontId10 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         stanberry : a.load_font(r"./board_graphic/assets\img\ui\font\stanberry.credits(Jayvee D_ Enaguas _ Grand Chaos).aa.ttf").await.tag_add_credit("(Jayvee D_ Enaguas _ Grand Chaos)").tag_add_aa(),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("stanberry :".to_owned()); self.stanberry.get_credits(s);
      
      }
   
}


pub struct AssetIconId12 {
   pub board : Texture2D,
   pub puzzle_piece : Texture2D,
   pub versus : Texture2D,
   
}


impl AssetIconId12 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         board : a.load_texture2d(r"./board_graphic/assets\img\ui\icon.pa.credits(Mewily)\board.png").await.tag_add_pa().tag_add_credit("(Mewily)"),
         puzzle_piece : a.load_texture2d(r"./board_graphic/assets\img\ui\icon.pa.credits(Mewily)\puzzle_piece.png").await.tag_add_pa().tag_add_credit("(Mewily)"),
         versus : a.load_texture2d(r"./board_graphic/assets\img\ui\icon.pa.credits(Mewily)\versus.png").await.tag_add_pa().tag_add_credit("(Mewily)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("board :".to_owned()); self.board.get_credits(s);
      s.push("puzzle_piece :".to_owned()); self.puzzle_piece.get_credits(s);
      s.push("versus :".to_owned()); self.versus.get_credits(s);
      
      }
   
}


pub struct AssetSoundId14 {
   pub board : AssetBoardId16,
   pub stream_loops_2023_11_29 : Sound,
   pub ui : AssetUiId38,
   
}


impl AssetSoundId14 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         board : AssetBoardId16::load(a).await,
         stream_loops_2023_11_29 : a.load_sound(r"./board_graphic/assets\sound\stream_loops_2023_11_29.credits(Tallbeard Studios itch io music-loop-bundle).music.wav").await.tag_add_credit("(Tallbeard Studios itch io music-loop-bundle)").tag_add_music(),
         ui : AssetUiId38::load(a).await,
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("board :".to_owned()); self.board.get_credits(s);
      s.push("stream_loops_2023_11_29 :".to_owned()); self.stream_loops_2023_11_29.get_credits(s);
      s.push("ui :".to_owned()); self.ui.get_credits(s);
      
      }
   
}


pub struct AssetBoardId16 {
      pub bishop : AssetBishopId20,
      pub king : AssetKingId22,
      pub knight : AssetKnightId24,
      pub pawn : AssetPawnId26,
      pub queen : AssetQueenId28,
      pub rook : AssetRookId30,
      pub event : AssetEventId32,
   pub promotion : Sound,
   
}


impl AssetBoardId16 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
            bishop : AssetBishopId20::load(a).await,
            king : AssetKingId22::load(a).await,
            knight : AssetKnightId24::load(a).await,
            pawn : AssetPawnId26::load(a).await,
            queen : AssetQueenId28::load(a).await,
            rook : AssetRookId30::load(a).await,
            event : AssetEventId32::load(a).await,
         promotion : a.load_sound(r"./board_graphic/assets\sound\board\promotion.credits(580310__colorscrimsontears__fanfare-2-rpg).wav").await.tag_add_credit("(580310__colorscrimsontears__fanfare-2-rpg)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
         s.push("bishop :".to_owned()); self.bishop.get_credits(s);
         s.push("king :".to_owned()); self.king.get_credits(s);
         s.push("knight :".to_owned()); self.knight.get_credits(s);
         s.push("pawn :".to_owned()); self.pawn.get_credits(s);
         s.push("queen :".to_owned()); self.queen.get_credits(s);
         s.push("rook :".to_owned()); self.rook.get_credits(s);
         s.push("event :".to_owned()); self.event.get_credits(s);
      s.push("promotion :".to_owned()); self.promotion.get_credits(s);
      
      }
   
}


pub struct AssetBishopId20 {
   pub captured : Sound,
   pub captured2 : Sound,
   pub captured3 : Sound,
   pub captured4 : Sound,
   pub moving : Sound,
   pub moving2 : Sound,
   pub select : Sound,
   pub select2 : Sound,
   
}


impl AssetBishopId20 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         captured : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\bishop\captured.credits(176005__quartzgate__qvarz-5).wav").await.tag_add_credit("(176005__quartzgate__qvarz-5)"),
         captured2 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\bishop\captured2.todo.credits(81857__alienxxx__blip_001).wav").await.tag_add_credit("(81857__alienxxx__blip_001)"),
         captured3 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\bishop\captured3.credits(643655__snowfightstudios__goblin_yell).wav").await.tag_add_credit("(643655__snowfightstudios__goblin_yell)"),
         captured4 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\bishop\captured4.todo.credits(81857__alienxxx__blip_002).wav").await.tag_add_credit("(81857__alienxxx__blip_002)"),
         moving : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\bishop\moving.credits(531137__ryusa__choir-female-pad-pitch-up).wav").await.tag_add_credit("(531137__ryusa__choir-female-pad-pitch-up)"),
         moving2 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\bishop\moving2.credits(81857__alienxxx__blip_001).wav").await.tag_add_credit("(81857__alienxxx__blip_001)"),
         select : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\bishop\select.credits(582352__nmtvesounds__the-choir).wav").await.tag_add_credit("(582352__nmtvesounds__the-choir)"),
         select2 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\bishop\select2.volume(100).credits(26296__dr-fab__cough_male_01).wav").await.tag_add_volume(100).tag_add_credit("(26296__dr-fab__cough_male_01)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("captured :".to_owned()); self.captured.get_credits(s);
      s.push("captured2 :".to_owned()); self.captured2.get_credits(s);
      s.push("captured3 :".to_owned()); self.captured3.get_credits(s);
      s.push("captured4 :".to_owned()); self.captured4.get_credits(s);
      s.push("moving :".to_owned()); self.moving.get_credits(s);
      s.push("moving2 :".to_owned()); self.moving2.get_credits(s);
      s.push("select :".to_owned()); self.select.get_credits(s);
      s.push("select2 :".to_owned()); self.select2.get_credits(s);
      
      }
   
}


pub struct AssetKingId22 {
   pub captured : Sound,
   pub moving : Sound,
   pub select : Sound,
   pub select2 : Sound,
   
}


impl AssetKingId22 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         captured : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\king\captured.credits(193934__zagi2__fanfare-announcement).wav").await.tag_add_credit("(193934__zagi2__fanfare-announcement)"),
         moving : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\king\moving.volume(40).credits(449069__ricniclas__fanfare).wav").await.tag_add_volume(40).tag_add_credit("(449069__ricniclas__fanfare)"),
         select : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\king\select.volume(50).credits(580310__colorscrimsontears__fanfare-2-rpg).wav").await.tag_add_volume(50).tag_add_credit("(580310__colorscrimsontears__fanfare-2-rpg)"),
         select2 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\king\select2.volume(20).credits(243888__zuluonedrop__eq-tru132-trumpet).wav").await.tag_add_volume(20).tag_add_credit("(243888__zuluonedrop__eq-tru132-trumpet)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("captured :".to_owned()); self.captured.get_credits(s);
      s.push("moving :".to_owned()); self.moving.get_credits(s);
      s.push("select :".to_owned()); self.select.get_credits(s);
      s.push("select2 :".to_owned()); self.select2.get_credits(s);
      
      }
   
}


pub struct AssetKnightId24 {
   pub captured : Sound,
   pub moving : Sound,
   pub moving2 : Sound,
   pub select : Sound,
   
}


impl AssetKnightId24 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         captured : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\knight\captured.credits(557302__seb_monserrano__caballo-ajedrez).wav").await.tag_add_credit("(557302__seb_monserrano__caballo-ajedrez)"),
         moving : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\knight\moving.credits(322448__deadxcreed__horse-gallop-loop).wav").await.tag_add_credit("(322448__deadxcreed__horse-gallop-loop)"),
         moving2 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\knight\moving2.todo.credits(475479__o_ciz__horsesnort_2).wav").await.tag_add_credit("(475479__o_ciz__horsesnort_2)"),
         select : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\knight\select.credits(475479__o_ciz__horsesnort_2).wav").await.tag_add_credit("(475479__o_ciz__horsesnort_2)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("captured :".to_owned()); self.captured.get_credits(s);
      s.push("moving :".to_owned()); self.moving.get_credits(s);
      s.push("moving2 :".to_owned()); self.moving2.get_credits(s);
      s.push("select :".to_owned()); self.select.get_credits(s);
      
      }
   
}


pub struct AssetPawnId26 {
   pub captured : Sound,
   pub moving : Sound,
   pub select : Sound,
   
}


impl AssetPawnId26 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         captured : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\pawn\captured.volume(40).credits(319590__hybrid_v__shield-bash-impact).wav").await.tag_add_volume(40).tag_add_credit("(319590__hybrid_v__shield-bash-impact)"),
         moving : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\pawn\moving.credits(351518__mh2o__chess_move_on_alabaster).wav").await.tag_add_credit("(351518__mh2o__chess_move_on_alabaster)"),
         select : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\pawn\select.credits(566176__njjjjjjjjjjjjjjjjjjjjjjjj__flicking-a-switch).wav").await.tag_add_credit("(566176__njjjjjjjjjjjjjjjjjjjjjjjj__flicking-a-switch)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("captured :".to_owned()); self.captured.get_credits(s);
      s.push("moving :".to_owned()); self.moving.get_credits(s);
      s.push("select :".to_owned()); self.select.get_credits(s);
      
      }
   
}


pub struct AssetQueenId28 {
   pub captured : Sound,
   pub captured2 : Sound,
   pub hum_hum : Sound,
   pub moving : Sound,
   pub selected : Sound,
   
}


impl AssetQueenId28 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         captured : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\queen\captured.credits(475296__hetsumani__woman-expressing-pain).wav").await.tag_add_credit("(475296__hetsumani__woman-expressing-pain)"),
         captured2 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\queen\captured2.credits(639860__owstu__female-voc-txt-_oh-no).wav").await.tag_add_credit("(639860__owstu__female-voc-txt-_oh-no)"),
         hum_hum : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\queen\hum_hum.credits(532425__zoshp__woman-clearing-throat-fx).wav").await.tag_add_credit("(532425__zoshp__woman-clearing-throat-fx)"),
         moving : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\queen\moving.credits(456641__370295__lazer-sound).wav").await.tag_add_credit("(456641__370295__lazer-sound)"),
         selected : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\queen\selected.credits(211424__seidhepriest__magical-bomb-falling).wav").await.tag_add_credit("(211424__seidhepriest__magical-bomb-falling)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("captured :".to_owned()); self.captured.get_credits(s);
      s.push("captured2 :".to_owned()); self.captured2.get_credits(s);
      s.push("hum_hum :".to_owned()); self.hum_hum.get_credits(s);
      s.push("moving :".to_owned()); self.moving.get_credits(s);
      s.push("selected :".to_owned()); self.selected.get_credits(s);
      
      }
   
}


pub struct AssetRookId30 {
   pub captured : Sound,
   pub captured2 : Sound,
   pub move2 : Sound,
   pub move3 : Sound,
   pub moving : Sound,
   pub select : Sound,
   
}


impl AssetRookId30 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         captured : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\rook\captured.credits(77074__benboncan__bricks-falling).wav").await.tag_add_credit("(77074__benboncan__bricks-falling)"),
         captured2 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\rook\captured2.credits(434897__thebuilder15__collapse).wav").await.tag_add_credit("(434897__thebuilder15__collapse)"),
         move2 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\rook\move2.credits(233698__timkahn__clobs).wav").await.tag_add_credit("(233698__timkahn__clobs)"),
         move3 : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\rook\move3.credits(398698__bbrocer__slow-pebble-tumble).wav").await.tag_add_credit("(398698__bbrocer__slow-pebble-tumble)"),
         moving : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\rook\moving.credits(473582__nox_sound__rock_movement_friction_02).wav").await.tag_add_credit("(473582__nox_sound__rock_movement_friction_02)"),
         select : a.load_sound(r"./board_graphic/assets\sound\board\.sfx\rook\select.credits(473575__nox_sound__rock_movement_friction_01).wav").await.tag_add_credit("(473575__nox_sound__rock_movement_friction_01)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("captured :".to_owned()); self.captured.get_credits(s);
      s.push("captured2 :".to_owned()); self.captured2.get_credits(s);
      s.push("move2 :".to_owned()); self.move2.get_credits(s);
      s.push("move3 :".to_owned()); self.move3.get_credits(s);
      s.push("moving :".to_owned()); self.moving.get_credits(s);
      s.push("select :".to_owned()); self.select.get_credits(s);
      
      }
   
}


pub struct AssetEventId32 {
      pub defeat : Sound,
      pub draw : Sound,
      pub start : Sound,
      pub victory : Sound,
         pub game_over : Sound,
      pub hover : Sound,
      pub hover_team : Sound,
      pub piece_move : Sound,
      pub select_piece : Sound,
      pub select_piece2 : Sound,
      pub unselect : Sound,
      
}


impl AssetEventId32 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
            defeat : a.load_sound(r"./board_graphic/assets\sound\board\event\.music\defeat.volume(80).credits(731657__kanaizo__piano-game-over-theme-85-bpm-f-minor).wav").await.tag_add_music().tag_add_volume(80).tag_add_credit("(731657__kanaizo__piano-game-over-theme-85-bpm-f-minor)"),
            draw : a.load_sound(r"./board_graphic/assets\sound\board\event\.music\draw.credits(522245__dzedenz__result-8).wav").await.tag_add_music().tag_add_credit("(522245__dzedenz__result-8)"),
            start : a.load_sound(r"./board_graphic/assets\sound\board\event\.music\start.volume(50).credits(193934__zagi2__fanfare-announcement).wav").await.tag_add_music().tag_add_volume(50).tag_add_credit("(193934__zagi2__fanfare-announcement)"),
            victory : a.load_sound(r"./board_graphic/assets\sound\board\event\.music\victory.volume(40).credits(516911__xythe__snippet).wav").await.tag_add_music().tag_add_volume(40).tag_add_credit("(516911__xythe__snippet)"),
               game_over : a.load_sound(r"./board_graphic/assets\sound\board\event\.sfx\game_over.credits(380277__rhodesmas__alert-04).wav").await.tag_add_sfx().tag_add_credit("(380277__rhodesmas__alert-04)"),
            hover : a.load_sound(r"./board_graphic/assets\sound\board\event\.sfx\hover.volume(20).credtis(237394__squareal__mouth-blowing).wav").await.tag_add_sfx().tag_add_volume(20),
            hover_team : a.load_sound(r"./board_graphic/assets\sound\board\event\.sfx\hover_team.credits(25072__freqman__whoosh03).wav").await.tag_add_sfx().tag_add_credit("(25072__freqman__whoosh03)"),
            piece_move : a.load_sound(r"./board_graphic/assets\sound\board\event\.sfx\piece_move.credits(568294__sheyvan__sfx-whoosh-high-1-3).wav").await.tag_add_sfx().tag_add_credit("(568294__sheyvan__sfx-whoosh-high-1-3)"),
            select_piece : a.load_sound(r"./board_graphic/assets\sound\board\event\.sfx\select_piece.credits(537709__kostas17__wooden-object-place).wav").await.tag_add_sfx().tag_add_credit("(537709__kostas17__wooden-object-place)"),
            select_piece2 : a.load_sound(r"./board_graphic/assets\sound\board\event\.sfx\select_piece2.credits(546079__stavsounds__button-selected).wav").await.tag_add_sfx().tag_add_credit("(546079__stavsounds__button-selected)"),
            unselect : a.load_sound(r"./board_graphic/assets\sound\board\event\.sfx\unselect.credits(731994__6sdeimos__click-2).wav").await.tag_add_sfx().tag_add_credit("(731994__6sdeimos__click-2)"),
            
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
         s.push("defeat :".to_owned()); self.defeat.get_credits(s);
         s.push("draw :".to_owned()); self.draw.get_credits(s);
         s.push("start :".to_owned()); self.start.get_credits(s);
         s.push("victory :".to_owned()); self.victory.get_credits(s);
            s.push("game_over :".to_owned()); self.game_over.get_credits(s);
         s.push("hover :".to_owned()); self.hover.get_credits(s);
         s.push("hover_team :".to_owned()); self.hover_team.get_credits(s);
         s.push("piece_move :".to_owned()); self.piece_move.get_credits(s);
         s.push("select_piece :".to_owned()); self.select_piece.get_credits(s);
         s.push("select_piece2 :".to_owned()); self.select_piece2.get_credits(s);
         s.push("unselect :".to_owned()); self.unselect.get_credits(s);
         
      }
   
}


pub struct AssetUiId38 {
   pub cancel : Sound,
   pub hover_in : Sound,
   pub ok : Sound,
   pub ok2 : Sound,
   pub press : Sound,
   pub redo : Sound,
   pub resize : Sound,
   pub resize2 : Sound,
   pub split : Sound,
   pub split2 : Sound,
   pub undo : Sound,
   
}


impl AssetUiId38 {
   pub async fn load(a : &mut ContextAssetManager) -> Self {
      Self {
         cancel : a.load_sound(r"./board_graphic/assets\sound\ui\cancel.credits(388047__paep3nguin__beep_down).wav").await.tag_add_credit("(388047__paep3nguin__beep_down)"),
         hover_in : a.load_sound(r"./board_graphic/assets\sound\ui\hover_in.credits(422836__gamedevc__g_ui_button_hover_1).wav").await.tag_add_credit("(422836__gamedevc__g_ui_button_hover_1)"),
         ok : a.load_sound(r"./board_graphic/assets\sound\ui\ok.credits(388046__paep3nguin__beep_up).wav").await.tag_add_credit("(388046__paep3nguin__beep_up)"),
         ok2 : a.load_sound(r"./board_graphic/assets\sound\ui\ok2.credits(403013__inspectorj__ui-confirmation-alert-b5).wav").await.tag_add_credit("(403013__inspectorj__ui-confirmation-alert-b5)"),
         press : a.load_sound(r"./board_graphic/assets\sound\ui\press.credits(566194__scholzi982__press_button_03).wav").await.tag_add_credit("(566194__scholzi982__press_button_03)"),
         redo : a.load_sound(r"./board_graphic/assets\sound\ui\redo.todo.credtis(155174__razzdaspazz__gunslide).wav").await,
         resize : a.load_sound(r"./board_graphic/assets\sound\ui\resize.volume(50).credits(735521__avreference__clock-end).wav").await.tag_add_volume(50).tag_add_credit("(735521__avreference__clock-end)"),
         resize2 : a.load_sound(r"./board_graphic/assets\sound\ui\resize2.credits(488985__phonosupf__wood-frog-sequence).wav").await.tag_add_credit("(488985__phonosupf__wood-frog-sequence)"),
         split : a.load_sound(r"./board_graphic/assets\sound\ui\split.credits(28882__junggle__btn072).wav").await.tag_add_credit("(28882__junggle__btn072)"),
         split2 : a.load_sound(r"./board_graphic/assets\sound\ui\split2.credits(72123__bugfish__whopity).wav").await.tag_add_credit("(72123__bugfish__whopity)"),
         undo : a.load_sound(r"./board_graphic/assets\sound\ui\undo.credits(155174__razzdaspazz__gunslide).wav").await.tag_add_credit("(155174__razzdaspazz__gunslide)"),
         
      }
      
   }
   pub fn get_credits(&self, s : &mut Vec<String>) {
      s.push("cancel :".to_owned()); self.cancel.get_credits(s);
      s.push("hover_in :".to_owned()); self.hover_in.get_credits(s);
      s.push("ok :".to_owned()); self.ok.get_credits(s);
      s.push("ok2 :".to_owned()); self.ok2.get_credits(s);
      s.push("press :".to_owned()); self.press.get_credits(s);
      s.push("redo :".to_owned()); self.redo.get_credits(s);
      s.push("resize :".to_owned()); self.resize.get_credits(s);
      s.push("resize2 :".to_owned()); self.resize2.get_credits(s);
      s.push("split :".to_owned()); self.split.get_credits(s);
      s.push("split2 :".to_owned()); self.split2.get_credits(s);
      s.push("undo :".to_owned()); self.undo.get_credits(s);
      
      }
   
}


