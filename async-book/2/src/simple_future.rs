use std::fmt::Debug;


pub enum Poll<T> {
  Done(T),
  Pending,
}

pub enum PartialPoll<F, T> {
  Done(T),
  Pending(F),
  Dropped,
}

impl<F: SimpleFuture<Output = T>, T> PartialPoll<F, T>  {
  pub fn poll(&mut self, wake: fn()) -> bool {
    if let Self::Pending(f) = self {
      match f.poll(wake) {
        Poll::Done(x) => {*self = Self::Done(x); true},
        Poll::Pending => false,
      }
    } else {
      true
    }
  }

  pub fn ok(self) -> Result<T, Self> {
    if let Self::Done(x) = self {
      Ok(x)
    } else {
      Err(self)
    }
  }

  pub fn take(&mut self) -> T {
    std::mem::take(self).ok().expect("PartialPoll wasn't done yet or was already dropped")
  }
}

pub trait SimpleFuture {
  type Output;
  fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

pub struct Join<A, B, X, Y>(PartialPoll<X, A>, PartialPoll<Y, B>)
  where X: SimpleFuture<Output = A>, Y: SimpleFuture<Output = B>;

pub struct AndThen<A, B, X, Y>(PartialPoll<X, A>, PartialPoll<Y, B>)
  where X: SimpleFuture<Output = A>, Y: SimpleFuture<Output = B>;



  impl<A, B, X, Y> SimpleFuture for Join<A, B, X, Y> where X: SimpleFuture<Output = A>, Y: SimpleFuture<Output = B> {
    type Output = (A, B);
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
      let done0 = self.0.poll(wake);
      let done1 = self.1.poll(wake);
      if done0 && done1 {
        Poll::Done((self.0.take(), self.1.take()))
      } else {
        Poll::Pending
      }
    }
  }
  
impl<A, B, X, Y> SimpleFuture for AndThen<A, B, X, Y> where X: SimpleFuture<Output = A>, Y: SimpleFuture<Output = B> {
  type Output = (A, B);
  fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
    if self.0.poll(wake) {
      if self.1.poll(wake) {
        return Poll::Done((self.0.take(), self.1.take()))
      }
    }
    Poll::Pending
  }
}

    

impl<T> From<Option<T>> for Poll<T> {
  fn from(value: Option<T>) -> Self {
    match value {
      Some(x) => Self::Done(x),
      None => Self::Pending,
    }
  }
}

impl<F, T> Default for PartialPoll<F, T> {
  fn default() -> Self {
    Self::Dropped
  }
}

impl<F, T> Debug for PartialPoll<F, T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Done(_) => f.debug_tuple("Done").field(&"..").finish(),
      Self::Pending(_) => f.debug_tuple("Pending").field(&"..").finish(),
      Self::Dropped => write!(f, "Dropped"),
    }
  }
}
