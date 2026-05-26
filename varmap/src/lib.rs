//! Heterogeneous, typed key–value maps for Rust.
//!
//! Each key holds one value of a known type. Values use a compact 16-byte representation;
//! strings, byte slices, and some large types are stored in a per-map arena so reads can
//! return `&str` and `&[u8]` without allocating.
//!
//! # Map types
//!
//! | Type | Keys | When to use |
//! | ---- | ---- | ----------- |
//! | [`VarMap`] | [`Key`] (compile-time hash via [`var!`]) | Fixed names known at compile time |
//! | [`StrVarMap`] | `&str` (FNV-1a at runtime) | Dynamic names from config or user input |
//! | [`struct@EnumVarMap`] | `E: [`EnumVarMapKey`]` | Closed set of keys as an enum |
//!
//! # Usage model
//!
//! VarMap is aimed at **write once, read many** workloads. The arena is append-only:
//! overwriting a key replaces the map entry but does not free a previous arena allocation.
//! Prefer setting each key once, then reading often. Call [`VarMap::clear`], [`StrVarMap::clear`],
//! or [`EnumVarMap::clear`] to reset a map between logical snapshots; `clear` retains allocated
//! capacity for later writes.
//!
//! # Supported values
//!
//! Built-in types: integers, floats, `bool`, `char`, `&str`, `&[u8]`, and IP address types.
//! Strings and byte slices up to 14 bytes are stored inline; longer payloads use the arena.
//!
//! Custom `Copy` types (alignment 1–16) can use `#[derive(VarMapValue)]`.
//!
//! # Examples
//!
//! ```
//! use varmap::StrVarMap;
//!
//! let mut map = StrVarMap::new();
//! map.set("port", 8080u16);
//! map.set("host", "localhost");
//!
//! assert_eq!(map.get_u16("port"), Some(8080));
//! assert_eq!(map.get_str("host"), Some("localhost"));
//! ```

mod arena;
mod value;
mod mem_align;
mod var_map;
mod str_var_map;
mod enum_var_map;
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
#[doc(hidden)]
pub use traits::VarMapCustomValue;
pub use traits::EnumVarMapKey;
pub use value::Value;
pub use value::ValueBuilder;
pub use varmap_proc_macro::*;
pub use enum_var_map::EnumVarMap;
