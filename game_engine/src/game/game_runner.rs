use crate::*;

#[derive(Clone, Debug, PartialEq)]
pub struct GameRunner<G : IGame>
{
    pub input_provider : G::InputProvider,
    pub game           : G,
    pub game_time      : GameTime,
}

impl<G : IGame> GameRunner<G>
{
    pub async fn new_and_run(game : G, ctx : &mut DefaultContext<G::Global>) -> Self
    {
        let mut r = Self::new(game, ctx);
        r.run(ctx).await;
        r
    }

    pub fn new(game : G, ctx : &mut DefaultContext<G::Global>) -> Self
    {
        Self 
        {
            input_provider: G::InputProvider::new(ctx),
            game_time: ___(),
            game
        }
    }

    pub fn input(&mut self, ctx : &mut DefaultContext<G::Global>) -> G::Input 
    { 
        let input = self.input_provider.get_input(&mut self.game, self.game_time, ctx);
        if input.have_side_effect() { self.game_time.last_input = self.game_time.total; }
        input
    }

    pub async fn input_and_update(&mut self, delta : Time, ctx : &mut DefaultContext<G::Global>) 
    {
        let input = self.input(ctx);
        self.update(input, delta, ctx).await;
    }

    fn update_delta_time(&mut self, delta : Time)
    {
        self.game_time.total += delta;
        self.game_time.delta  = delta;
    }

    pub async fn update(&mut self, input : G::Input, delta : Time, ctx : &mut DefaultContext<G::Global>) 
    { 
        self.update_delta_time(delta);
        self.game.update(input, self.game_time, ctx).await;
    }

    pub fn draw(&self, ctx : &mut DefaultContext<G::Global>) 
    { 
        self.game.draw(self.game_time, ctx); 
    }

    pub async fn run(&mut self, ctx : &mut DefaultContext<G::Global>)
    {
        <DefaultContext<G::Global> as ContextEvent<G::Global>>::init_begin(ctx);
        self.game.init(ctx).await;
        <DefaultContext<G::Global> as ContextEvent<G::Global>>::init_end(ctx);

        loop
        {
            let delta_time = Time::from_s(macroquad::prelude::get_frame_time() as real);

            <DefaultContext<G::Global> as ContextEvent<G::Global>>::tick_begin(ctx);
            {
                <DefaultContext<G::Global> as ContextEvent<G::Global>>::input_begin(ctx);
                let input = self.input(ctx);
                <DefaultContext<G::Global> as ContextEvent<G::Global>>::input_end(ctx);
                
                <DefaultContext<G::Global> as ContextEvent<G::Global>>::update_begin(ctx);
                self.update(input, delta_time, ctx).await;
                <DefaultContext<G::Global> as ContextEvent<G::Global>>::update_end(ctx);
    
                <DefaultContext<G::Global> as ContextEvent<G::Global>>::draw_begin(ctx);
                self.draw(ctx);
                <DefaultContext<G::Global> as ContextEvent<G::Global>>::draw_end(ctx);
            }
            <DefaultContext<G::Global> as ContextEvent<G::Global>>::tick_end(ctx);

            self.next_frame().await;
        }
    }

    pub async fn next_frame(&mut self) { macroquad::prelude::next_frame().await }
}
