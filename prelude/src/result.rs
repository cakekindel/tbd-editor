use super::monad::*;

pub trait NormalizeResult<T>: Sized {
  /// normalize the Err of a Result to a Boxed std::Error
  fn norm(self) -> Result<T, crate::AnyError>;

  /// shorthand for norm
  fn ee(self) -> Result<T, crate::AnyError> {self.norm()}
}

pub trait Open<A>
  where Self: Sized
{
  /// Given a Result that has been homogenized to Result<T, T>,
  /// extract the T
  fn open(self) -> A;
}

pub trait ThenDo<T, E>
  where Self: Sized + Monad<T, T, Out = Result<T, E>>
{
  /// Perform an action with the Ok value of a result, keeping an error it produces
  /// but discarding its Ok value
  fn then_do<T_>(self, f: impl FnOnce(&mut T) -> Result<T_, E>) -> Result<T, E>;
}

impl<T, E> ThenDo<T, E> for Result<T, E> {
  fn then_do<T_>(self, f: impl FnOnce(&mut T) -> Result<T_, E>) -> Result<T, E> {
    self.bind(|mut t| f(&mut t).map(|_| t))
  }
}

impl<T, E: std::error::Error + Send + Sync + 'static> NormalizeResult<T>
  for Result<T, E>
{
  fn norm(self) -> Result<T, crate::AnyError> {
    self.map_err(|e| Box::new(e) as crate::AnyError)
  }
}

impl<A> Open<A> for Result<A, A> {
  fn open(self) -> A {
    self.unwrap_or_else(|e| e)
  }
}
