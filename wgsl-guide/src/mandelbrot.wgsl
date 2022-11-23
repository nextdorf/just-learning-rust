@group(0) @binding(0)
var<storage, read_write> arr: array<u32>;


fn cfn(x: vec2<f32>, size: vec2<f32>, i: vec2<u32>, iLen: vec2<u32>) -> vec2<f32> {
  return vec2<f32>(i)/vec2<f32>(iLen - 1u)*size + x - size/2.;
}

fn cFull(i: vec2<u32>, iLen: vec2<u32>) -> vec2<f32> {
  return cfn(vec2<f32>(-0.5, 0.), vec2<f32>(3., 2.), i, iLen);
}

@compute @workgroup_size(1)
fn main_comp(
  @builtin(num_workgroups) ngroups: vec3<u32>,
  @builtin(global_invocation_id) idx: vec3<u32>)
{
  let idx1 = idx.x + ngroups.x*(idx.y + ngroups.y*idx.z);
  // let c = cFull(idx.x, idx.y, ngroups.x, ngroups.y);
  let c = cfn(
    vec2<f32>(-0.810546875, -0.1796875), vec2<f32>(1.5,1.) * 0.01, idx.xy, ngroups.xy);
  var z = vec2<f32>();

  let iMax = 4u*256u;
  var i = 0u;

  loop {
    z = vec2(z.x*z.x - z.y*z.y, 2.*z.x*z.y) + c;
    let zAbs2 = z.x*z.x + z.y*z.y;
    i++;
    if(i >= iMax || zAbs2 > 4.) {
      break;
    }
  }

  arr[idx1] = i;
}


