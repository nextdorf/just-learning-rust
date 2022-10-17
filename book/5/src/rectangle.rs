#[derive(Debug)]
struct Rect{width: u32, height: u32}

pub fn main_rect(){
  let s = 2;
  let r = Rect{
    width: dbg!(1980*s),
    height: 1080
  };
  let s = Rect{
    width: 320,
    height: 240
  };
  dbg!(&r);
  println!("r {:?}: {}", r, r.area());
  println!("s {:?}: {}", s, s.area());
  println!("s fits in r: {}", r.can_hold(&s));
  dbg!(Rect::square(10));
}

impl Rect {
  fn area(&self) -> u32{
    self.width * self.height
  }

  fn can_hold(&self, r: &Rect) -> bool{
    self.width>r.width && self.height>r.height
  }

  fn square(width: u32) -> Rect{
    Rect { width, height: width }
  }
}
