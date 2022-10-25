use std::{slice, ptr::NonNull};

pub mod ffi;

pub struct Frame<'a>{
  width: i32,
  height: i32,
  linesize: [i32; 8],
  data: [&'a mut [u8]; 8]
}

impl Frame<'_>{
  pub fn from(path: &str, skip_frames: i32) -> Option<Frame> {
    dbg!(skip_frames);
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut linesize: [i32; 8] = [0; 8];
    let data: [&mut [u8]; 8];
    unsafe{
      let mut data_ptrs: [*mut u8; 8] = [0 as *mut u8; 8];
      let width_ref = &mut width;
      let height_ref = &mut height;
      let err = ffi::renderfrom(
        path.as_ptr() as *const ::std::os::raw::c_char,
        data_ptrs.as_mut_ptr(),
        width_ref as *mut i32,
        height_ref as *mut i32,
        linesize.as_mut_ptr(),
        skip_frames
      );
      if err != 0 {
        return None;
      }

      let data_ptrs: Vec<_> = (0..8).map(|i| {
        let len = (linesize[i] * height) as usize;
        let ptr = if len>0 {data_ptrs[i]} else {NonNull::<u8>::dangling().as_ptr()};
        slice::from_raw_parts_mut(ptr, len)
      }).collect();
      data = data_ptrs.try_into().unwrap();
      // data = data_ptrs.map(|ptr| slice::from_raw_parts_mut(ptr,));
    }

    dbg!(format!("{:?}", {let q: [_; 16] = data[0][..16].try_into().unwrap(); q}));
    Some(Frame {width, height, linesize, data})
  }

  pub fn width(&self) -> i32 { self.width }
  pub fn height(&self) -> i32 { self.height }
  pub fn linesize(&self) -> [i32; 8] { self.linesize }
  pub fn channel(&self, idx: usize) -> &[u8] { self.data[idx] }
  pub fn channel_mut(&mut self, idx: usize) -> &mut [u8] { self.data[idx] }

}


impl Drop for Frame<'_>{
  fn drop(&mut self) {
    println!("Drops Frame");
    for x in self.data.iter_mut() {
      if x.len() == 0 {continue;}
      unsafe{
        ffi::freeData((*x).as_mut_ptr());
        *x = slice::from_raw_parts_mut(NonNull::<u8>::dangling().as_ptr(), 0);
      }
    }
  }
}

