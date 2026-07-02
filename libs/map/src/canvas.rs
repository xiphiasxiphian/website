use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use log::info;
use log::warn;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::Window;
use wgpu::Adapter;
use wgpu::Device;
use wgpu::MemoryHints;
use wgpu::PowerPreference;
use wgpu::Queue;
use wgpu::Surface;
use wgpu::{
    DeviceDescriptor, Instance, Limits, PresentMode, RequestAdapterOptions, SurfaceConfiguration, SurfaceTarget,
    TextureUsages,
};

use crate::clock::Clock;
use crate::input::InputState;
use crate::renderer::Renderable;
use crate::renderer::Renderer;
use crate::renderer::texture::Sprite;
use crate::util::assets;
use crate::util::assets::assetpool::AssetPool;

pub struct Canvas<'a>
{
    window: Window,

    // internal gpu info
    dims: (u32, u32),
    instance: Instance,
    surface: Surface<'a>,
    adapter: Adapter,
    device: Device,
    queue: Queue,

    // user level
    renderer: Renderer,
    scene: Vec<Box<dyn Renderable>>, // tmp will have an actual Scene type later
    input: Rc<RefCell<InputState>>,
    asset_pool: AssetPool,
}

impl<'a> Canvas<'a>
{
    pub async fn attach(canvas_id: &str) -> Self
    {
        let mut window = web_sys::window().expect("Window not found");
        let document = window.document().expect("Document not found");

        let element = document.get_element_by_id(canvas_id).unwrap();

        let input = Rc::new(RefCell::new(InputState::new()));
        InputState::attach_listeners(input.clone(), &mut window, element.as_ref());
        info!("Successfully init input state and attached listeners");

        let canvas: HtmlCanvasElement = element
            .dyn_into::<HtmlCanvasElement>()
            .expect("Element is not a canvas");
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

        info!("Attempting to attach {}x{} canvas", canvas_width, canvas_height);

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

        let asset_pool = AssetPool::preloaded(
            assets::TEXTURES,
            &device,
            &queue,
            &renderer.texture_bind_group_layout
        )
        .expect("Failed to init asset pool");
        info!("Successfully init assetpool");

        Self {
            window,
            dims: (canvas_width, canvas_height),
            instance,
            adapter,
            surface,
            device,
            queue,
            renderer,
            scene: vec![],
            input,
            asset_pool,
        }
    }

    pub async fn run(&mut self)
    {
        let texture = self
            .asset_pool
            .get_texture("grass")
            .unwrap();

        let sprite = Box::new(Sprite::new(texture, (100.0, 100.0), (200.0, 200.0)));
        self.scene.push(sprite);

        let mut clock = Clock::new(&self.window).expect("Failed to init clock");

        // main render loop
        loop
        {
            Self::next_animation_frame().await;
            let _dt = clock.tick();

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
            self.renderer.draw(
                &self.scene,
                &self.device,
                &self.queue,
                &view,
                (self.dims.0 as f32, self.dims.1 as f32),
            );

            output.present();
            self.input.borrow_mut().flush();
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
