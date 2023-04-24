// #![warn(missing_docs)]              // full docs
#![deny(non_snake_case)]            // match { ... } bugs
#![deny(unreachable_patterns)]      // match { ... } bugs

mod addr;                           pub use addr::*;
mod context;                        pub use context::*;
mod decode;                         pub use decode::*;
pub mod font;
mod memory;                         pub use memory::*;
mod nibble;                         pub use nibble::*;
mod op;                             pub use op::*;
mod registers;                      pub use registers::*;
mod screen;                         pub use screen::*;
mod syscalls;                       pub use syscalls::*;
pub mod tls;
mod v;                              pub use v::*;
