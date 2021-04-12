pub type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Allows calling the NormalizeResult trait point-free.
#[macro_export]
macro_rules! ee {
  ($f:path) => {
    (|v| $f(v).ee())
  }
}

pub mod monad;
pub use monad::*;

pub mod tap;
pub use tap::*;

pub mod functor;
pub use functor::*;

pub mod iter;
pub use iter::*;

pub mod result;
pub use result::*;
