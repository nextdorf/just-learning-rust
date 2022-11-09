use std::{num::NonZeroU64, sync::Arc};

use eframe::{
  egui_wgpu::{self, wgpu},
  wgpu::util::DeviceExt,
  egui
};

fn main() {
  let native_options = eframe::NativeOptions {
    renderer: eframe::Renderer::Wgpu,
    ..eframe::NativeOptions::default()
  };
  eframe::run_native(
    "My egui App",
    native_options,
    Box::new(|cc| Box::new(MyEguiApp::new(cc).unwrap()))
  );
}

struct RenderResource {
  pub pipeline: wgpu::RenderPipeline,
  pub bind_group: wgpu::BindGroup,
  pub uniform_buffer: wgpu::Buffer,
}

impl RenderResource {
  fn new(device: &wgpu::Device, texture_format: &wgpu::TextureFormat) -> Self {

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("bind_group_layout"),
      entries: &[wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
          ty: wgpu::BufferBindingType::Uniform,
          has_dynamic_offset: false,
          min_binding_size: NonZeroU64::new(16),
        },
        count: None,
      }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("pipeline_layout"),
      bind_group_layouts: &[&bind_group_layout],
      push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("pipeline"),
      layout: Some(&pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[],
      },
      fragment: Some(wgpu::FragmentState {
          module: &shader,
          entry_point: "fs_main",
          targets: &[Some(texture_format.clone().into())],
      }),
      primitive: wgpu::PrimitiveState::default(),
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview: None,
    });

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("uniform_buffer"),
        contents: bytemuck::cast_slice(&[0.0_f32; 4]), // 16 bytes aligned!
        // Mapping at creation (as done by the create_buffer_init utility) doesn't require us to to add the MAP_WRITE usage
        // (this *happens* to workaround this bug )
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind_group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
    });

    // Because the graphics pipeline must have the same lifetime as the egui render pass,
    // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
    // `paint_callback_resources` type map, which is stored alongside the render pass.
    RenderResource {
      pipeline,
      bind_group,
      uniform_buffer,
    }

  }
}

#[derive(Default)]
struct MyEguiApp {
  pub angle: f32
}

impl MyEguiApp {
  fn new(cc: &eframe::CreationContext<'_>) -> Option<Self> {
    // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
    // Restore app state using cc.storage (requires the "persistence" feature).
    // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
    // for e.g. egui::PaintCallback.
    let renderstate = cc.wgpu_render_state.as_ref()?;

    renderstate
      .egui_rpass
      .write()
      .paint_callback_resources
      .insert(RenderResource::new(
        &renderstate.device,
        &renderstate.target_format,
      ));

    Some(Self { angle: 0.0 })
  }
}

impl eframe::App for MyEguiApp {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Hello World!");

      let (rect, response) =
        ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

      self.angle += response.drag_delta().x * 0.01;
      let angle = self.angle;

      let callback = egui_wgpu::CallbackFn::new()
        .prepare(move | _device, queue, type_map | {
          let resource: &RenderResource = type_map.get().unwrap();
          queue.write_buffer(
            &resource.uniform_buffer,
            0,
            bytemuck::cast_slice(&[angle,0.0,0.0,0.0])
          ); })
        .paint(move | _info, render_pass, type_map | {
          let resource: &RenderResource = type_map.get().unwrap();
          render_pass.set_pipeline(&resource.pipeline);
          render_pass.set_bind_group(0, &resource.bind_group, &[]);
          render_pass.draw(0..3, 0..1);
          //TODO: Define Triangle
        });

      let callback = egui::PaintCallback { rect, callback: Arc::new(callback) };
      ui.painter().add(callback);
    });
  }
}

