use crate::{
    input::key::Key, jade::ecs::{component::{Component, ComponentContext}, object::Object},
};
use std::any::Any;

pub struct PlayerController {
    pub speed: f64,
}

impl Component for PlayerController {
    fn tick(&mut self, parent: &mut Object, ctx: &mut ComponentContext, dt: f64)
    {
        let input = ctx.input;

        if input.is_key_held(Key::A) { parent.transform.pos.0 -= self.speed * dt; }
        if input.is_key_held(Key::D) { parent.transform.pos.0 += self.speed * dt; }
        if input.is_key_held(Key::W) { parent.transform.pos.1 -= self.speed * dt; }
        if input.is_key_held(Key::S) { parent.transform.pos.1 += self.speed * dt; }
    }

    fn as_any(&self)         -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
