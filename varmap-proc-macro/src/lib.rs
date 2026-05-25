mod derive;
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


#[proc_macro_derive(EnumVarMap)]
pub fn derive_enum_var_map(input: TokenStream) -> TokenStream {
    match derive::process_enum_var_map(input) {
        Ok(ts) => ts,
        Err(msg) => format!("compile_error!({:?});", msg).parse().unwrap(),
    }
}