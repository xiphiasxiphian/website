use std::any::Any;

use crate::jade::ecs::{component::{Component, ComponentContext}, components::default_any_impl, object::Object};

pub struct CameraLock;

impl Component for CameraLock
{
    fn tick(&mut self, parent: &mut Object, ctx: &mut ComponentContext, _dt: f64)
    {
        ctx.camera.position.x = parent.transform.pos.0 as f32;
        ctx.camera.position.y = parent.transform.pos.1 as f32;
    }

    default_any_impl!();
}
