use std::{collections::HashMap, default, hash::Hash, marker::PhantomData, ops::*, vec, fmt::Debug};

use game_engine::*;

use crate::*;

#[derive(Default, Clone, Debug)]
pub struct InputProvidedByUi;

impl<G:IGame<InputProvider=Self>+UiOwner> IInputProvider<G> for InputProvidedByUi where G::Input : Default
{
    fn get_input(&mut self, game : &mut G, _time : GameTime, ctx : &mut DefaultContext<<G as IGame>::Global>) -> <G as IGame>::Input 
    {
        game.get_input_ui(self, ctx)
    }
}
/* 
struct InputProviderUi<G:IGame+UiOwner> { _phantom : PhantomData<G> }
impl<G:IGame+UiOwner> Default for InputProviderUi<G> { fn default() -> Self { Self { _phantom: ___() } }}

impl<G:IGame<InputProvider=Self>+UiOwner> IInputProvider<G> for InputProviderUi<G>
{
    fn get_input(&mut self, game : &mut G, time : DrawTime, ctx : &mut DefaultContext<<G as IGame>::Global>) -> <G as IGame>::Input {
        game.ui().get_input_ui(game, self, time, ctx)
    }
}
*/

type UiIdx = usize;

/* 
#[derive(Clone, Debug, PartialEq)]
pub struct UiInput<Owner : UiOwner>
{
    name : UiName<Owner>,
    kind : UiInterractiveState,
}
impl<Owner : UiOwner> Deref for UiInput<Owner> { type Target=UiInterractiveState; fn deref(&self) -> &Self::Target { &self.kind }}
impl<Owner : UiOwner> UiInput<Owner>
{ 
    /// panic if not named
    pub fn name(&self) -> &Owner::UiName { self.name.as_named() }

    pub fn ui_name(&self) -> &UiName<Owner> { &self.name }
    pub fn kind(&self) -> &UiInterractiveState { &self.kind }
    fn new(name : UiName<Owner>, kind : UiInterractiveState) -> Self { Self { name, kind }}
}*/

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub enum UiName<Owner : UiOwner>
{
    Root,
    #[default]
    Lambda,
    Named(Owner::UiName)
}
impl<Owner : UiOwner> UiName<Owner>
{
    pub fn is_root(&self) -> bool { matches!(self, UiName::Root) }
    pub fn is_lambda(&self) -> bool { matches!(self, UiName::Lambda) }

