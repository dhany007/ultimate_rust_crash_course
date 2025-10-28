pub const FIRST: i32 = 1;
pub const SECOND: i32 = 2;
pub const THIRD: i32 = 3;

pub mod sound;

pub mod prelude {
    pub use super::{FIRST, SECOND, THIRD}; // re-export semua item publik dari library
    pub use super::sound::tame::*;
    pub use super::sound::wild::*;
}
