use log::info;
use log::warn;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use wgpu::Adapter;
use wgpu::Device;
use wgpu::MemoryHints;
use wgpu::PowerPreference;
use wgpu::Queue;
use wgpu::Surface;
use wgpu::{
    CommandEncoderDescriptor, DeviceDescriptor, Instance, Limits, LoadOp, Operations, PresentMode,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, SurfaceConfiguration,
    SurfaceTarget, TextureUsages,
};

use crate::renderer::Renderable;
use crate::renderer::Renderer;
use crate::renderer::texture::Sprite;
use crate::renderer::texture::Texture;

pub struct Canvas<'a>
{
    dims: (u32, u32),
    instance: Instance,
    surface: Surface<'a>,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    renderer: Renderer,
    scene: Vec<Box<dyn Renderable>> // tmp will have an actual Scene type later
}

impl<'a> Canvas<'a>
{
    pub async fn attach(canvas_id: &str) -> Self
    {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();

        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().expect("Element is not a canvas");

        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

        let instance = Instance::default();
        let surface = instance.create_surface(SurfaceTarget::Canvas(canvas)).unwrap();

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate graphics adapter");

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: Limits::downlevel_webgl2_defaults(),
                    memory_hints: MemoryHints::default(),
                },
                None,
            )
            .await
            .expect("Failed to create wgpu device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = *surface_caps
            .formats
            .get(0)
            .expect("Surface incompatible with the adapter");

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: canvas_width,
            height: canvas_height,
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        info!("Succesfully attached wgpu to canvas");

        let renderer = Renderer::new(&device, surface_format);
        info!("Succesfully init renderer");

        Self {
            dims: (canvas_width, canvas_height),
            instance,
            adapter,
            surface,
            device,
            queue,
            renderer,
            scene: vec![],
        }
    }

    pub async fn run(&mut self)
    {
        let texture = Texture::from_bytes(
            include_bytes!("../assets/images/loki_purple.png"),
            &self.device,
            &self.queue,
            &self.renderer.texture_bind_group_layout,
        ).unwrap();

        let sprite = Box::new(Sprite::new(texture, (-0.5, 0.5), (1.0, 1.0)));
        self.scene.push(sprite);

        // main render loop
        loop
        {
            Self::next_animation_frame().await;

            let output = match self.surface.get_current_texture()
            {
                Ok(texture) => texture,
                Err(e) =>
                {
                    warn!("Surface texture dropped: {:?}", e);
                    continue;
                }
            };

            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            self.renderer.draw(&self.scene, &self.device, &self.queue, &view);

            output.present();
        }
    }

    async fn next_animation_frame()
    {
        let promise = js_sys::Promise::new(&mut |resolve, _reject| {
            let window = web_sys::window().unwrap();
            window.request_animation_frame(&resolve).unwrap();
        });
        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
    }
}
