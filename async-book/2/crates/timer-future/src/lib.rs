use std::{
  task::{Waker, Context, Poll},
  future::Future,
  pin::Pin,
  sync::{Mutex, Arc}, time::Duration, thread
};

#[derive(Default)]
pub struct TimerFuture {
  state: Arc<Mutex<SharedState>>
}


#[derive(Default)]
pub struct SharedState {
  completed: bool,
  waker: Option<Waker>,
}

impl Future for TimerFuture {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let mut state_guard = self.state.lock().unwrap();
    if state_guard.completed {
      Poll::Ready(())
    } else{
      state_guard.waker = Some(cx.waker().clone());
      Poll::Pending
    }
  }
}


impl TimerFuture {
  pub fn new(duration: Duration) -> Self {
    let res = Self::default();
    let shared_state = res.state.clone();
    thread::Builder::new().name("TimeFuture".to_string()).spawn(move || {
      thread::sleep(duration);
      let mut state_guard = shared_state.lock().unwrap();
      state_guard.completed = true;
      if let Some(waker) = state_guard.waker.take() {
        drop(state_guard);
        waker.wake()
      }
    }).unwrap();
    res
  }
}
