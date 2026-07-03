use std::any::Any;

use crate::input::InputState;

pub trait Component: Any
{
    fn start(&mut self, _ctx: &mut ComponentContext) {}

    fn tick(&mut self, _ctx: &mut ComponentContext, _dt: f32) {}

    // downcasting
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ComponentContext<'a>
{
    pub input: &'a InputState,
}
