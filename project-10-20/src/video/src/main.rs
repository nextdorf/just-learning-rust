use std::slice;

//run with LD_LIBRARY_PATH=$PWD/libvideoc/install/libs cargo run 
fn main(){
  let data;
  unsafe{
    let len = 1024;//*1024*4;
    data = slice::from_raw_parts_mut(video::genSomeData(len), len);
  }

  for i in 0..data.len(){
    data[i] = (((i*i) as u16) % 256) as u8
  }
  println!("{:?}", data);

  unsafe{
    video::freeData(data.as_mut_ptr());
  }
}


