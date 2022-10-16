use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
  println!("Guess the number!");
  println!("> ");

  let secret_number = rand::thread_rng().gen_range(1..10);

  loop {
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read input");

    let guess: i32 = match guess.trim().parse() {
      Ok(x) => x,
      Err(_) => continue
    };

    match guess.cmp(&secret_number){
      Ordering::Less => println!("Too low"),
      Ordering::Greater => println!("Too high"),
      Ordering::Equal => {
        println!("Correct!!!");
        break;
      },
    };
  }
}

