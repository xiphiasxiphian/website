use std::mem;

use crate::{jade::ecs::{component::{Component, ComponentContext}, transform::Transform}, util::assets::assetpool::TextureAsset};

pub struct Object
{
    pub name: String,
    transform: Transform,
    z_index: i32,
    components: Vec<Box<dyn Component>>,
    started: bool,
    texture: Option<TextureAsset>
}

impl Object
{
    pub fn new(
        name: &str,
        transform: Transform,
    ) -> Self
    {
        Self {
            name: name.to_string(),
            transform,
            components: vec![],
            started: false,
            texture: None,
        }
    }

    pub fn with_texture(mut self, texture: TextureAsset) -> Self
    {
        self.texture = Some(texture);
        self
    }

    pub fn with_z_index(mut self, z_index: i32) -> Self
    {
        self.z_index = z_index;
        self
    }

    pub fn add_component<C: Component>(&mut self, component: C) -> &mut Self
    {
        self.components.push(Box::new(component));
        self
    }

    pub fn get_component<C: Component>(&self) -> Option<&C>
    {
        self.components.iter().find_map(|x| x.as_any().downcast_ref::<C>())
    }

    pub fn get_component_mut<C: Component>(&mut self) -> Option<&mut C>
    {
        self.components.iter_mut().find_map(|x| x.as_any_mut().downcast_mut::<C>())
    }

    pub fn has_component<C: Component>(&self) -> bool
    {
        self.get_component::<C>().is_some()
    }


    pub fn start(&mut self, ctx: &mut ComponentContext)
    {
        if self.started { return; }

        for component in &mut self.components
        {
            component.start(ctx);
        }

        self.started = true;
    }

    pub fn tick(&mut self, ctx: &mut ComponentContext, dt: f32)
    {
        let mut components = mem::take(&mut self.components);
        for component in &mut components
        {
            component.tick(ctx, dt);
        }

        self.components = components;
    }
}

impl Renderable for Object
{

}
