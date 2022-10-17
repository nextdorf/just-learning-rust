struct User {
  active: bool,
  user_name: String,
  email: String,
  signin_count: u64
}

struct Point(i32, i32, i32);

struct AlwaysEqual;

pub fn main_example() {
  let user1 = new_user(
    String::from("user@example.com"),
    String::from("Dr. No")
  );
  let user2 = User {user_name: String::from("Bond"), ..user1};
  println!("User1: {}\nUser2: {}", user1.user_name, user2.user_name);
  // println!("User1 mail: {}", user1.email); //error

  let p1 = Point(1,2,3);
  println!("|p1|^2: {}", norm2(p1));

  let like_unit = AlwaysEqual;
}

fn new_user(email: String, user_name: String) -> User{
  User {
    email,
    user_name,
    active: true,
    signin_count: 1,
  }
}

fn norm2(p:Point) -> i32 {
  p.0*p.0 + p.1*p.1 + p.2*p.2
}
