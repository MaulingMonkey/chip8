// #![warn(missing_docs)]              // full docs
#![deny(non_snake_case)]            // match { ... } bugs
#![deny(unreachable_patterns)]      // match { ... } bugs

mod addr;                           pub use addr::*;
mod decode;                         pub use decode::*;
mod memory;                         pub use memory::*;
mod nibble;                         pub use nibble::*;
mod op;                             pub use op::*;
mod registers;                      pub use registers::*;
mod run;                            pub use run::*;
mod screen;                         pub use screen::*;
mod v;                              pub use v::*;
