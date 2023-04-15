// #![warn(missing_docs)]              // full docs
#![deny(non_snake_case)]            // match { ... } bugs
#![deny(unreachable_patterns)]      // match { ... } bugs

mod addr;                           pub use addr::*;
mod decode;                         pub use decode::*;
mod nibble;                         pub use nibble::*;
mod op;                             pub use op::*;
mod v;                              pub use v::*;
