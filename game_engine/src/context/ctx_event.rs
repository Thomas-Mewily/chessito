use crate::*;

pub(crate) trait ContextEvent<Glob>
{
    #[allow(unused_variables)]
    fn init_begin  (ctx : &mut DefaultContext<Glob>) {}
    #[allow(unused_variables)]
    fn init_end    (ctx : &mut DefaultContext<Glob>) {}

    #[allow(unused_variables)]
    fn tick_begin  (ctx : &mut DefaultContext<Glob>) {}
    #[allow(unused_variables)]
    fn tick_end    (ctx : &mut DefaultContext<Glob>) {}

    #[allow(unused_variables)]
    fn input_begin (ctx : &mut DefaultContext<Glob>) {}
    #[allow(unused_variables)]
    fn input_end   (ctx : &mut DefaultContext<Glob>) {}

    #[allow(unused_variables)]
    fn update_begin(ctx : &mut DefaultContext<Glob>) {}
    #[allow(unused_variables)]
    fn update_end  (ctx : &mut DefaultContext<Glob>) {}

    #[allow(unused_variables)]
    fn draw_begin  (ctx : &mut DefaultContext<Glob>) {}
    #[allow(unused_variables)]
    fn draw_end    (ctx : &mut DefaultContext<Glob>) {}
}