mod arena;
mod value;
mod mem_align;
mod var_map;
mod str_var_map;
mod traits;
mod types;

#[cfg(test)]
mod tests;

pub(crate) use arena::{Arena, ArenaIndex};
pub(crate) use traits::VarMapStoredValue;
pub(crate) use value::ValueKind;

pub use mem_align::MemAlign;
pub use var_map::VarMap;
pub use var_map::Key;
pub use str_var_map::StrVarMap;
pub use traits::VarMapValue;
pub use value::Value;
pub use value::ValueBuilder;
pub use varmap_proc_macro::*;