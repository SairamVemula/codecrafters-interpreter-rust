pub mod callable;
pub mod clock_fn;
pub mod environment;
pub mod function;
pub mod interpreter;
pub mod resolver;
pub mod class;
pub mod instance;

pub use callable::Callable;
pub use environment::Environment;
pub use function::Function;
pub use interpreter::Interpreter;
pub use class::Class;
pub use instance::Instance;

use crate::error::RuntimeError;

pub type Result<T> = std::result::Result<T, RuntimeError>;
