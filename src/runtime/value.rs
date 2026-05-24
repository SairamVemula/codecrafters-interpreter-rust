// use super::*;
// use std::fmt::{self, Display};


// #[derive(Clone, Debug, PartialEq)]
// pub enum RuntimeValue {
//     Null,
//     String(String),
//     Number(f64, String),
//     Boolean(bool),
// }

// impl Display for RuntimeValue {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             RuntimeValue::String(s) => write!(f, "{s}"),
//             RuntimeValue::Number(_, s) => write!(f, "{s}"),
//             RuntimeValue::Null => write!(f, "nil"),
//             RuntimeValue::Boolean(b) => write!(f, "{b}"),
//         }
//     }
// }
// impl RuntimeValue {
//     pub fn is_truthy(&self) -> bool {
//         match self {
//             RuntimeValue::Null => false,
//             RuntimeValue::String(_) => true,
//             RuntimeValue::Number(_, _) => true,
//             RuntimeValue::Boolean(b) => *b,
//         }
//     }
// }
