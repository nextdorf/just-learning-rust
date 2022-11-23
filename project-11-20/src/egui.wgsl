// epaint::mesh::Vertex
// pub struct Vertex {
//   /// Logical pixel coordinates (points).
//   /// (0,0) is the top left corner of the screen.
//   pub pos: Pos2, // 64 bit
//
//   /// Normalized texture coordinates.
//   /// (0, 0) is the top left corner of the texture.
//   /// (1, 1) is the bottom right corner of the texture.
//   pub uv: Pos2, // 64 bit
//
//   /// sRGBA with premultiplied alpha
//   pub color: Color32, // 32 bit
// }
struct Vertex {                   // 32 bytes
  @location(0) pos: vec2<f32>,    // 8 bytes
  @location(1) uv: vec2<f32>,     // 8 bytes
  @location(2) color: vec4<f32>,  // 16 bytes, rgba8, see also https://gpuweb.github.io/gpuweb/wgsl/#channel-formats
  //No padding
}

// emath::rect::Rect
// pub struct Rect {
//   /// One of the corners of the rectangle, usually the left top one.
//   pub min: Pos2,
// 
//   /// The other corner, opposing [`Self::min`]. Usually the right bottom one.
//   pub max: Pos2,
// }
struct Rect {                   // 16 bytes
  @location(0) min: vec2<f32>,  // 8 bytes
  @location(1) max: vec2<f32>,  // 8 bytes
  //No padding
}

// #[repr(C)]
// #[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct WindowSize {
//   pub width: u32,
//   pub height: u32,
//   pub scale: f32,
// }
struct WindowSize {             // 12 + 4 = 16 bytes
  @location(0) size: vec2<u32>, // 8 bytes
  @location(1) scale: f32,      // 4 bytes
  // implicit padding of 4 bytes
}


// struct VertexInput {
//   @location(0) pos: vec3<f32>,
//   @location(1) uv: vec2<f32>,
// };

struct VertexOutput {
  @builtin(position) pos: vec4<f32>,
  @location(0) color: vec4<f32>,
  @location(1) uv: vec2<f32>,
};


// Vertex shader

@group(1) @binding(0)
var<uniform> window_size: WindowSize;

@vertex
fn main_vs(vert: Vertex) -> VertexOutput {
  var res: VertexOutput;
  // res.pos = vec4((vert.pos - 0.5)*2./128., 0., 1.);
  res.pos = vec4(
    vert.pos.x/f32(window_size.size.x)*2. - 1.,
    1. - vert.pos.y/f32(window_size.size.y)*2.,
    0.,
    1./window_size.scale );
  res.color = vert.color;
  res.uv = vert.uv;
  return res;
}

// Fragment shader

@group(0) @binding(0)
var egui_texture: texture_2d<f32>;
@group(0) @binding(1)
var egui_sampler: sampler;

@fragment
fn main_fs(vert: VertexOutput) -> @location(0) vec4<f32> {
  return vert.color*textureSample(egui_texture, egui_sampler, vert.uv);
}