    pub fn try_as_named(&self) -> Option<&Owner::UiName>
    {
        match self
        {
            UiName::Named(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_named(&self) -> &Owner::UiName
    { self.try_as_named().expect("ui was not named") }
}


#[derive(Clone, Debug, PartialEq)]
pub struct UiBuilder<Owner : UiOwner>
{
    last   : LastStack<UiIdx>,
    proto  : LastStack<UiElementData<Owner>>,

    //_owner : PhantomData<Owner>,
    //split  : UiSplitConfig,

    pos_margin_rest : Vec<UiPos>,
}
impl<Owner : UiOwner> UiPage<Owner>
{
    pub fn last(&self) -> &UiElement<Owner> { self.get(*self.last.last()) }
    pub fn last_mut(&mut self) -> &mut UiElement<Owner> { self.get_mut(*self.last.last()) }
    pub fn last_pop(&mut self)  { self.last.pop() }
    pub fn last_push(&mut self)  { self.last.push() }


    pub fn proto(&self) -> &UiElementData<Owner> { self.proto.last() }
    pub fn proto_mut(&mut self) -> &mut UiElementData<Owner> { self.proto.last_mut() }

    pub fn proto_push(&mut self) { self.proto.push(); }
    pub fn proto_pop(&mut self) { self.proto.pop(); }
    pub fn proto_set(&mut self, proto : UiElementData<Owner>) { self.proto.set(proto); }

    /// set the source and the dest
    pub fn proto_set_fullscreen(&mut self)
    {
        //let mut e = UiElementData::___();
        //self.proto_set(e);
        self.proto.pos.set_src(UiRect::ONE).set_dest(UiRect::ONE);
    }

    pub fn proto_reset(&mut self)
    {
        self.proto.clear();
        self.last.clear();
        self.last.set(ROOT_ID);
        self.pos_margin_rest.clear();

        self.proto_set_fullscreen();
    }
}

impl<Owner : UiOwner> UiPage<Owner>
{
    pub fn update_ui(&mut self, time : GameTime, ctx : &mut DefaultContext<Owner::Global>)
    {
        self.time += time.delta();

        let win_size_px = ctx.cam().window_size_px();
        if win_size_px != self.window_size_px
        {
            self.window_size_px = win_size_px;
            self.resolution_changed(ctx);
        };
        
        self._update_ui(ROOT_ID, ___(), ctx);
    }

    fn _update_ui(&mut self, idx : UiIdx, mut relative : UiRelative, ctx : &mut DefaultContext<Owner::Global>)
    {
        let time = self.time;

        ctx.cam_mut().push();

        let mut c = self.get(idx);
        if c.is_inactive() { return; }

        relative = UiRelative::new(c.apply_relative_and_get_next(self.time, relative, ctx).pos);

        if c.input_can_interract()
        {
            let name = self.get(idx).input_name();
            let target_idx = self.ui_named_to_idx(&name).unwrap_or(idx);
    
            let target = self.get_mut(target_idx);
            target.input.time_hover.update(UiElement::<Owner>::is_hover(ctx), time);
            target.input.time_press.update(UiElement::<Owner>::is_just_pressed(ctx), time);
        }

        c = self.get(idx);
        let mut childs_idx = 0;
        while childs_idx < c.child.len()
        {
            let child_id = c.child[childs_idx];
            self._update_ui(child_id, relative, ctx);
            childs_idx += 1;
            c = self.get(idx);
        }

        ctx.cam_mut().pop();
    }
    
    pub fn get_input_ui(game : &mut Owner, input_provider : &mut <Owner as IGame>::InputProvider, ctx : &mut DefaultContext<Owner::Global>) -> Owner::Input
        where <Owner as IGame>::Input : Default
    {
        let mut input = <Owner as IGame>::Input::default();
        for c in Self::get_input_vec_ui(game, input_provider, ctx)
        {
            input.combine(c);
        }
        input
    }

    fn get_input_vec_ui(game : &mut Owner, input_provider : &mut <Owner as IGame>::InputProvider, ctx : &mut DefaultContext<Owner::Global>) -> Vec<Owner::Input>
    { 
        let mut inputs = Vec::new();
        Self::_get_input_vec_ui(game, ROOT_ID, &mut inputs, input_provider, ___(), ctx);
        inputs
    }
    
    fn _get_input_vec_ui(game : &mut Owner, idx : UiIdx, inputs : &mut Vec<Owner::Input>, input_provider : &mut <Owner as IGame>::InputProvider, mut relative : UiRelative, ctx : &mut DefaultContext<Owner::Global>)
    {

        let mut s = game.ui();

        let mut c = s.get(idx);
        if c.is_inactive() { return; }

        ctx.cam_mut().push();

        {
            let rect_px = c.apply_relative_and_get_next(s.time, relative, ctx);

            if c.input_can_interract() 
            {
                if let Some(input) = s.update_interrative_idx(idx, ctx)
                {
                    inputs.push(input);
                }

                if let Some(name) = c.name().try_as_named()
                {
                    if let Some(input) = game.get_input_ui_element(input_provider, name.clone(), ctx)
                    {
                        inputs.push(input);
                    }
                    s = game.ui();
                }
            }
            relative = UiRelative::new(rect_px.pos);
        }

        c = s.get(idx);
        
        let mut childs_idx = 0;
        while childs_idx < c.child.len()
        {
            let child_id = c.child[childs_idx];
            Self::_get_input_vec_ui(game, child_id, inputs, input_provider, relative, ctx);
            childs_idx += 1;
            s = game.ui();
            c = s.get(idx);
        }
        ctx.cam_mut().pop();
    }

    fn update_interrative_idx(&self, idx : UiIdx, ctx : &mut DefaultContext<Owner::Global>) -> Option<Owner::Input>
    {
        let s = self.get(idx);

        let name = s.input_name();
        let target_idx = self.ui_named_to_idx(&name).unwrap_or(idx);
        let target = self.get(target_idx);

        if target.input.time_hover.pull_changed() 
        { 
            if target.input.time_hover.is_pull_up()
            { 
                // just hover
                target.input.sound.hover_in.as_ref().map(|e| e.play(ctx.audio_mut()));
                return target.input.hover_in_input.clone();
            } else
            { 
                // hover is released
                target.input.sound.hover_out.as_ref().map(|e| e.play(ctx.audio_mut()));
                return target.input.hover_out().clone();
            };
        }

        if target.input.time_press.pull_changed() 
        { 
            if target.input.time_press.is_pull_up()
            { 
                target.input.sound.pressed.as_ref().map(|e| e.play(ctx.audio_mut()));
                return target.input.pressed_input.clone();
            } 
            else
            {
                target.input.sound.released.as_ref().map(|e| e.play(ctx.audio_mut()));
                return target.input.released_input.clone();
            }
        }
        None
    }

    fn _draw(&self, idx : UiIdx, owner : &Owner, mut relative : UiRelative, ctx : &mut DefaultContext<Owner::Global>)
    {
        let mut c = self.get(idx);
        if c.is_inactive() { return; }
        
        ctx.cam_mut().push();

        relative = c.draw(self.time, relative, owner, ctx);
        let mut childs_idx = 0;
        while childs_idx < c.child_draw_order.len()
        {
            let child_id = c.child_draw_order[childs_idx];
            self._draw(child_id, owner, relative, ctx);
            childs_idx += 1;
            c = self.get(idx);
        }

        ctx.cam_mut().pop();
    }

    pub fn draw(&self, owner : &Owner, ctx : &mut DefaultContext<Owner::Global>)
    {
        self._draw(ROOT_ID, owner, ___(), ctx);
    }

    fn _resolution_changed(&mut self, idx : UiIdx, mut relative : UiRelative, ctx : &mut DefaultContext<Owner::Global>)
    {
        ctx.cam_mut().push();


        let mut c = self.get_mut(idx);
        c.pos.update_cache(ctx.cam());
        relative = self.split_update(idx, relative, ctx);

        c = self.get_mut(idx);
        c.text.resolution_changed(ctx);

        let mut childs_idx = 0;
        while childs_idx < c.child.len()
        {
            let child_id = c.child[childs_idx];
            self._resolution_changed(child_id, relative, ctx);

            childs_idx += 1;
            c = self.get_mut(idx);
        }
        ctx.cam_mut().pop();
    }

    pub fn resolution_changed(&mut self, ctx : &mut DefaultContext<Owner::Global>)
    { self._resolution_changed(ROOT_ID, ___(), ctx); }

    fn _build(&mut self, idx : UiIdx, ctx : &mut DefaultContext<Owner::Global>)
    {
        self.get_mut(idx).is_build = true;

        let mut c = self.get_mut(idx);
        let mut childs_idx = 0;

        while childs_idx < c.child.len()
        {
            let c_idx = c.child[childs_idx];
            self._build(c_idx, ctx);
            c = self.get_mut(idx);
            childs_idx += 1;
        }
    }
    
    pub fn build(&mut self, ctx : &mut DefaultContext<Owner::Global>)
    {
        self.resolution_changed(ctx);
        self._build(ROOT_ID, ctx);
        self.proto_reset();
    }
    
    fn split_update(&mut self, idx : UiIdx, relative : UiRelative, ctx : &mut DefaultContext<Owner::Global>) -> UiRelative
    {
        let time = self.time;

        let mut s = self.get(idx);
        let rect_px = s.apply_relative_from_pos_and_get_next(relative, s.pos.dest(), ctx);

        //let sum : real = self.reserve.iter().map(|e| e.weight).sum();
        
        let mut split : Vec<(UiIdx, UiSplit)> = s.child.iter().filter_map(|child_idx| self.get(*child_idx).in_split.map(|e| (*child_idx, e))).collect();
        
        let mut childs_sorted_by_z : Vec<(UiIdx, real)> = s.child.iter().map(|e| (*e, self.get(*e).z_idx())).collect();
        childs_sorted_by_z.sort_by(|a, b| a.1.total_cmp(&b.1));
        self.get_mut(idx).child_draw_order = childs_sorted_by_z.into_iter().map(|(idx, _z)| idx).collect();
        s = self.get(idx);

        if split.len() == 0 { return UiRelative::new(rect_px.pos); }
        if s.split_on.rev { split.reverse(); }
        let margin = s.split_on.margin;

        let cam = ctx.cam_mut();

        let axis = match s.split_on.axis
        {
            UiSplitConfigAxis::Axis(axis) => axis,
            UiSplitConfigAxis::Min => 
            { if cam.parent_size_px().x < cam.parent_size_px().y { Axis::X } else { Axis::Y }}
            UiSplitConfigAxis::Max =>
            { if cam.parent_size_px().x < cam.parent_size_px().y { Axis::Y } else { Axis::X }}
            UiSplitConfigAxis::WindowMin => 
            { if cam.window_size_px().x < cam.window_size_px().y { Axis::X } else { Axis::Y }}
            UiSplitConfigAxis::WindowMax =>
            { if cam.window_size_px().x < cam.window_size_px().y { Axis::Y } else { Axis::X }}
        };

        if s.split_on.freeze_axis_once_calculated
        {
            self.get_mut(idx).split_on.axis = UiSplitConfigAxis::Axis(axis);
            s = self.get(idx);
        }

        if Some(axis) != s.split_on.prev_cached_axis
        {
            if s.is_build && s.is_active() { s.split_on.split_sound.as_ref().map(|e| { e.play(ctx.audio_mut()); }); }
            let s = self.get_mut(idx);
            s.split_on.prev_cached_axis = Some(axis);
        }

        s = self.get(idx);


        //let mut p = self.pos;
        let mut p = UiRect::ONE;

        match axis
        {
            Axis::X => 
            {
                match s.split_on.margin_kind
                {
                    UiSplitConfigMarginKind::InBetween => {},
                    UiSplitConfigMarginKind::InBetweenAndEdge => 
                    {
                        p.add_margin_left_and_right(margin);
                    },
                }

                let mut space = p;
                space.size.x -= s.split_on.margin * (split.len() -1).to_real();

                let mut only_fixed_size = true; 

                let mut sum_weight = 0.; 
                split.iter()
                    .for_each(|(_ui_idx, split_info)| 
                    {
                        match split_info.weight
                        {
                            UiSplitWeight::Weight(v) => { sum_weight += v; only_fixed_size = false; }
                            UiSplitWeight::Unit(v) => { space.size.x -= v; },
                        }
                    });

                let margin_if_fixed_size = if only_fixed_size { space.size.x / (split.len()-1).to_real() } else { UiUnit::ZERO };
                
                for (ui_idx, split_info) in split.into_iter()
                {
                    p.size.x = match split_info.weight
                    {
                        UiSplitWeight::Weight(v) => space.size.x * v / sum_weight,
                        UiSplitWeight::Unit(v) => v,
                    };

                    let e = self.get_mut(ui_idx);
                    if e.is_build
                    {
                        self.get_mut(ui_idx).pos.animate_to(time, p);
                    }else
                    {
                        e.pos.for_all_mut(|t| *t = p);
                    }
                    
                    let size_x = p.size.x;
                    p.pos.x += margin + size_x + margin_if_fixed_size;
                }
            },
            Axis::Y => 
            {
                match s.split_on.margin_kind
                {
                    UiSplitConfigMarginKind::InBetween => {},
                    UiSplitConfigMarginKind::InBetweenAndEdge => 
                    {
                        p.add_margin_top_and_bot(margin);
                    },
                }

                let mut space = p;
                space.size.y -= s.split_on.margin * (split.len() -1).to_real();

                let mut only_fixed_size = true; 

                let mut sum_weight = 0.; 
                split.iter()
                    .for_each(|(_ui_idx, split_info)| 
                    {
                        match split_info.weight
                        {
                            UiSplitWeight::Weight(v) => { sum_weight += v; only_fixed_size = false; }
                            UiSplitWeight::Unit(v) => { space.size.y -= v; },
                        }
                    });

                let margin_if_fixed_size = if only_fixed_size { space.size.y / (split.len()-1).to_real() } else { UiUnit::ZERO };
                
                for (ui_idx, split_info) in split.into_iter().rev()
                {
                    p.size.y = match split_info.weight
                    {
                        UiSplitWeight::Weight(v) => space.size.y * v / sum_weight,
                        UiSplitWeight::Unit(v) => v,
                    };

                    let e = self.get_mut(ui_idx);
                    if e.is_build
                    {
                        self.get_mut(ui_idx).pos.animate_to(time, p);
                    }else
                    {
                        e.pos.for_all_mut(|t| *t = p);
                    }

                    let size_y = p.size.y;
                    p.pos.y += margin + size_y + margin_if_fixed_size;
                }
            },
        }

        UiRelative::new(rect_px.pos)
    }


    // pub fn parent_set(&mut self, parent : &UiElement<Name>) { self.parent.set(parent.idx) }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum UiSplitConfigMarginKind
{
    #[default]
    InBetween,
    InBetweenAndEdge,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum UiSplitConfigAxis
{
    /// Parent Axis
    Axis(Axis),
    /// Parent Min
    Min,
    /// Parent Max
    #[default]
    Max,

    /// Window Min
    WindowMin,
    /// Window Max
    WindowMax,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UiSplitOn
{
    pub axis             : UiSplitConfigAxis,
    pub prev_cached_axis : Option<Axis>,


    /// if true, the axis will be fixed to X or Y once it has been calculated once.
    pub freeze_axis_once_calculated : bool,
    pub rev         : bool,

    pub margin      : UiUnit,
    pub margin_kind : UiSplitConfigMarginKind,

    pub split_sound : Option<Sound>,

    //reserve : Vec<SplitReserve<Name>>,
}
impl UiSplitOn
{
    pub fn new() -> Self { Self::___() }
    /* 
    pub fn new(axis : UiSplitConfigAxis, margin : UiNumber, margin_kind : UiSplitConfigMarginKind, rev : bool) -> Self
    {
        Self { axis, rev, margin, margin_kind, ..___() }
    }*/

    pub fn with_x_axis(self) -> Self { self.with_axis(UiSplitConfigAxis::Axis(Axis::X)) }
    pub fn with_y_axis(self) -> Self { self.with_axis(UiSplitConfigAxis::Axis(Axis::Y)) }

    pub fn with_axis(mut self, axis : UiSplitConfigAxis) -> Self
    {
        self.set_axis(axis);
        self
    }
    pub fn set_axis(&mut self, axis : UiSplitConfigAxis) -> &mut Self
    {
        self.axis = axis;
        self
    }


    pub fn with_margin_and_kind(self, margin : UiUnit, margin_kind : UiSplitConfigMarginKind) -> Self
    { self.with_margin(margin).with_margin_kind(margin_kind) }

    pub fn set_margin_and_kind(&mut self, margin : UiUnit, margin_kind : UiSplitConfigMarginKind) -> &mut Self
    { self.set_margin(margin).set_margin_kind(margin_kind) }

    /// Freeze the axis once calculated.
    /// 
    /// The split will transform special axis like `Min` / `Max` / `WindowMin`/ `WindowMax` to only focus the `X` or `Y` axis when calculated for the first time
    pub fn and_freeze_axis(&mut self) -> &mut Self { self.set_freeze_axis(true) }
    
    /// Freeze the axis once calculated.
    /// 
    /// The split will transform special axis like `Min` / `Max` / `WindowMin`/ `WindowMax` to only focus the `X` or `Y` axis when calculated for the first time
    pub fn set_freeze_axis(&mut self, freeze_axis_once_calculated : bool) -> &mut Self { self.freeze_axis_once_calculated = freeze_axis_once_calculated; self }


    pub fn with_margin(mut self, margin : UiUnit) -> Self
    {
        self.set_margin(margin);
        self
    }
    pub fn set_margin(&mut self, margin : UiUnit) -> &mut Self
    {
        self.margin = margin;
        self
    }


    pub fn with_margin_kind(mut self, margin_kind : UiSplitConfigMarginKind) -> Self
    {
        self.set_margin_kind(margin_kind);
        self
    }
    pub fn set_margin_kind(&mut self, margin_kind : UiSplitConfigMarginKind) -> &mut Self
    {
        self.margin_kind = margin_kind;
        self
    }

    pub fn with_reverse(mut self, rev : bool) -> Self
    { self.set_rev(rev); self }
    pub fn and_rev(&mut self) -> &mut Self
    { self.set_rev(!self.rev) }
    pub fn set_rev(&mut self, rev : bool) -> &mut Self
    {
        self.rev = rev;
        self
    }

    pub fn with_sfx(mut self, split_sfx : Option<Sfx>) -> Self { self.split_sound = split_sfx; self }
    pub fn add_sfx(&mut self, split_sfx : &Sfx) -> &mut Self { self.set_sfx(Some(split_sfx)) }
    pub fn set_sfx(&mut self, split_sfx : Option<&Sfx>) -> &mut Self { self.split_sound = split_sfx.map(|e| e.clone()); self  }

}


impl<Owner : UiOwner> UiElement<Owner>
{
    // Short Hand

    pub fn add_split_on_axis(&mut self, axis : UiSplitConfigAxis) -> &mut Self
    { self.split_on.axis = axis; self }

    pub fn add_split_on_y(&mut self) -> &mut Self
    { self.add_split_on_axis(UiSplitConfigAxis::Axis(Axis::Y)) }

    pub fn add_split_on_x(&mut self) -> &mut Self
    { self.add_split_on_axis(UiSplitConfigAxis::Axis(Axis::X)) }

    pub fn add_split_on_max(&mut self) -> &mut Self
    { self.add_split_on_axis(UiSplitConfigAxis::Max) }

    pub fn add_split_on_min(&mut self) -> &mut Self
    { self.add_split_on_axis(UiSplitConfigAxis::Min) }
    
    pub fn add_split_on_window_max(&mut self) -> &mut Self
    { self.add_split_on_axis(UiSplitConfigAxis::WindowMax) }

    pub fn add_split_on_window_min(&mut self) -> &mut Self
    { self.add_split_on_axis(UiSplitConfigAxis::WindowMin) }
    
    /// The margin is between each element. There is no margin at the beginning and the end
    pub fn set_split(&mut self, split_on : UiSplitOn) -> &mut Self
    {
        self.split_on = split_on;
        self
    }

    pub fn add_split_sfx(&mut self, sound : &Sound) -> &mut Self
    { self.split_on.add_sfx(sound); self }

    
    pub fn set_split_margin(&mut self, margin : UiUnit) -> &mut Self
    { self.split_on.set_margin(margin); self }

    pub fn set_split_margin_kind(&mut self, margin_kind : UiSplitConfigMarginKind) -> &mut Self
    { self.split_on.set_margin_kind(margin_kind); self }

    pub fn set_split_margin_kind_and_kind(&mut self, margin : UiUnit, margin_kind : UiSplitConfigMarginKind) -> &mut Self
    { self.split_on.set_margin_and_kind(margin, margin_kind); self }
}   


impl<Owner : UiOwner> Default for UiBuilder<Owner>
{
    fn default() -> Self 
    { 
        Self 
        { 
            proto: ___(), 
            pos_margin_rest: ___(),
            // 0 is the root
            last: LastStack::new(ROOT_ID),
        } 
    }
}
impl<Owner : UiOwner> Deref for UiBuilder<Owner> { type Target=UiElementData<Owner>; fn deref(&self) -> &Self::Target { &self.proto }}
impl<Owner : UiOwner> DerefMut for UiBuilder<Owner> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.proto }}

pub trait UiOwner : IGame
{
    type UiName     : Hash + Eq + Clone + Debug;
    type UiPageName : Hash + Eq + Clone + Debug;

    fn ui(&self) -> &UiManager<Self>;
    fn ui_mut(&mut self) -> &mut UiManager<Self>;

    fn draw_ui_element(&self, name : &Self::UiName, ctx : &mut DefaultContext<Self::Global>);

    /// only for interractive element
    #[allow(unused_variables)]
    fn get_input_ui_element(&mut self, input_provider : &mut Self::InputProvider, name : Self::UiName, ctx : &mut DefaultContext<Self::Global>) -> Option<Self::Input> { None }

    fn get_input_ui(&mut self, input_provider : &mut Self::InputProvider, ctx : &mut DefaultContext<Self::Global>) -> Self::Input
        where Self::Input : Default 
    {
        UiManager::<Self>::get_input_ui(self, input_provider, ctx)
    }
}

/* 
pub trait UiInputProvider<Owner : UiOwner + IGame> : IInputProvider<I,G,Glob>
    where G : UiOwner<Name, Glob>
{ 
    fn get_input_ui(&mut self, name : &Name, ui : &UiManager<Name, G, Glob>, game : &mut G, time : DrawTime, ctx : &mut DefaultContext<Glob>) -> I;
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct UiManager<Owner : UiOwner>
{
    pages : Vec<UiPage<Owner>>
}
impl<Owner : UiOwner> Deref for UiManager<Owner>
{
    type Target=UiPage<Owner>;

    fn deref(&self) -> &Self::Target {
        self.pages.last().unwrap()
    }
}
impl<Owner : UiOwner> DerefMut for UiManager<Owner>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pages.last_mut().unwrap()
    }
}
impl<Owner : UiOwner> Default for UiManager<Owner>
{
    fn default() -> Self { Self::empty() }
}
impl<Owner : UiOwner> UiManager<Owner>
{
    pub fn empty() -> Self { Self { pages: vec![] }}

    pub fn new(page_name : Owner::UiPageName) -> Self 
    { 
        let mut s = Self::empty();
        s.push_page(page_name);
        s 
    }

    pub fn get_input_ui(game : &mut Owner, input_provider : &mut <Owner as IGame>::InputProvider, ctx : &mut DefaultContext<Owner::Global>) -> Owner::Input
        where <Owner as IGame>::Input : Default
    {
        UiPage::<Owner>::get_input_ui(game, input_provider, ctx)
    }

    pub fn push_page(&mut self, page_name : Owner::UiPageName)
    {
        self.pop_page_named(&page_name);
        self.pages.push(UiPage::new(page_name));
        self.proto_reset();
    }

    pub fn pop_page_named(&mut self, page_name : &Owner::UiPageName)
    {
        self.pages.retain(|e| &e.page_name != page_name);
    }

    pub fn clear_pages(&mut self)
    {
        self.pages.clear();
    }

    pub fn pop_page(&mut self)
    {
        self.pages.pop();
    }
    



    //pub fn update_ui(&mut self, time : GameTime, ctx : &mut DefaultContext<Owner::Global>)
    //pub fn get_input_ui(game : &mut Owner, input_provider : &mut <Owner as IGame>::InputProvider, time : GameTime, ctx : &mut DefaultContext<Owner::Global>) -> Owner::Input where <Owner as IGame>::Input : Default

    pub fn draw(&self, owner : &Owner, ctx : &mut DefaultContext<Owner::Global>)
    {
        for p in self.pages.iter()
        {
            p.draw(owner, ctx)
        }
    }
    pub fn resolution_changed(&mut self, ctx : &mut DefaultContext<Owner::Global>)
    {
        for p in self.pages.iter_mut()
        {
            p.resolution_changed(ctx)
        }
    }
    pub fn build(&mut self, ctx : &mut DefaultContext<Owner::Global>)
    {
        for p in self.pages.iter_mut()
        {
            p.build(ctx)
        }
    }


}

/// A flatten graph that store all UiElement
#[derive(Clone, Debug, PartialEq)]
pub struct UiPage<Owner : UiOwner>
{
    page_name : Owner::UiPageName,

    named  : HashMap<Owner::UiName, UiIdx>,
    
    /// Indexed by UiIdx
    all    : Vec<Option<UiElement<Owner>>>,
    free_slot : Vec<UiIdx>,

    time : Time,
    
    //input : Vec<UiInput<Owner>>,
    
    window_size_px : Vec2,

    builder : UiBuilder<Owner>,
}
impl<Owner : UiOwner> Deref for UiPage<Owner>
{
    type Target=UiBuilder<Owner>; fn deref(&self) -> &Self::Target { &self.builder }
}
impl<Owner : UiOwner> DerefMut for UiPage<Owner>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.builder }
}


impl<Owner : UiOwner> Index<Owner::UiName> for UiPage<Owner>
{
    type Output=UiElement<Owner>;
    fn index(&self, name: Owner::UiName) -> &Self::Output {
        self.get_by_name(&name)
    }
}
impl<Owner : UiOwner> IndexMut<Owner::UiName> for UiPage<Owner>
{
    fn index_mut(&mut self, name: Owner::UiName) -> &mut Self::Output {
        self.get_by_name_mut(&name)
    }
}

const ROOT_ID : UiIdx = 0;

impl<Owner : UiOwner> UiPage<Owner>
{
    pub fn new(page_name  : Owner::UiPageName) -> Self 
    {
        let mut s = Self 
        { 
            page_name : page_name,
            named: ___(), 
            all: ___(), 
            
            free_slot: ___(),
            builder: ___(),

            window_size_px: Vec2::ONE,
            time: ___(),
        };

        s._add(UiName::Root).set_pos(UiPos::fullscreen());
        s
    }

    pub fn clear(&mut self)
    {
        self.remove_all_child(ROOT_ID);
    }

    /// Set the proto to the target name
    pub fn set_focus(&mut self, name : Owner::UiName)
    { 
        let v = &self[name];
        let last = v.idx;
        self.builder.proto.set(v.data.clone());
        self.last.set(last);
    }

    fn ui_named_to_idx(&self, name : &UiName<Owner>) -> Option<UiIdx>
    {
        match name
        {
            UiName::Named(n) => self.named.get(n).map(|e| *e),
            _ => None
        }
    }

    fn get_by_name(&self, name : &Owner::UiName) -> &UiElement<Owner> { self.get(self.named.get(name).copied().unwrap()) }
    fn get_by_name_mut(&mut self, name : &Owner::UiName) -> &mut UiElement<Owner> { self.get_mut(self.named.get(name).copied().unwrap()) }

    fn get(&self, idx : UiIdx) -> &UiElement<Owner> { self.all[idx].as_ref().unwrap() }
    fn get_mut(&mut self, idx : UiIdx) -> &mut UiElement<Owner> { self.all[idx].as_mut().unwrap() }

    pub fn remove(&mut self, name : &Owner::UiName) -> bool
    {
        let Some(idx) = self.named.get(name).copied() else { return false; };
        self.remove_idx(idx);
        true
    }

    pub fn root(&self) -> &UiElement<Owner> { self.get(ROOT_ID) }
    pub fn root_mut(&mut self) -> &mut UiElement<Owner> { self.get_mut(ROOT_ID) }

    fn remove_all_child(&mut self, idx : UiIdx)
    {
        let mut elem = self.get(idx);

        while !elem.child.is_empty()
        {
            self.remove_idx(elem.child.last().copied().unwrap());
            elem = self.get(idx);
        }
    }

    fn remove_idx(&mut self, idx : UiIdx)
    {
        self.last.remove_all(&idx);

        let elem: &UiElement<Owner> = self.get(idx);
        let (elem_idx, elem_name) = (elem.idx, elem.name.clone());

        self.get_mut(elem.parent).remove_child(elem_idx);
        
        self.remove_all_child(idx);
        
        elem_name.try_as_named().map(|n| self.named.remove(n));
        self.free_slot.push(elem_idx);
        self.all[idx] = None;
    }

    pub fn add_lambda(&mut self) -> &mut UiElement<Owner> { self._add(UiName::Lambda) }
    pub fn add_named(&mut self, name : Owner::UiName) -> &mut UiElement<Owner> { self._add(UiName::Named(name)) }
    pub fn _add(&mut self, name : UiName<Owner>) -> &mut UiElement<Owner>
    {
        name.try_as_named().map(|n| self.remove(n));

        if self.free_slot.is_empty()
        {
            self.free_slot.push(self.all.len());
            self.all.push(None);
        }

        let idx = self.free_slot.pop().unwrap();

        let parent_idx = *self.last.last();
        if idx != ROOT_ID
        {
            self.get_mut(parent_idx).add_child(idx);
        }

        self.all[idx] = Some(UiElement::<Owner>::new(name.clone(), idx, parent_idx, self.proto.clone_last()));

        name.try_as_named().map(|n| self.named.insert(n.clone(), idx));
        self.last.push();
        self.last.set(idx);
        self.proto_set_fullscreen();

        self.get_mut(idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = &UiElement<Owner>>
    {
        self.all.iter().filter_map(|e| e.as_ref())
    }
}


impl<Owner : UiOwner> UiPage<Owner>
{
    pub fn move_by(&mut self, delta : UiVec) { self.proto.pos.for_src_and_dest_mut(|e| { e.move_by(delta); }); }

    pub fn pos_pop_rest(&mut self) 
    { 
        self.proto.pos = self.pos_margin_rest.pop().expect("forgot to add a margin");
        self.last_pop();
    }
    pub fn pos_pop_rest_x2(&mut self) 
    {
        self.pos_pop_rest();
        self.pos_pop_rest();
    }
    pub fn pos_pop_rest_x4(&mut self) 
    {
        self.pos_pop_rest_x2();
        self.pos_pop_rest_x2();
    }

    


    pub fn pos_push_margin_top(&mut self, margin_top : UiUnit) -> &mut Self
    { 
        let mut rest = self.proto.pos.clone();
        rest.for_all_mut(|e| { e.glue_bot(margin_top); });

        self.proto.pos.for_all_mut(|e| { e.add_margin_top(margin_top);});

        rest.for_all_mut(|e| { e.move_by(self.proto.pos.dest().size._0_y()); });
        self.pos_margin_rest.push(rest);
        
        self.last.push();

        self 
    }

    pub fn pos_push_margin_bot(&mut self, margin_bot : UiUnit) -> &mut Self
    { 
        let mut rest = self.proto.pos.clone();
        rest.for_all_mut(|e| { e.glue_top(margin_bot);});

        self.proto.pos.for_all_mut(|e| { e.add_margin_bot(margin_bot);});

        rest.for_all_mut(|e| { e.move_by(self.proto.pos.dest().size._0_ry());});
        self.pos_margin_rest.push(rest);
        
        self.last.push();

        self 
    }

    pub fn pos_push_margin_right(&mut self, margin_right : UiUnit) -> &mut Self
    { 
        let mut rest = self.proto.pos.clone();
        rest.for_all_mut(|e| { e.glue_left(margin_right);});

        self.proto.pos.for_all_mut(|e| { e.add_margin_right(margin_right);});

        rest.for_all_mut(|e| { e.move_by(self.proto.pos.dest().size.x_0());});
        self.pos_margin_rest.push(rest);
        
        self.last.push();

        self 
    }

    pub fn pos_push_margin_left(&mut self, margin_left : UiUnit) -> &mut Self
    { 
        let mut rest = self.proto.pos.clone();
        rest.for_all_mut(|e| { e.glue_right(margin_left);});

        self.proto.pos.for_all_mut(|e| { e.add_margin_left(margin_left);});

        rest.for_all_mut(|e| { e.move_by(self.proto.pos.dest().size.rx_0());});
        self.pos_margin_rest.push(rest);
        
        self.last.push();

        self 
    }


    /// Push 2 margins
    pub fn pos_push_x2_margin_left_right(&mut self, margin_left_and_right : UiUnit) -> &mut Self
    { 
        self.pos_push_margin_left(margin_left_and_right);
        self.pos_push_margin_right(margin_left_and_right);
        self
    }

    /// Push 2 margins
    pub fn pos_push_x2_margin_top_bot(&mut self, margin_top_and_bot : UiUnit) -> &mut Self
    { 
        self.pos_push_margin_top(margin_top_and_bot);
        self.pos_push_margin_bot(margin_top_and_bot);
        self
    }

    /// Push 4 margins
    pub fn pos_push_x4_margin_left_right_top_bot(&mut self, margin_left_right_top_bot : UiUnit) -> &mut Self
    { 
        self.pos_push_x2_margin_left_right(margin_left_right_top_bot);
        self.pos_push_x2_margin_top_bot(margin_left_right_top_bot);
        self
    }

    /// Push 4 margins
    pub fn pos_push_x4_margin_left_right_top_bot_vec(&mut self, margin_left_right_top_bot : UiVec) -> &mut Self
    { 
        self.pos_push_x2_margin_left_right(margin_left_right_top_bot.x);
        self.pos_push_x2_margin_top_bot(margin_left_right_top_bot.y);
        self
    }


    pub fn pos_push_glue_bot(&mut self, size_bot : UiUnit) -> &mut Self  
    {
        self.pos_push_margin_top(self.proto.pos.dest().size.y - size_bot);
        self
    }

    pub fn pos_push_glue_top(&mut self, size_top : UiUnit) -> &mut Self  
    {
        self.pos_push_margin_bot(self.proto.pos.dest().size.y - size_top);
        self
    }

    pub fn pos_push_glue_right(&mut self, size_right : UiUnit) -> &mut Self  
    {
        self.pos_push_margin_left(self.proto.pos.dest().size.x - size_right);
        self
    }

    pub fn pos_push_glue_left(&mut self, size_left : UiUnit) -> &mut Self  
    {
        self.pos_push_margin_right(self.proto.pos.dest().size.x - size_left);
        self
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum UiSplitWeight
{
    Weight(real),
    Unit(UiUnit),
}
impl UiSplitWeight
{
    pub fn square() -> Self { Self::Unit(1.ui_min()) }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct UiSplit
{
    weight : UiSplitWeight,
    z : real,
}
impl UiSplit
{
    pub fn new_square() -> Self { Self::new(UiSplitWeight::square()) }
    pub fn new_with_weight(weight : real) -> Self { Self::new(UiSplitWeight::Weight(weight)) }
    pub fn new_one_weight() -> Self { Self::new_with_weight(1.)}

    pub fn new(weight : UiSplitWeight) -> Self
    {
        Self { weight, z : 0. }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UiElementInput<Owner : UiOwner>
{
    time_hover : AlsoStoreOld<bool, Time>,
    time_press : AlsoStoreOld<bool, Time>,

    interractive : bool,

    disable : bool,
    // if none execute itself.
    // if some press another button instead, LoL
    on_press_call_other_ui_instead : Option<UiName<Owner>>,

    pressed_input   : Option<Owner::Input>,
    released_input  : Option<Owner::Input>,

    hover_in_input  : Option<Owner::Input>,
    hover_out_input : Option<Owner::Input>,

    sound : UiElementInputSound,
}
#[derive(Clone, PartialEq, Debug, Default)]
pub struct UiElementInputSound
{
    hover_in  : Option<Sound>,
    hover_out : Option<Sound>,

    pressed  : Option<Sound>,
    released : Option<Sound>,
}
impl UiElementInputSound
{
    pub fn add_hover_in(mut self, sound : &Sound) -> Self { self.set_hover_in(Some(sound)); self }
    pub fn set_hover_in(&mut self, sound : Option<&Sound>) -> &mut Self { self.hover_in = sound.map(|e| e.clone()); self}
    pub fn hover_in(&self) -> &Option<Sound> { &self.hover_in }

    pub fn add_hover_out(mut self, sound : &Sound) -> Self { self.set_hover_out(Some(sound)); self }
    pub fn set_hover_out(&mut self, sound : Option<&Sound>) -> &mut Self { self.hover_out = sound.map(|e| e.clone()); self}
    pub fn hover_out(&self) -> &Option<Sound> { &self.hover_out }

    pub fn add_pressed(mut self, sound : &Sound) -> Self { self.set_pressed(Some(sound)); self }
    pub fn set_pressed(&mut self, sound : Option<&Sound>) -> &mut Self { self.pressed = sound.map(|e| e.clone()); self}
    pub fn pressed(&self) -> &Option<Sound> { &self.pressed }

    pub fn add_released(mut self, sound : &Sound) -> Self { self.set_released(Some(sound)); self }
    pub fn set_released(&mut self, sound : Option<&Sound>) -> &mut Self { self.released = sound.map(|e| e.clone()); self}
    pub fn released(&self) -> &Option<Sound> { &self.released }

    pub fn clear(&mut self)
    {
        self.set_hover_in(None);
        self.set_hover_out(None);
        self.set_pressed(None);
        self.set_released(None);
    }
}

impl<Owner : UiOwner> Default for UiElementInput<Owner>
{
    fn default() -> Self {
        Self 
        {
            time_hover: ___(),
            time_press: ___(),
            disable: ___(),
            on_press_call_other_ui_instead: ___(),

            sound: ___(),

            pressed_input: ___(),
            released_input: ___(),
            hover_in_input: ___(),
            hover_out_input: ___(),
            interractive: ___(),
        }
    }
}
impl<Owner : UiOwner> UiElementInput<Owner>
{
    fn new() -> Self { ___() }

    pub fn hover(&self) -> &AlsoStoreOld<bool, Time> { &self.time_hover }
    pub fn press(&self) -> &AlsoStoreOld<bool, Time> { &self.time_press }

    /// Want to do some trolling ?
    pub fn set_on_press_call_other_ui_instead(&mut self, on_press_call_other_ui : Option<Owner::UiName>) -> &mut Self 
    { self.on_press_call_other_ui_instead = on_press_call_other_ui.map(|n| UiName::Named(n)); self }

    pub fn with_sound(mut self, sound : UiElementInputSound) -> Self { self.set_sound(sound); self }
    pub fn set_sound(&mut self, sound : UiElementInputSound) -> &mut Self { self.sound = sound; self }

    pub fn sound(&self) -> &UiElementInputSound { &self.sound }
    pub fn sound_mut(&mut self) -> &mut UiElementInputSound { &mut self.sound }

    pub fn clear_sound(&mut self)
    {
        self.sound.clear();
    }

    /// add an input when the button is pressed
    pub fn add_pressed(&mut self, pressed : Owner::Input) -> &mut Self { self.set_pressed(Some(pressed)) }
    /// set the input when the button is pressed
    pub fn set_pressed(&mut self, pressed : Option<Owner::Input>) -> &mut Self { self.pressed_input = pressed; self }
    /// input executed when pressed
    pub fn pressed(&self) -> &Option<Owner::Input> { &self.pressed_input }

    /// add an input when the button is released
    pub fn add_released(&mut self, released : Owner::Input) -> &mut Self { self.set_released(Some(released)) }
    /// set the input when the button is released
    pub fn set_released(&mut self, released : Option<Owner::Input>) -> &mut Self { self.released_input = released; self }
    /// input executed when released
    pub fn released(&self) -> &Option<Owner::Input> { &self.released_input }

    /// add an input when the button is hover_in
    pub fn add_hover_in(&mut self, hover_in : Owner::Input) -> &mut Self { self.set_hover_in(Some(hover_in)) }
    /// set the input when the button is hover_in
    pub fn set_hover_in(&mut self, hover_in : Option<Owner::Input>) -> &mut Self { self.hover_in_input = hover_in; self }
    /// input executed when hover_in
    pub fn hover_in(&self) -> &Option<Owner::Input> { &self.hover_in_input }

    /// add an input when the button is hover_out
    pub fn add_hover_out(&mut self, hover_out : Owner::Input) -> &mut Self { self.set_hover_out(Some(hover_out)) }
    /// set the input when the button is hover_out
    pub fn set_hover_out(&mut self, hover_out : Option<Owner::Input>) -> &mut Self { self.hover_out_input = hover_out; self }
    /// input executed when hover_out
    pub fn hover_out(&self) -> &Option<Owner::Input> { &self.hover_out_input }


    pub fn add_interactivity(&mut self) -> &mut Self { self.set_interactivity(true) }
    pub fn set_interactivity(&mut self, is_interractive : bool) -> &mut Self { self.interractive = is_interractive; self }
    pub fn is_interractive(&self) -> bool { self.interractive }

    pub fn enable(&mut self) -> &mut Self { self.set_disable(false) }
    pub fn disable(&mut self) -> &mut Self { self.set_disable(true) }

    pub fn is_disable(&self) -> bool { self.disable }
    pub fn is_enable(&self)  -> bool { !self.is_disable() }

    pub fn set_disable(&mut self, disable : bool) -> &mut Self { self.disable = disable; self }
    pub fn set_enable(&mut self, enable : bool) -> &mut Self { self.set_disable(!enable) }

    /// Include interactivity + disable but not activate
    pub fn can_interract_if_activate(&self) -> bool { self.is_interractive() && self.is_enable() }
}

/// Have an identity
#[derive(Clone, Debug, PartialEq)]
pub struct UiElement<Owner : UiOwner>
{
    name : UiName<Owner>,
    idx  : UiIdx,

    parent : UiIdx,

    data  : UiElementData<Owner>,

    is_build : bool,

    /// if the element is used for the input, update, draw...
    is_active : bool,

    child  : Vec<UiIdx>,
    child_draw_order  : Vec<UiIdx>,

    //_owner : PhantomData<Owner>,
    //_glob  : PhantomData<Owner::Global>,
}
impl<Owner : UiOwner> UiElement<Owner>
{
    pub fn iter_child(&self) -> impl Iterator<Item = UiIdx> + '_ { self.child.iter().copied() }

    fn new(name : UiName<Owner>, idx : UiIdx, parent_ixd : UiIdx, data : UiElementData<Owner>) -> Self
    {
        Self { name, idx, parent: parent_ixd, data, child: ___(), is_build: false, is_active: true, child_draw_order: ___() }
    }

    fn add_child(&mut self, child : UiIdx)
    {
        self.child.push(child);
    }

    fn remove_child(&mut self, idx : UiIdx)
    {
        if idx == ROOT_ID { return; }
        self.child.remove(self.child.iter().position(|e| *e == idx).unwrap());
    }
}

impl<Owner : UiOwner> UiElement<Owner>
{
    pub fn name(&self) -> &UiName<Owner> { &self.name }

    pub fn input_name(&self) -> &UiName<Owner>
    { 
        if self.is_interractive() 
        { 
            match &self.input.on_press_call_other_ui_instead
            {
                Some(v) => v,
                None => self.name(),
            }
        } else { self.name() }
    }
}

impl<Owner : UiOwner> Deref for UiElement<Owner> { type Target=UiElementData<Owner>; fn deref(&self) -> &Self::Target { &self.data }}
impl<Owner : UiOwner> DerefMut for UiElement<Owner> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data }}

/// Don't have any identity
#[derive(Clone, Debug, PartialEq)]
pub struct UiElementData<Owner : UiOwner>
{
    pub param  : UiElementParams<Owner>,
    pub pos    : UiPos,
}

impl<Owner : UiOwner> Deref for UiElementData<Owner>
{
    type Target=UiElementParams<Owner>;
    fn deref(&self) -> &Self::Target { &self.param }
}
impl<Owner : UiOwner> DerefMut for UiElementData<Owner>
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.param }
}

pub trait ToUiSprite<Global>
{
    fn to_ui_sprite(self, ctx : &DefaultContext<Global>) -> UiSprite;
}
impl<Global> ToUiSprite<Global> for UiSprite { fn to_ui_sprite(self, _ctx : &DefaultContext<Global>) -> UiSprite { self }}
impl<Global> ToUiSprite<Global> for &Texture2D { fn to_ui_sprite(self, _ctx : &DefaultContext<Global>) -> UiSprite { UiSprite::new(self) }}

pub trait ToUiIcon<Global> : Sized
{
    fn to_ui_icon(self, ctx : &DefaultContext<Global>) -> UiIcon;
}

impl<Global> ToUiIcon<Global> for UiSprite 
{
    fn to_ui_icon(self, _ctx : &DefaultContext<Global>) -> UiIcon {
        UiIcon::new(Some(self))
    }
}
impl<Global> ToUiIcon<Global> for &Texture2D
{
    fn to_ui_icon(self, _ctx : &DefaultContext<Global>) -> UiIcon {
        UiIcon::new_from_texture(self)
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiInterractiveState
{
    Normal,
    Hover,
    Hold,
    Pressed,
}
impl UiInterractiveState
{
    pub fn is_normal(&self) -> bool { matches!(self, UiInterractiveState::Normal) }
    pub fn is_hover(&self) -> bool { matches!(self, UiInterractiveState::Hover) }
    pub fn is_hold(&self) -> bool { matches!(self, UiInterractiveState::Hold) }
    pub fn is_pressed(&self) -> bool { matches!(self, UiInterractiveState::Pressed) }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiSprite
{
    pub normal  : Sprite,
    pub hover   : Sprite,
    pub pressed : Sprite,
}
impl UiSprite
{
    pub fn new(texture : &Texture2D) -> Self { Self::new_from_sprite(Sprite::new(texture)) }
    pub fn new_from_sprite(sprite : Sprite) -> Self 
    { Self::new_full(sprite.clone(), sprite.clone(), sprite) }

    pub fn new_full(normal : Sprite, hover : Sprite, pressed : Sprite) -> Self { Self { normal, hover, pressed }}

    /// Each sprite follow the previous from left to right in the following order : (normal, hover, pressed)
    pub fn new_animated_from_idx(texture : &Texture2D, idx : usize) -> Self
    {
        Self::new_full(
            Sprite::new_with_param(texture, DrawTexture::source(texture.sheet_rect_from_idx(idx*3))),
            Sprite::new_with_param(texture, DrawTexture::source(texture.sheet_rect_from_idx(idx*3+1))),
            Sprite::new_with_param(texture, DrawTexture::source(texture.sheet_rect_from_idx(idx*3+2))),
        )
    }

    /// Each sprite follow the previous from left to right in the following order : (normal, hover, pressed)
    pub fn new_animated_from_point2(texture : &Texture2D, idx : Point2) -> Self
    {
        Self::new_animated_from_idx(texture, texture.sheet_point2_to_idx(idx))
    }

    fn new_with_param(texture : &Texture2D, params : DrawTexture, offset : Vec2) -> Self
    {
        Self::new_full
        (
            Sprite::new_with_param(texture, params),
            Sprite::new_with_param(texture, {let mut p = params; p.source.as_mut().map(|e| e.move_by(offset)); p}),
            Sprite::new_with_param(texture, {let mut p = params; p.source.as_mut().map(|e| e.move_by(offset*2.)); p}),
        )
    }

    pub fn current_sprite(&self, draw_state : UiInterractiveState) -> &Sprite
    {
        match draw_state
        {
            UiInterractiveState::Normal  => &self.normal,
            UiInterractiveState::Hover   => &self.hover,
            UiInterractiveState::Hold    => &self.pressed,
            UiInterractiveState::Pressed => &self.pressed,
        }
    }

    pub fn map<F>(&mut self, f : F) where F : Fn(&mut Sprite)
    {
        f(&mut self.hover); f(&mut self.normal); f(&mut self.pressed);
    }

    pub fn draw<Glob>(&self, draw_state : UiInterractiveState, px : Rect2, ctx : &mut DefaultContext<Glob>)
    {
        self.current_sprite(draw_state).draw(px, ctx);
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UiIconFillMode
{
    Center(Vec2),
    Stretch,
}
impl Default for UiIconFillMode
{
    fn default() -> Self { Self::Center(half()) }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UiIcon
{
    pub sprite    : Option<UiSprite>,
    pub fill_mode : UiIconFillMode,
    /// The margin for both side
    pub margin    : UiVec,
}

impl From<UiSprite> for UiIcon { fn from(sprite: UiSprite) -> Self { Self::new(Some(sprite)) }}
impl From<Sprite> for UiIcon { fn from(sprite: Sprite) -> Self { Self::new_from_sprite(sprite) }}
impl From<&Texture2D> for UiIcon { fn from(texture: &Texture2D) -> Self { Self::new_from_texture(texture) }}

impl UiIcon
{
    pub fn new(sprite : Option<UiSprite>) -> Self { Self { sprite, fill_mode : ___(), margin : zero() } }
    pub fn new_from_texture(texture : &Texture2D) -> Self { Self::new(Some(UiSprite::new(texture))) }
    pub fn new_from_sprite(sprite : Sprite) -> Self { Self::new(Some(UiSprite::new_from_sprite(sprite))) }
    
    pub fn margin(&self) -> UiVec { self.margin }
    pub fn with_margin(mut self, margin_both_side : UiVec) -> Self { (&mut self).set_margin(margin_both_side); self }
    pub fn set_margin(&mut self, margin_both_side : UiVec) -> &mut Self { self.margin = margin_both_side; self}

    pub fn fill_mode(&self) -> UiIconFillMode { self.fill_mode }
    pub fn with_fill_mode(mut self, fill_mode : UiIconFillMode) -> Self { (&mut self).set_fill_mode(fill_mode); self }
    pub fn set_fill_mode(&mut self, fill_mode : UiIconFillMode) -> &mut Self { self.fill_mode = fill_mode; self}

    pub fn add_sprite(&mut self, sprite : UiSprite) -> &mut Self { self.set_sprite(Some(sprite)) }
    pub fn set_sprite(&mut self, sprite : Option<UiSprite>) -> &mut Self { self.sprite = sprite; self }
    pub fn remove_sprite(&mut self) { self.set_sprite(None); }

    pub fn draw<Glob>(&self, draw_state : UiInterractiveState, mut rect_px : Rect2, ctx : &mut DefaultContext<Glob>)
    {
        if let Some(sprite) = &self.sprite
        {
            let s = self.margin.to_px(ctx.cam());
            rect_px.size -= s * 2.;
            rect_px.pos  += s;
    
            match self.fill_mode
            {
                UiIconFillMode::Center(coef) => sprite.draw(draw_state, rect_px.put_inside(sprite.current_sprite(draw_state).size(), coef), ctx),
                UiIconFillMode::Stretch => sprite.draw(draw_state, rect_px, ctx),
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct UiElementParams<Owner : UiOwner>
{
    pub color        : Option<Color>,

    pub nine_slice   : Option<Box<UiNineSlice>>,
    pub text         : UiText<Owner>,

    pub icon         : UiIcon,

    pub input        : UiElementInput<Owner>,

    /// The split config if this element have any child
    pub split_on        : UiSplitOn,

    /// The split config if this element is added as a child
    pub in_split  : Option<UiSplit>,
}
impl<Owner : UiOwner> Default for UiElementParams<Owner>
{
    fn default() -> Self {
        Self 
        { 
            color: ___(),
            nine_slice: ___(),
            text: ___(),
            icon: ___(),
            input: ___(),
            split_on: ___(),
            in_split: ___(),
        }
    }
}

impl<Owner : UiOwner> UiElement<Owner>
{
    pub fn with_pos(mut self, pos : UiPos) -> Self { self.pos = pos; self }
    pub fn set_pos(&mut self, pos : UiPos) -> &mut Self { self.pos = pos; self }

    pub fn set_visual(&mut self, visual : UiElementParams<Owner>) -> &mut Self
    { self.param = visual; self }
    
    pub fn add_color(&mut self, c : Color) -> &mut Self { self.set_color(Some(c))}
    pub fn set_color(&mut self, c : Option<Color>) -> &mut Self
    {
        self.color = c;
        self
    }

    pub fn add_nine_slice(&mut self, slice : UiNineSlice) -> &mut Self { self.set_nine_slice(Some(slice)) }
    pub fn set_nine_slice(&mut self, slice : Option<UiNineSlice>) -> &mut Self
    {
        self.nine_slice = slice.map(Box::new);
        self
    }

    pub fn set_nine_slice_angle_size(&mut self, angle_size : UiVec) -> &mut Self
    {
        self.nine_slice.as_mut().map(|e| { e.angle_size = angle_size});
        self
    }



    /// Add a sprite that will be display in the element
    pub fn add_sprite(&mut self, sprite : UiSprite) -> &mut Self { self.set_sprite(Some(sprite)); self}
    /// Add a sprite that will be display in the element
    pub fn set_sprite(&mut self, sprite : Option<UiSprite>) -> &mut Self
    {
        self.icon.sprite = sprite; self
    }

    /// A more flexible version of add_sprite
    pub fn add_icon(&mut self, icon : UiIcon) -> &mut Self { self.set_icon(icon) }
    /// A more flexible version of add_sprite
    pub fn set_icon(&mut self, icon : UiIcon) -> &mut Self
    {
        self.icon = icon;
        self
    }
    pub fn set_icon_margin(&mut self, margin : UiVec) -> &mut Self 
    {
        self.icon.set_margin(margin);
        self
    }
    pub fn set_icon_fill_mode(&mut self, fill_mode : UiIconFillMode) -> &mut Self 
    {
        self.icon.set_fill_mode(fill_mode);
        self
    }

    pub fn remove_icon(&mut self) -> &mut Self { self.icon.remove_sprite(); self }


    pub fn in_split_default(&mut self) -> &mut Self { self.in_split_weight(1.) }
    pub fn in_split_weight(&mut self, weight : real) -> &mut Self { self.in_split_with(UiSplit::new_with_weight(weight)) }
    pub fn in_split_square(&mut self) -> &mut Self { self.in_split_with(UiSplit::new_square()) }
    pub fn in_split_unit(&mut self, unit : UiUnit) -> &mut Self { self.in_split_with(UiSplit::new(UiSplitWeight::Unit(unit))) }
    
    
    pub fn in_split_with(&mut self, in_split : UiSplit) -> &mut Self { self.set_in_split(Some(in_split)) }
    pub fn set_in_split(&mut self, in_split : Option<UiSplit>) -> &mut Self
    {
        self.in_split = in_split;
        self
    }

    pub fn is_active(&self) -> bool { self.is_active }
    pub fn is_inactive(&self) -> bool { !self.is_active() }

    pub fn set_active(&mut self, active : bool) -> &mut Self { self.is_active = active; self }
    pub fn activate(&mut self) -> &mut Self { self.set_active(true) }
    pub fn desactivate(&mut self) -> &mut Self { self.set_active(false) }

    fn set_ui_text(&mut self, text : UiText::<Owner>) -> &mut Self
    {
        self.text = text;
        self
    }

    pub fn add_text(&mut self, txt : String) -> &mut Self { self.set_ui_text(UiText::new(UiString::new(txt))) }
    pub fn set_text_font(&mut self, font : Option<Font>) -> &mut Self { self.text.txt.set_font(font); self }
    
    pub fn in_split_set_z_idx(&mut self, z : real) -> &mut Self 
    { 
        match &mut self.in_split
        {
            Some(v) => v.z = z,
            None => {},
        }
        self
    }
    pub fn z_idx(&self) -> real
    {
        match &self.in_split
        {
            Some(v) => v.z,
            None => { real::MAX },
        }
    }
}

// Inputs :
impl<Owner : UiOwner> UiElement<Owner> 
{
    pub fn input(&self) -> &UiElementInput<Owner> { &self.input }
    pub fn input_mut(&mut self) -> &mut UiElementInput<Owner> { &mut self.input }

    /// Add an input when the button is pressed.
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn add_input_pressed(&mut self, pressed : Owner::Input) -> &mut Self { self.set_input_pressed(Some(pressed)); self }
    /// Set the input when the button is pressed
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn set_input_pressed(&mut self, pressed : Option<Owner::Input>) -> &mut Self { if !self.is_build { self.add_interactivity(); } self.input.set_pressed(pressed); self }
    /// Input executed when pressed
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn input_pressed(&self) -> &Option<Owner::Input> { self.input.pressed() }
    pub fn add_input_pressed_sound(&mut self, sound : &Sound) -> &mut Self { self.set_input_pressed_sound(Some(sound)) }
    pub fn set_input_pressed_sound(&mut self, sound : Option<&Sound>) -> &mut Self { self.input.sound.set_pressed(sound); self }


    /// Add an input when the button is released
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn add_input_released(&mut self, released : Owner::Input) -> &mut Self { self.set_input_released(Some(released)); self }

    /// Set the input when the button is released
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn set_input_released(&mut self, released : Option<Owner::Input>) -> &mut Self { if !self.is_build { self.add_interactivity(); } self.input.set_released(released); self }
    
    /// Input executed when released
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn input_released(&self) -> &Option<Owner::Input> { self.input.released() }
    pub fn add_input_released_sound(&mut self, sound : &Sound) -> &mut Self { self.set_input_released_sound(Some(sound)) }
    pub fn set_input_released_sound(&mut self, sound : Option<&Sound>) -> &mut Self { self.input.sound.set_released(sound); self }


    /// Add an input when the button is hover_in
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn add_input_hover_in(&mut self, hover_in : Owner::Input) -> &mut Self { self.set_input_hover_in(Some(hover_in)); self }
    /// Set the input when the button is hover_in
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn set_input_hover_in(&mut self, hover_in : Option<Owner::Input>) -> &mut Self { if !self.is_build { self.add_interactivity(); } self.input.set_hover_in(hover_in); self }
    /// Input executed when hover_in
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn input_hover_in(&self) -> &Option<Owner::Input> { self.input.hover_in() }
    pub fn add_input_hover_in_sound(&mut self, sound : &Sound) -> &mut Self { self.set_input_hover_in_sound(Some(sound)) }
    pub fn set_input_hover_in_sound(&mut self, sound : Option<&Sound>) -> &mut Self { self.input.sound.set_hover_in(sound); self }


    /// Add an input when the button is hover_out
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn add_input_hover_out(&mut self, hover_out : Owner::Input) -> &mut Self { self.set_input_hover_out(Some(hover_out)); self }
    /// Set the input when the button is hover_out
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn set_input_hover_out(&mut self, hover_out : Option<Owner::Input>) -> &mut Self { if !self.is_build { self.add_interactivity(); } self.input.set_hover_out(hover_out); self }
    /// Input executed when hover_out
    /// 
    /// Also define the UI element as interactive if it is not build.
    pub fn input_hover_out(&self) -> &Option<Owner::Input> { self.input.hover_out() }
    pub fn add_input_hover_out_sound(&mut self, sound : &Sound) -> &mut Self { self.set_input_hover_out_sound(Some(sound)) }
    pub fn set_input_hover_out_sound(&mut self, sound : Option<&Sound>) -> &mut Self { self.input.sound.set_hover_out(sound); self }

    pub fn input_sound_clear(&mut self) -> &mut Self { self.input.sound.clear(); self }


    /// Will react to any input. Automaticcaly added if any input is defined when building
    pub fn add_interactivity(&mut self) -> &mut Self { self.input.add_interactivity(); self }
    /// Will react to any input. Automaticcaly added if any input is defined when building
    pub fn set_interactivity(&mut self, is_interractive : bool) -> &mut Self { self.input.set_interactivity(is_interractive); self }
    /// If the UI Element can react to event
    pub fn is_interractive(&self) -> bool { self.input.is_interractive() }

    pub fn disable_input(&mut self) -> &mut Self { self.input.disable(); self }
    pub fn enable_input(&mut self) -> &mut Self { self.input.enable(); self }

    pub fn set_input_disable(&mut self, disable : bool) -> &mut Self { self.input.set_disable(disable); self }
    pub fn set_input_enable(&mut self, disable : bool) -> &mut Self { self.input.set_enable(disable); self }

    pub fn is_input_disable(&self) -> bool { self.input.is_disable() }
    pub fn is_input_enable(&self) -> bool { self.input.is_enable() }

    /// Include interactivity + disable + active
    pub fn input_can_interract(&self) -> bool { self.is_active && self.input.can_interract_if_activate() }
}

impl<Owner : UiOwner> Default for UiElementData<Owner>
{
    fn default() -> Self 
    {
        Self 
        {
            param: ___(),
            pos   : ___(),
        }
    }
}

/// Ui Relative when drawing and getting the input
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct UiRelative
{
    /// offset to add
    pixel_offset : Vec2,
    //parent_pos : UiVec,
    // Todo : add scaling ?
}
impl UiRelative
{
    pub fn new(pixel_offset : Vec2) -> Self
    {
        Self { pixel_offset }
    }
}


impl<Owner : UiOwner> UiElement<Owner>
{
    fn is_hover(ctx : &mut DefaultContext<Owner::Global>) -> bool { ctx.input.touch().any(|t| ctx.pen.cam.current().rect_px.is_inside(t.position_px().cur())) }
    fn is_just_pressed(ctx : &mut DefaultContext<Owner::Global>) -> bool { ctx.input.touch_pressed().any(|t| ctx.pen.cam.current().rect_px.is_inside(t.position_px().cur())) }
    //fn is_just_released<Owner::Global>(ctx : &mut DefaultContext<Owner::Global>) -> bool { ctx.input.touch_just_released().any(|t| ctx.pen.cam.current().rect_px.is_inside(t.position_px().cur())) }

    pub fn interractive_state(&self) -> UiInterractiveState
    {
        if self.is_interractive()
        {
            if self.input.press().cur() { UiInterractiveState::Hold } 
            else if self.input.hover().cur() { UiInterractiveState::Hover } 
            else { UiInterractiveState::Normal }
        }else
        {
            UiInterractiveState::Normal
        }
    }

    pub fn apply_relative_from_pos_and_get_next(&self, relative : UiRelative, pos : UiRect, ctx : &mut DefaultContext<Owner::Global>) -> Rect2
    {
        let mut rect_px = pos.to_px(ctx.cam());
        rect_px.pos += relative.pixel_offset;
        ctx.pen.cam.set_rect_px(rect_px).apply();

        rect_px
    }
    // return a (rect_px, relative)
    pub fn apply_relative_and_get_next(&self, time : Time, relative : UiRelative, ctx : &mut DefaultContext<Owner::Global>) -> Rect2
    {
        self.apply_relative_from_pos_and_get_next(relative, self.pos.get(time), ctx)
    }

    pub fn draw(&self, time : Time, relative : UiRelative, owner : &Owner, ctx : &mut DefaultContext<Owner::Global>) -> UiRelative
    {
        let rect_px = self.apply_relative_and_get_next(time, relative, ctx);
        /* 
        let ui_time = time.total;
        let mut pen = &mut ctx.pen;

        let pos = self.pos;
        let mut rect_px = pos.get(ui_time).to_px(&pen.cam);
        rect_px.pos += relative.pixel_offset;
        pen.cam.set_rect_px(rect_px).apply();
        */

        let mut pen = &mut ctx.pen;

        if let Some(c) = self.color
        {
            pen.fill_world_background(c);
        }

        let interractive_state = self.interractive_state();

        if let Some(background) = &self.nine_slice
        {
            background.draw(interractive_state, rect_px, ctx);
            pen = &mut ctx.pen;
        }

        self.icon.draw(interractive_state, rect_px, ctx);

        self.text.draw(rect_px, time, ctx);


        /* 
        if self.is_interractive() && self.input.hover.cur()
        {
            pen = &mut ctx.pen;
            pen.fill_world_background(WHITE.with_a(0.5));
        }
        pen = &mut ctx.pen;*/

        if let Some(name) = self.name().try_as_named()
        {
            owner.draw_ui_element(name, ctx);
        }
        //self.name().try_as_named().map(|n| owner.draw_ui(n, time, ctx));
        
        UiRelative::new(rect_px.pos)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct UiString<Owner : UiOwner>
{ 
    text   : String,
    words  : Vec<String>,
    lines  : Vec<String>,
    _owner : PhantomData<Owner>,
    font   : Option<Font>,
}
impl<Owner : UiOwner> Default for UiString<Owner>
{
    fn default() -> Self {
        Self { text: ___(), words: ___(), lines: ___(), _owner: ___(), font: ___() }
    }
}
impl<Owner : UiOwner> From<String> for UiString<Owner> { fn from(value: String) -> Self { Self::new(value) }}

impl<Owner : UiOwner> UiString<Owner>
{
    pub fn new(value: String) -> Self { Self { text: value, ..___() }}

    pub fn text(&self) -> &String { &self.text }
    pub fn set_text(&mut self, text : String) -> &mut Self { self.text = text; self } 

    pub fn font(&self) -> &Option<Font> { &self.font }
    pub fn set_font(&mut self, font : Option<Font>) -> &mut Self { self.font = font; self } 

    fn font_scale_px(&self, ctx : &mut DefaultContext<Owner::Global>) -> real
    {
        (1./30.).ui_window_min().to_px_y(ctx.cam())
    }

    fn resolution_changed(&mut self, ctx : &mut DefaultContext<Owner::Global>)
    {
        if self.words.is_empty() 
        {
            for c in self.text.chars()
            {
                if self.words.is_empty() { self.words.push(c.to_string()); continue; }

                if matches!(c, '\r') { continue; }

                if matches!(c, ' ' | '\n')
                {
                    self.words.push(c.to_string());
                    self.words.push("".to_owned());
                }else
                {
                    self.words.last_mut().unwrap().push(c);
                }
            }
            //self.words = self.text.split_inclusive(|e| matches!(e, ' ' | '\r' | '\n')).map(|e| e.to_owned()).collect();
        }
        if self.words.is_empty() { return; }
        self.lines.clear();

        if self.font.is_none() { self.font = ctx.pen.font().map(|e| e.clone()); }

        let size = ctx.cam().parent_size_px();

        let font_scale = self.font_scale_px(ctx); 

        let mut cur_line_length = 0.;
        let max_line_length = size.x;

        let mut w_idx = 0;
        while w_idx < self.words.len()
        {
            // for w in self.words.iter()
            let w = &self.words[w_idx];
            w_idx+=1;

            let (x, mut _y) = ctx.pen.mesure_text(w, self.font.as_ref(), font_scale).into();
            
            let is_new_line = matches!(w.as_str(), "\n");

            if cur_line_length + x >= max_line_length || is_new_line
            {
                cur_line_length = 0.;

                if is_new_line { self.lines.push("".to_owned()) }
                else if x <= max_line_length { self.lines.push(w.clone()) }
                else
                {
                    w_idx-=1;
                    let begin = w[0..1].to_owned();
                    let rest = w[1..].to_owned();
                    self.words[w_idx] = begin;
                    self.words.insert(w_idx+1, rest);
                }
            }else
            {
                if self.lines.is_empty() { self.lines.push(w.clone()) }
                else { self.lines.last_mut().unwrap().push_str(&w) }
            }

            cur_line_length += x;
        }

        //self.txt.resolution_changed(relative, ctx);
    }

    pub fn draw(&self, rect_px : Rect2, time : Time, ctx : &mut DefaultContext<Owner::Global>)
    {
        let text_size = self.font_scale_px(ctx);

        let size_y_px = self.lines.len().to_real() * text_size;
        let delta = rect_px.height() - size_y_px;

        //let coef = (((time.s() / 10.).turn()+0.75.turn()).sin()*0.5+0.5);
        let t = (time.s() / (1. * self.lines.len().to_real())) % 2.;
        let coef = if t <= 1. { t * 1.1 } else { 1. - (t-1.)* 1.1  }.min(1.).max(0.);

        let mut p = rect_px.middle_top();
        p.y -= coef * delta;

        let bot = rect_px.bot_value();
        let top = rect_px.top_value();

        for l in self.lines.iter()
        {
            let (alpha, render) = 

            if p.y >= bot + text_size * 1. && p.y <= top
            {
                if p.y >= bot + text_size * 2. && p.y <= top - text_size
                {
                    (1., true)
                }else 
                {
                    let d = (p.y-(bot+ text_size)).abs().min((p.y-top).abs());
                    if d <= text_size
                    {
                        (d / text_size, true)
                    }else
                    {
                        (0., false)
                    }
                }
            }else
            {
                (0., false)
            };

            if render
            {
                ctx.pen.text(&l, p, text_size, vec2(0.5, 1.), Color::WHITE.with_a(alpha), DrawFont::___().with_optionnal_font(self.font.as_ref()));
            }
            p.y -= text_size;
        }

        
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct UiText<Owner : UiOwner>
{
    pub txt : UiString<Owner>,
}
impl<Owner : UiOwner> Default for UiText<Owner>
{
    fn default() -> Self { Self { txt: ___() } }
}

impl<Owner : UiOwner> UiText<Owner>
{
    pub fn new(txt : UiString::<Owner>) -> Self { Self { txt }}

    pub fn draw(&self, rect_px : Rect2, time : Time, ctx : &mut DefaultContext<Owner::Global>)
    {
        self.txt.draw(rect_px, time, ctx);
    }

    fn resolution_changed(&mut self, ctx : &mut DefaultContext<Owner::Global>)
    {
        self.txt.resolution_changed(ctx);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UiNineSlice
{
    pub nine_slice : NineSlice<UiSprite>,
    angle_size : UiVec,
}
impl Deref for UiNineSlice { type Target=NineSlice<UiSprite>; fn deref(&self) -> &Self::Target { &self.nine_slice }}
impl DerefMut for UiNineSlice { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.nine_slice }}


pub trait ToUiNineSlice<Global>
{
    fn to_ui_nine_slice(self, ctx : &mut DefaultContext<Global>) -> UiNineSlice;
}
impl<Global> ToUiNineSlice<Global> for UiNineSlice { fn to_ui_nine_slice(self, _ctx : &mut DefaultContext<Global>) -> UiNineSlice { self }}

impl UiNineSlice
{
    pub fn new_full(nine_slice : NineSlice<UiSprite>, angle_size : UiVec) -> Self
    {
        Self{nine_slice, angle_size}
    }

    pub fn angle_size(&self) -> UiVec { self.angle_size }
    pub fn set_angle_size(&mut self, angle_size : UiVec) -> &mut Self { self.angle_size = angle_size; self }
    pub fn with_angle(mut self, angle_size : UiVec) -> Self { self.set_angle_size(angle_size); self }

    pub fn new_from_texture(texture : &Texture2D, idx : usize) -> Self { Self::new_from_texture_point2(texture, texture.sheet_idx_to_point2(idx)) }
    pub fn new_from_texture_point2(texture : &Texture2D, idx : Point2) -> Self
    {
        //Self::new_from_texture_custom(texture, texture.sheet_px_size().to_vec2(), texture.sheet_px_size().to_vec2(), texture.sheet_px_margin().to_vec2(), )

        let sprite_offset = (texture.sheet_px_size_with_margin().x_0() * 3).to_vec2();

        let nine_slice : NineSlice<UiSprite> = NineSlice::<UiSprite>::new_full
        (
            UiSprite::new_with_param(&texture, DrawTexture::source(texture.sheet_rect_from_point2(idx + point2(0, 0))), sprite_offset),
            UiSprite::new_with_param(&texture, DrawTexture::source(texture.sheet_rect_from_point2(idx + point2(1, 0))), sprite_offset),
            UiSprite::new_with_param(&texture, DrawTexture::source(texture.sheet_rect_from_point2(idx + point2(2, 0))), sprite_offset),
            
            UiSprite::new_with_param(&texture, DrawTexture::source(texture.sheet_rect_from_point2(idx + point2(0, 1))), sprite_offset),
            UiSprite::new_with_param(&texture, DrawTexture::source(texture.sheet_rect_from_point2(idx + point2(1, 1))), sprite_offset),
            UiSprite::new_with_param(&texture, DrawTexture::source(texture.sheet_rect_from_point2(idx + point2(2, 1))), sprite_offset),
            
            UiSprite::new_with_param(&texture, DrawTexture::source(texture.sheet_rect_from_point2(idx + point2(0, 2))), sprite_offset),
            UiSprite::new_with_param(&texture, DrawTexture::source(texture.sheet_rect_from_point2(idx + point2(1, 2))), sprite_offset),
            UiSprite::new_with_param(&texture, DrawTexture::source(texture.sheet_rect_from_point2(idx + point2(2, 2))), sprite_offset),
        );
        let angle_size = 0.1.ui_window_min().splat2();
        Self::new_full(nine_slice, angle_size)
    }

    pub fn draw<Global>(&self, draw_state : UiInterractiveState, px : Rect2, c : &mut DefaultContext<Global>)
    {
        let angle_size = self.angle_size.to_px(c.cam()).min((px.size / 2.).min_element().splat2());
        let mid_size = px.size - angle_size * 2.;

        self.bot_left.draw(draw_state, Rect2::new(px.pos, angle_size), c);
        self.bot_mid.draw(draw_state, Rect2::new(px.pos + angle_size.x_0(), vec2(mid_size.x, angle_size.y)), c);
        self.bot_right.draw(draw_state, Rect2::new(px.pos + angle_size.x_0() + mid_size.x_0(), angle_size), c);

        self.left.draw(draw_state, Rect2::new(px.pos + angle_size._0_y(), vec2(angle_size.x, mid_size.y)), c);
        self.mid.draw(draw_state, Rect2::new(px.pos + angle_size, mid_size), c);
        self.right.draw(draw_state, Rect2::new(px.pos + angle_size + mid_size.x_0(), vec2(angle_size.x, mid_size.y)), c);

        let h = angle_size._0_y() + mid_size._0_y();
        self.top_left.draw(draw_state, Rect2::new(px.pos+h, angle_size), c);
        self.top_mid.draw(draw_state, Rect2::new(px.pos + angle_size.x_0()+h, vec2(mid_size.x, angle_size.y)), c);
        self.top_right.draw(draw_state, Rect2::new(px.pos + angle_size.x_0() + mid_size.x_0() +h, angle_size), c);
    }

    pub fn map_sprite<F>(&mut self, f: F)
    where
        F: Fn(&mut Sprite) + Copy,
    {
        self.nine_slice.map(|e| 
            {
                e.map(f)
            });
    }
}