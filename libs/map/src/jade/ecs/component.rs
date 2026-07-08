use std::any::Any;

use crate::{input::InputState, jade::ecs::object::Object};

pub trait Component: Any
{
    fn start(&mut self, _parent: &mut Object, _ctx: &mut ComponentContext) {}

    fn tick(&mut self, _parent: &mut Object, _ctx: &mut ComponentContext, _dt: f64) {}

    // downcasting
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ComponentContext<'a>
{
    pub input: &'a InputState,
}
