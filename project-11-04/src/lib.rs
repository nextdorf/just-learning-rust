pub mod texture;

use wgpu::util::DeviceExt;
use winit::{window::Window, event::WindowEvent, dpi::PhysicalSize};

pub struct WinState {
  pub surface: wgpu::Surface,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
  pub size: winit::dpi::PhysicalSize<u32>,

  pub render_pipeline: wgpu::RenderPipeline,
  pub vertex_buffer: wgpu::Buffer,
  // pub num_vertices: u32,
  pub index_buffer: wgpu::Buffer,
  pub num_indices: u32,
  pub diffuse_bind_group: wgpu::BindGroup,
  pub texture: texture::Texture,
}

const SRGB2RGB_A: f64 = 0.055;
const SRGB2RGB_GAMMA: f64 = 2.4;
pub fn rgb_to_srgb(rgb: u8) -> f64 { (((rgb as f64)/255. + SRGB2RGB_A)/(1.+SRGB2RGB_A)).powf(SRGB2RGB_GAMMA) }
pub fn rgb_to_srgb_f32(rgb: u8) -> f32 { rgb_to_srgb(rgb) as _ }
pub fn srgb_to_rgb(srgb: f64) -> u8 { ((srgb.powf(1./SRGB2RGB_GAMMA)*(1.+SRGB2RGB_A) - SRGB2RGB_A)*255.).round() as _ }
pub fn srgb_to_rgb_f32(srgb: f32) -> u8 { srgb_to_rgb(srgb as _) }
// pub fn srgb_to_rgb(srgb: f64) -> u8 { (((srgb + SRGB2RGB_A)/(1.+SRGB2RGB_A)).powf(SRGB2RGB_GAMMA) * 255.).round() as _ }
// pub fn rgb_to_srgb(rgb: u8) -> f64 { ((rgb as f64) /255.).powf(1./SRGB2RGB_GAMMA)*(1.+SRGB2RGB_A) - SRGB2RGB_A }



#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
  pos: [f32; 3],
  uv: [f32; 2],
}
const TRIVERTS: &[Vertex] = &[
  Vertex { pos: [0.,    0.5,  0.], uv: [0.5, 0.,] },
  Vertex { pos: [-0.5,  -0.5, 0.], uv: [0.,  1.] },
  Vertex { pos: [0.5,   -0.5, 0.], uv: [1.,  1.] },
];
const QUADVERTS: &[Vertex] = &[
  Vertex { pos: [-0.5,  0.5,  0.], uv: [0., 0.] },
  Vertex { pos: [-0.5,  -0.5, 0.], uv: [0., 1.] },
  Vertex { pos: [0.5,   -0.5, 0.], uv: [1., 1.] },
  Vertex { pos: [0.5,   0.5,  0.], uv: [1., 0.] },
];
const QUADINDS: &[u16] = &[
  0, 1, 2,
  2, 3, 0,
];

