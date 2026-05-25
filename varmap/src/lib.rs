mod arena;
mod value;
mod mem_align;
mod key;
mod var_map;
mod traits;
mod types;

#[cfg(test)]
mod tests;

use arena::Arena;
use arena::ArenaIndex;
use value::ValueKind;


pub use key::Key;
pub use mem_align::MemAlign;
pub use var_map::VarMap;
pub use traits::VarMapValue;
pub use value::Value;
pub use value::ValueBuilder;