pub mod simple_future;
use std::time::Duration;

use simple_future::*;
use timer_future::*;

fn main() {
  let executor = {
    let (spawner, res) = Task::channel();
    spawner.spawn(async {
      println!("Hello");
      TimerFuture::new(Duration::from_secs(2)).await;
      println!("World");
    }).unwrap();
    res
  };
  executor.run();
}
