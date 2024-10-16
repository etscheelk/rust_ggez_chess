pub mod cannon;
pub mod missile;
pub mod player;

/// test test

/// Update is a trait describing an object which should be updated every single frame
pub(crate) trait Update<I>
{
    fn update(&mut self, context: &mut ggez::Context) -> ggez::GameResult;
}

/// FixedUpdate is a trait describing an object which should be updated 
/// every fixed interval, such as 60 times a second for 60fps.
/// 
/// Use for things like physics especially.
pub(crate) trait FixedUpdate<I>
{
    fn fixed_update(&mut self, context: &mut ggez::Context) -> ggez::GameResult;
}

/// Draw is a trait describing an object which will be drawn to the 
/// supplied canvas in some way.
pub(crate) trait Draw<I>
{
    fn draw(&self, context: &mut ggez::Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult;
}