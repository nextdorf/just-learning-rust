use std::fmt::Display;

fn main() {
  let number_list = vec![34., 50., 25., 100.1];
  let s1 = STest {x: 1, y: 2};
  let e1 = ETest::E1("Enum with generic");

  let s1y = s1.y;

  let largest = find_largest(&number_list);
  println!("Largest number of {:?} is {}", number_list, largest);

  let s12;
  {
    let (s1, s2) = ("abcd".to_string(), "xyz");
    print!("Longest between \"{}\" and \"{}\": ", &s1, s2);
    s12 = find_longest(&s1, s2);
    println!("{}", s12); //fine
  }
  // println!("{}", s12); //wrong unless s1 is &str
}

struct STest<T> { x: T, y: T }
enum ETest<T> { E1(T), E2 }

// fn find_largest<T: PartialOrd + Clone>(numbers: &[T]) -> T{
fn find_largest<T>(numbers: &[T]) -> T
where T: PartialOrd + Clone {
  let mut largest = &numbers[0];
  for n in numbers{
    if n > largest{
      largest = n;
    }
  }
  largest.clone()
}

impl<T> STest<T>{
  fn x(&self) -> &T { &self.x }
}

impl STest<i32>{
  fn y(&self) -> i32 { self.y }
}

fn find_longest<'a>(x: &'a str, y: &'a str) -> &'a str{
  if x.len() > y.len(){
    x
  } else{
    y
  }
}

fn longest_with_an_announcement<'a, T>(
  x: &'a str,
  y: &'a str,
  ann: T,
) -> &'a str
where T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
      x
  } else {
      y
  }
}
