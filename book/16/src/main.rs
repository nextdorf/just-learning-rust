use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

fn main() {
  let (tx, rx) = mpsc::channel();
  let _handle = thread::spawn(move || {
    thread::sleep(Duration::from_millis(100));
    tx.send("world!").unwrap();
  });
  println!("Hello, ");
  // handle.join().unwrap();
  println!("{}", rx.recv().unwrap());


  let counter = Arc::new(Mutex::new(5));
  print!("Attempt to increase counter 100 times\n  before: {}", *counter.lock().unwrap());
  let mut threads = vec![];
  for _ in 0..100 {
    let counter_ref = counter.clone();
    threads.push(thread::spawn(move || {
      match counter_ref.lock() {
        Ok(mut m) => *m += 1,
        Err(_) => println!("Couldn't lock mutex!!!")
      }
    }));
  }
  for t in threads {
    t.join().unwrap();
  }
  println!(", after: {}", *counter.lock().unwrap());

}
