struct CameraUniform {
  proj_view: mat4x4<f32>,
};


struct VertexInput {
  @location(0) pos: vec3<f32>,
  @location(1) uv: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
};


// Vertex shader

@group(1) @binding(0)
var<uniform> camera: CameraUniform; 

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
  var out: VertexOutput;
  out.pos = camera.proj_view * vec4<f32>(model.pos, 1.0);
  out.uv = model.uv;
  return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(t_diffuse, s_diffuse, in.uv);
}

 

 