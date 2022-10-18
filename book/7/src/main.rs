mod mod1{
  pub fn print_test(){
    println!("Bin ein Module...");
    mod2::print_test2(); // fine
  }

  mod mod2{
    pub fn print_test2(){
      println!("Bin noch ein Module...");
    }
  }

  pub mod mod3{
    pub fn print_test3(){
      super::mod2::print_test2(); // fine
    }
  }

  pub struct A{
    pub x: i32, y: i32
  }
  impl A{
    pub fn new(x: i32, y:i32) -> A {A {x,y}}
  }
}


fn main() {
  // Creates root binary crate (build target)
  println!("Hello, world!");
  mod1::print_test();
  // mod1::mod2::print_test(); //wrong because mod1::mod2 is not public
  let mut a = mod1::A::new(1,1);
  a.x = 0;
  // a.y = 0; //y not public
}