impl WinState {
  //surface, device, queue, config, size, render_pipeline, vertex_buffer
  async fn new_surface_device_queue(window: &Window) -> Option<(wgpu::Surface, wgpu::Adapter, wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window) };
    let adapter = instance.request_adapter(
      &wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface)
      }
    ).await?;

    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        features: wgpu::Features::empty(),
        limits: wgpu::Limits::default(),
        label: None,
      },
      None
    ).await.ok()?;
    Some((surface, adapter, device, queue))
  }
  
  fn new_config(size: &PhysicalSize<u32>, surface: &wgpu::Surface, adapter: &wgpu::Adapter, device: &wgpu::Device) -> wgpu::SurfaceConfiguration {
    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface.get_supported_formats(&adapter)[0],
      width: if size.width > 0 {size.width} else { 1 },
      height: if size.height > 0 {size.height} else { 1 },
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: wgpu::CompositeAlphaMode::Auto,
    };
    surface.configure(&device, &config);
    config
  }

  fn new_render_pipline(device: &wgpu::Device, shader: &wgpu::ShaderModule, config: &wgpu::SurfaceConfiguration, bind_group_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
    let render_pipeline_layout = device.create_pipeline_layout(
      &wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    });

    let vertex = wgpu::VertexState {
      module: &shader,
      entry_point: "vs_main", // 1.
      // buffers: &[], // 2.
      buffers: &[ Vertex::desc() ]
    };
    let fragment_targets = [Some(wgpu::ColorTargetState { // 4.
      format: config.format,
      blend: Some(wgpu::BlendState::REPLACE),
      write_mask: wgpu::ColorWrites::ALL,
    })];
    let fragment = Some(wgpu::FragmentState { // 3.
      module: &shader,
      entry_point: "fs_main",
      targets: &fragment_targets,
    });

    let primitive = wgpu::PrimitiveState {
      topology: wgpu::PrimitiveTopology::TriangleList, // 1.
      strip_index_format: None,
      front_face: wgpu::FrontFace::Ccw, // 2.
      cull_mode: Some(wgpu::Face::Back),
      // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
      polygon_mode: wgpu::PolygonMode::Fill,
      // Requires Features::DEPTH_CLIP_CONTROL
      unclipped_depth: false,
      // Requires Features::CONSERVATIVE_RASTERIZATION
      conservative: false,
    };

    let multisample = wgpu::MultisampleState {
      count: 1, // 2.
      mask: !0, // 3.
      alpha_to_coverage_enabled: false, // 4.
    };


    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex,
      fragment,
      primitive,
      depth_stencil: None, // 1.
      multisample,
      multiview: None, // 5.
    })
  }

  fn new_bind_group(diffuse_texture: &texture::Texture, device: &wgpu::Device, queue: &wgpu::Queue) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let texture_bind_group_layout = device.create_bind_group_layout(
      &wgpu::BindGroupLayoutDescriptor {
        entries: &[
          wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
              multisampled: false,
              view_dimension: wgpu::TextureViewDimension::D2,
              sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
          },
          wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            // This should match the filterable field of the
            // corresponding Texture entry above.
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
          },
        ],
        label: Some("texture_bind_group_layout"),
    });
    let texture_bind_group = device.create_bind_group(
      &wgpu::BindGroupDescriptor {
        label: Some("texture_bind_group"),
        layout: &texture_bind_group_layout,
        entries: &[
          wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
          },
          wgpu::BindGroupEntry {
            binding: 1,
            resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
          },
        ]
    });

    (texture_bind_group, texture_bind_group_layout)
  }

  pub async fn new(window: &Window) -> Self {
    let size = window.inner_size();

    let (surface, adapter, device, queue) = 
      WinState::new_surface_device_queue(window).await
      .expect("Could not set up the surface to device queue");

    let diffuse_bytes = include_bytes!("../logo.png");
    let diffuse_texture =
      texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "diffuse_texture")
      .expect("Couldnt load texture");

    let (diffuse_bind_group, diffuse_bind_group_layout) = 
      WinState::new_bind_group(&diffuse_texture, &device, &queue);

    let config = WinState::new_config(&size, &surface, &adapter, &device);
    eprintln!("--> {:?}", adapter.get_info());
    for fmt in surface.get_supported_formats(&adapter) {
      eprintln!("{:?}", fmt);
    }
    eprintln!("-----");

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });


    let render_pipeline = WinState::new_render_pipline(&device, &shader, &config, &diffuse_bind_group_layout);

    let vertex_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        // contents: bytemuck::cast_slice(TRIVERTS),
        contents: bytemuck::cast_slice(QUADVERTS),
        usage: wgpu::BufferUsages::VERTEX,
    });
    let index_buffer = device.create_buffer_init(
      &wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(QUADINDS),
        usage: wgpu::BufferUsages::INDEX,
    });

    Self {
      surface,
      device,
      queue,
      config,
      size,
      render_pipeline,
      vertex_buffer,
      // num_vertices: TRIVERTS.len() as _,
      index_buffer,
      num_indices: QUADINDS.len() as _,
      diffuse_bind_group,
      texture: diffuse_texture,
    }
  }
  
  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      self.surface.configure(&self.device, &self.config);
    }
  }
  
  pub fn input(&mut self, event: &WindowEvent) -> bool {
    false
  }
  
  pub fn update(&mut self) {
  }
  
  pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
    let output = self.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = self.device.create_command_encoder(
      &wgpu::CommandEncoderDescriptor {
        label: Some("Some encoder"),
      }
    );
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    // encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("Render Pass"),
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: &view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Clear(wgpu::Color {
            r: rgb_to_srgb(16), //16./255.,
            g: rgb_to_srgb(16), //16./255.,
            b: rgb_to_srgb(32), //32./255.,
            a: 1.0,
          }),
          store: true,
        },
      })],
      depth_stencil_attachment: None,
    });
    render_pass.set_pipeline(&self.render_pipeline);
    render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
    render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
    render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    drop(render_pass);


    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();
    Ok(())
  }
}


impl Vertex {
  fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Vertex>() as _,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &[
        wgpu::VertexAttribute {
          format: wgpu::VertexFormat::Float32x3,
          offset: 0,
          shader_location: 0,
        },
        wgpu::VertexAttribute {
          format: wgpu::VertexFormat::Float32x2,
          offset: std::mem::size_of::<[f32; 3]>() as _,
          shader_location: 1,
        },
    ]}

  }
}

