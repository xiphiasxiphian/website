use log::info;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement};
use wgpu::Adapter;
use wgpu::Device;
use wgpu::MemoryHints;
use wgpu::PowerPreference;
use wgpu::Queue;
use wgpu::Surface;
use wgpu::{
    Instance,
    SurfaceTarget,
    RequestAdapterOptions,
    DeviceDescriptor,
    Limits,
    SurfaceConfiguration,
    TextureUsages,
    PresentMode,
    CommandEncoderDescriptor,
    RenderPassDescriptor,
    RenderPassColorAttachment,
    Operations,
    LoadOp,
    StoreOp,
};

use crate::util::color::Color;

pub struct Canvas<'a>
{
    dims: (u32, u32),
    instance: Instance,
    surface: Surface<'a>,
    adapter: Adapter,
    device: Device,
    queue: Queue
}

impl<'a> Canvas<'a>
{
    pub async fn attach(canvas_id: &str) -> Self
    {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();

        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .expect("Element is not a canvas");

        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

        let instance = Instance::default();
        let surface = instance.create_surface(SurfaceTarget::Canvas(canvas)).unwrap();

        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.expect("Failed to find an appropriate graphics adapter");

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: Limits::downlevel_webgl2_defaults(),
                memory_hints: MemoryHints::default(),
            },
            None
        ).await.expect("Failed to create wgpu device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = *surface_caps.formats.get(0).expect("Surface incompatible with the adapter");

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

        Self {
            dims: (canvas_width, canvas_height),
            instance,
            adapter,
            surface,
            device,
            queue,
        }
    }

    pub async fn run(&mut self)
    {
        // main render loop
        loop
        {
            Self::next_animation_frame().await;

            let output = match self.surface.get_current_texture() {
                Ok(texture) => texture,
                Err(e) => {
                    log::warn!("Surface texture dropped: {:?}", e);
                    continue;
                }
            };

            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            let color: wgpu::Color = Color::from_hex("c4aff5", u8::MAX).unwrap().into();

            {
                let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(color),
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
            } // render pass dropped here, releasing the encoder borrow

            self.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        }
    }

    async fn next_animation_frame() {
        let promise = js_sys::Promise::new(&mut |resolve, _reject| {
            let window = web_sys::window().unwrap();
            window.request_animation_frame(&resolve).unwrap();
        });
        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
    }
}
