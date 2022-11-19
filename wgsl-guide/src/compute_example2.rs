use std::f64::consts::PI;

use wgpu::{self, Device, Queue, ComputePipelineDescriptor, ShaderModuleDescriptor, ShaderSource,
  CommandEncoderDescriptor, ComputePassDescriptor, BindGroupDescriptor, BindGroupEntry, BufferDescriptor,
  BufferUsages
};

pub fn run_shader(device: &Device, queue: &Queue) {
  let size = (512*3, 512*2);
  let size_type = std::mem::size_of::<u32>();
  let buffer_size = (size.0 * size.1 * size_type) as wgpu::BufferAddress;

  //Create shader
  let shader = device.create_shader_module(
    ShaderModuleDescriptor {
      label: Some("shader"),
      source: ShaderSource::Wgsl(include_str!("mandelbrot.wgsl").into())
  });

  //Create Pipeline
  let pipeline = device.create_compute_pipeline(
    &ComputePipelineDescriptor {
      label: Some("pipeline"),
      layout: None,
      module: &shader,
      entry_point: "main_comp"
  });
  let bindgroup0_layout = pipeline.get_bind_group_layout(0);

  //Create and bind Buffers
  let compute_result = device.create_buffer(
    &BufferDescriptor {
      label: Some("compute_result"),
      size: buffer_size,
      usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
      mapped_at_creation: false,
  });
  let compute_result_host = device.create_buffer(
    &BufferDescriptor {
      label: Some("compute_result_host"),
      size: buffer_size,
      usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
      mapped_at_creation: false,
  });
  let bindgroup0_elem0 = BindGroupEntry {
    binding: 0,
    resource: compute_result.as_entire_binding(),
  };
  let bindgroup0 = device.create_bind_group(
    &BindGroupDescriptor {
      label: Some("bindgroup[0]"),
      layout: &bindgroup0_layout,
      entries: &[bindgroup0_elem0],
  });

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
  compute_pass.set_bind_group(0, &bindgroup0, &[]);
  compute_pass.dispatch_workgroups(size.0 as _,size.1 as _,1);
  drop(compute_pass);

  encoder.copy_buffer_to_buffer(
    &compute_result,
    0,
    &compute_result_host,
    0,
    buffer_size);
  
  //Run shader
  queue.submit(std::iter::once(encoder.finish()));

  let res = compute_result_host
    .slice(..);
  res.map_async(wgpu::MapMode::Read, |q| {q.unwrap()});

  device.poll(wgpu::Maintain::Wait);
  let res: Vec<_> = res
    .get_mapped_range()
    .chunks_exact(size_type)
    .map(|bytes| { match bytes {
      [b0, b1, b2, b3] => (*b0 as u32)+(*b1 as u32)*256+(*b2 as u32)*256*256+(*b3 as u32)*256*256*256,
      _ => panic!()
    }})
    .collect();
  eprintln!("Max Val: {}", res.iter().max().unwrap());

  let mut imgbuf = image::ImageBuffer::new(size.0 as _, size.1 as _);

  let mut res_iter = res.iter();
  let i_max = 4.*256.;
  let n_curl = 8.;
  for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
    let v = *res_iter.next().unwrap() as f64;
    let x = v/(i_max-1.);
    let (s_angle, c_angle) = (2.*PI*n_curl*x).sin_cos();
    let intens = 1.-x*x;
    *pixel = image::Rgb([
      ((c_angle+1.)*127.5*intens).round() as u8,
      ((s_angle+1.)*127.5*intens).round() as u8,
      (x*255.*intens).round() as u8
    ]);
  }

  imgbuf.save("mandelbrot.png").unwrap();
}

