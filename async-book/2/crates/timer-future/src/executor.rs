use std::{sync::{mpsc::{Receiver, Sender, channel, SendError, SyncSender, sync_channel}, Arc, Mutex}, task::Context};

use futures::{future::BoxFuture, Future, FutureExt, task::{ArcWake, waker_ref}};

use crate::TimerFuture;

/// Receiver of Tasks
pub struct Executor {
  r_task: Receiver<Arc<Task>>
}

/// Sender of Tasks
pub struct Spawner {
  t_task: SyncSender<Arc<Task>>
}

/// Handle for a Future
pub struct Task {
  future: Mutex<Option<BoxFuture<'static, ()>>>,
  sender: SyncSender<Arc<Self>>
}


impl ArcWake for Task {
  fn wake_by_ref(arc_self: &Arc<Self>) {
    let cloned = arc_self.clone();
    arc_self.sender.send(cloned).expect("Queue size exceeded")
  }
}


impl Task {
  pub fn channel() -> (Spawner, Executor) {
    let (t_task, r_task) = sync_channel(10_000);
    (Spawner { t_task }, Executor { r_task })
  }
}

impl Spawner {
  pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) -> Result<(), SendError<Arc<Task>>> {
    let boxed_future = future.boxed();
    let task = Arc::new(Task {
      future: Mutex::new(Some(boxed_future)),
      sender: self.t_task.clone()
    });
    self.t_task.send(task)
  }
}

impl Executor {
  pub fn run(&self) {
    while let Ok(task) = self.r_task.recv() {
      let mut future_guard = task.future.lock().unwrap();
      if let Some(mut future) = future_guard.take() {
        let waker = waker_ref(&task);
        let mut context = Context::from_waker(&*waker);
        if future.as_mut().poll(&mut context).is_pending() {
          *future_guard = Some(future);
        }
      }
    }
  }
}

