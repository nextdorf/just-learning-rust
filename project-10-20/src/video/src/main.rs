use std::{slice, fs::File, io::Write};

fn _test1(){
  // let data;
  // unsafe{
  //   let len = 1024;//*1024*4;
  //   data = slice::from_raw_parts_mut(video::ffi::genSomeData(len), len);
  // }

  // for i in 0..data.len(){
  //   data[i] = (((i*i) as u16) % 256) as u8
  // }
  // println!("{:?}", data);

  // unsafe{
  //   video::ffi::freeData(data.as_mut_ptr());
  // }
}

fn extract_frame(video_path: &str, out_path: &str, skip_frames: i32) -> std::io::Result<()>
{
  let frm = video::Frame::from(video_path, skip_frames).expect("Video couldn't be opened");

  let mut f = match File::create(out_path) {
    Ok(g) => g,
    Err(e) => return Err(e)
  };
  for i in 0..8 {
    match f.write_all(frm.channel(i)) {
      Ok(()) => (),
      Err(e) => return Err(e)
    }
  }
  f.sync_data()
}


fn main(){
  let args: Vec<String> = std::env::args().collect();
  // dbg!(args);
  if args.len() < 3 || args.len() > 4{
    println!("Usage: {} video_path out_path [skip_frames=60*10]", args[0])
  } else {
    let skip_frames = args.get(3).and_then(|s| s.parse::<i32>().ok()).unwrap_or(60*10);
    extract_frame(args[1].as_str(), args[2].as_str(), skip_frames)
      .expect("")
  }
  // extract_frame("", "", 60*24);
}


