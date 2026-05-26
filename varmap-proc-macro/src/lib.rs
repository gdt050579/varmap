//! Proc-macros for the [`varmap`](https://docs.rs/varmap) crate.
//!
//! Re-exported from `varmap` as [`var`](var), [`EnumVarMap`](EnumVarMap), and
//! [`VarMapValue`](VarMapValue).

mod derive;
mod copy_types;
use proc_macro::*;
use std::str::FromStr;
extern crate proc_macro;

#[inline(always)]
const fn fnv1a(s: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000000001b3);
        i += 1;
    }
    hash
}

/// Expands a string literal to `varmap::Key::new(<fnv1a hash>)`.
///
/// Use with [`VarMap`](https://docs.rs/varmap/latest/varmap/struct.VarMap.html) for
/// compile-time key names without runtime hashing.
///
/// # Panics
///
/// Panics at compile time if the argument is not exactly one non-empty string literal.
///
/// # Example
///
/// ```ignore
/// use varmap::{VarMap, var};
///
/// let mut map = VarMap::new();
/// map.set(var!("app.port"), 443u16);
/// ```
#[proc_macro]
pub fn var(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();

    let mut string_param = match tokens.next() {
        Some(TokenTree::Literal(lit)) => lit.to_string(),
        _ => panic!("The parameter provided to the 'var!' macro must be a string literal."),
    };

    if tokens.peek().is_some() {
        panic!("Exactly one string must be provided as input.");
    }
    if (!string_param.starts_with('\"')) || (!string_param.ends_with('\"')) {
        panic!("The parameter provided to the 'var!' macro must be a string literal.");
    }
    if string_param.len() == 2 {
        panic!("You can not provide an empty string for 'var!' macro !");
    }

    string_param.remove(0);
    string_param.remove(string_param.len() - 1);
    let hash = fnv1a(&string_param);
    TokenStream::from_str(format!("Key::new({})", hash).as_str())
        .expect("Fail to convert name! to stream")
}

/// Derives [`EnumVarMapKey`](https://docs.rs/varmap/latest/varmap/trait.EnumVarMapKey.html)
/// for a unit enum.
///
/// # Requirements
///
/// - `#[repr(u16)]` on the enum
/// - Unit variants only (no fields, no explicit discriminants)
/// - At most 65 536 variants
#[proc_macro_derive(EnumVarMap)]
pub fn derive_enum_var_map(input: TokenStream) -> TokenStream {
    match derive::process_enum_var_map(input) {
        Ok(ts) => ts,
        Err(msg) => format!("compile_error!({:?});", msg).parse().unwrap(),
    }
}

/// Derives [`VarMapValue`](https://docs.rs/varmap/latest/varmap/trait.VarMapValue.html)
/// for a `Copy` struct or enum.
///
/// # Requirements
///
/// - `Copy` type without generic parameters
/// - Alignment between 1 and 16 bytes
#[proc_macro_derive(VarMapValue)]
pub fn derive_varmap_value(input: TokenStream) -> TokenStream {
    match copy_types::generate_implementation(input) {
        Ok(ts) => ts,
        Err(msg) => format!("compile_error!({:?});", msg).parse().unwrap(),
    }
}
