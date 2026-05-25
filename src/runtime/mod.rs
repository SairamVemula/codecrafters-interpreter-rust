pub mod callable;
pub mod clock_fn;
pub mod environment;
pub mod function;
pub mod interpreter;

pub use callable::Callable;
pub use environment::Environment;
pub use function::Function;
pub use interpreter::Interpreter;

use crate::error::RuntimeError;

pub type Result<T> = std::result::Result<T, RuntimeError>;
