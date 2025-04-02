use crate::*;

// Screen size, pen, render, asset, ... will be stored here

pub struct DefaultContext<Glob> 
{
    pub extern_time : ContextTime,

    pub globals : Glob,
    pub asset_manager : ContextAssetManager,

    pub pen   : ContextPen,
    pub audio : ContextAudio,

    pub input : ContextInput,
    pub perf  : ContextPerformance,


    pub debug_rng : Random,

    /*
    pub input : ContextInput,

    pub perf : ContextPerformance,*/
}

impl<Glob> DefaultContext<Glob>
{
    //pub fn ui_time(&self) -> Time { self.extern_time.total }

    pub fn pen(&self) -> &ContextPen { &self.pen }
    pub fn pen_mut(&mut self) -> &mut ContextPen { &mut self.pen }

    pub fn cam(&self) -> &ContextCamera { &self.pen.cam }
    pub fn cam_mut(&mut self) -> &mut ContextCamera { &mut self.pen.cam }

    pub fn audio(&self) -> &ContextAudio { &self.audio }
    pub fn audio_mut(&mut self) -> &mut ContextAudio { &mut self.audio }

    pub fn new(glob : Glob, asset_manager : ContextAssetManager) -> Self
    { 
        Self 
        { 
            // time: Default::default(),
            globals: glob,
            asset_manager,

            pen  : ___(),
            input: ___(),
            perf : ___(),
            extern_time : ___(),

            debug_rng : ___(),
            audio: ___(),
        }
    }

    pub fn unpack_pen_glob_mut(&mut self) -> (&mut ContextPen, &mut Glob)
    {
        (&mut self.pen, &mut self.globals)
    }
}

impl<Glob> ContextEvent<Glob> for DefaultContext<Glob>
{

    fn tick_begin  (ctx : &mut DefaultContext<Glob>) {
        ContextPerformance::tick_begin(ctx);
        ContextTime       ::tick_begin(ctx);
        ContextInput      ::tick_begin(ctx);
    }

    fn tick_end    (ctx : &mut DefaultContext<Glob>) {
        ContextPerformance::tick_end(ctx);
        ContextTime       ::tick_end(ctx);
        ContextInput      ::tick_end(ctx);

    }

    fn input_begin(ctx: & mut DefaultContext<Glob>) 
    {
        ContextPerformance::input_begin(ctx);

        ContextPen  ::input_begin(ctx);
        ContextInput::input_begin(ctx);
    }
    fn input_end  (ctx: & mut DefaultContext<Glob>) 
    {
        ContextPerformance::input_end(ctx);

        ContextInput::input_end(ctx);
        ContextPen  ::input_end(ctx);
    }

    fn update_begin(ctx: & mut DefaultContext<Glob>) 
    {
        ContextPerformance::update_begin(ctx);
    }
    fn update_end  (ctx: & mut DefaultContext<Glob>) 
    {
        ContextPerformance::update_end(ctx);
    }

    fn draw_begin(ctx: & mut DefaultContext<Glob>) 
    { 
        ContextPerformance::draw_begin(ctx);

        ContextInput ::draw_begin(ctx);
        ContextPen   ::draw_begin(ctx);
    }
    fn draw_end  (ctx: & mut DefaultContext<Glob>) 
    { 
        ContextPerformance::draw_end(ctx);

        ContextInput ::draw_end(ctx);
        ContextPen   ::draw_end(ctx);
    }
}