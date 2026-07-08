use crate::jade::ecs::{component::ComponentContext, object::Object};

#[derive(Default)]
pub struct Scene
{
    objects: Vec<Object>,
}

impl Scene
{
    pub fn with_object(mut self, object: Object) -> Self
    {
        self.objects.push(object);
        self
    }

    pub fn with_objects<I>(mut self, objects: I) -> Self
    where
        I: IntoIterator<Item = Object>
    {
        self.objects.extend(objects);
        self
    }

    pub fn add(&mut self, object: Object) { self.objects.push(object); }

    pub fn start(&mut self, ctx: &mut ComponentContext)
    {
        for object in &mut self.objects
        {
            object.start(ctx);
        }
    }

    pub fn tick(&mut self, ctx: &mut ComponentContext, dt: f64)
    {
        for object in &mut self.objects
        {
            object.tick(ctx, dt);
        }
    }

    pub fn objects(&self) -> &[Object] { &self.objects }
}
