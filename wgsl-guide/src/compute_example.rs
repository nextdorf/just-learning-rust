use wgpu::{self, Device, Queue, ComputePipelineDescriptor, ShaderModuleDescriptor, ShaderSource, CommandEncoderDescriptor, RenderPassDescriptor, ComputePassDescriptor};

const shader_code: &str = "\
  @compute @workgroup_size(2,2)
  fn main_comp(
    @builtin(num_workgroups) ngroups: vec3<u32>,
    @builtin(global_invocation_id) idx: vec3<u32>) {
    //Do stuff
  }
  ";

pub fn run_shader(device: &Device, queue: &Queue) {
  //Create shader
  let shader_descriptor = ShaderModuleDescriptor {
    label: Some("shader"),
    source: ShaderSource::Wgsl(shader_code.into())
  };
  let shader = device.create_shader_module(shader_descriptor);

  //Create Pipeline
  let pipeline_descriptor = ComputePipelineDescriptor {
    label: Some("pipeline"),
    layout: None,
    module: &shader,
    entry_point: "main_comp"
  };
  let pipeline = device.create_compute_pipeline(&pipeline_descriptor);

  //Create and initialize Computepass
  let mut encoder = device.create_command_encoder(
    &CommandEncoderDescriptor {
      label: Some("encoder")
  });

  let mut compute_pass = encoder.begin_compute_pass(
    &ComputePassDescriptor {
      label: Some("compute_pass"),
  });
  compute_pass.set_pipeline(&pipeline);
  compute_pass.dispatch_workgroups(256,256,1);
  drop(compute_pass);

  //Run shader
  queue.submit(std::iter::once(encoder.finish()));
}

