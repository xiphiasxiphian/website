use crate::{
    input::InputState, jade::{
        camera::Camera,
        ecs::{component::ComponentContext, object::Object},
    }, renderer::Renderable, util::assets::assetpool::AssetPool,
};

pub struct Scene
{
    objects: Vec<Object>,
    pub camera: Camera,
}

impl Scene
{
    pub fn new(viewport_dims: (f32, f32)) -> Self
    {
        Self {
            objects: vec![],
            camera: Camera::new(viewport_dims),
        }
    }

    pub fn with_object(mut self, object: Object) -> Self
    {
        self.objects.push(object);
        self
    }

    pub fn with_objects<I>(mut self, objects: I) -> Self
    where
        I: IntoIterator<Item = Object>,
    {
        self.objects.extend(objects);
        self
    }

    pub fn add(&mut self, object: Object)
    {
        let z = object.z_index();
        let pos = self.objects.partition_point(|x| x.z_index() <= z);

        self.objects.insert(pos, object);
    }

    pub fn start(&mut self, ctx: &mut ComponentContextIn)
    {
        for object in &mut self.objects
        {
            object.start(&mut ctx.resolve(&mut self.camera));
        }
    }

    pub fn tick(&mut self, ctx: &mut ComponentContextIn, dt: f64)
    {
        for object in &mut self.objects
        {
            object.tick(&mut ctx.resolve(&mut self.camera), dt);
        }
    }

    pub fn objects(&self) -> &[Object] { &self.objects }
}

pub struct ComponentContextIn<'a>
{
    pub input: &'a InputState,
    pub assetpool: &'a AssetPool,
}

impl<'a> ComponentContextIn<'a>
{
    pub fn resolve(&self, camera: &'a mut Camera) -> ComponentContext<'a>
    {
        ComponentContext {
            input: &self.input,
            assetpool: &self.assetpool,
            camera: camera,
        }
    }
}
