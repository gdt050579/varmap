use proc_macro::{TokenStream, TokenTree, Delimiter};

const fn fnv1a_32(s: &str) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u32;
        hash = hash.wrapping_mul(0x01000193);
        i += 1;
    }
    hash
}

pub(crate) fn generate_implementation(input: TokenStream) -> Result<TokenStream, String> {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let mut i = 0;

    // Walk past attributes and visibility, look for `struct` or `enum`
    while i < tokens.len() {
        match &tokens[i] {
            TokenTree::Punct(p) if p.as_char() == '#' => {
                // Skip attribute: # [ ... ]
                i += 1;
                if let Some(TokenTree::Group(g)) = tokens.get(i) {
                    if g.delimiter() == Delimiter::Bracket {
                        i += 1;
                        continue;
                    }
                }
            }
            TokenTree::Ident(id) => {
                let s = id.to_string();
                if s == "pub" {
                    i += 1;
                    // Optional (crate)/(super)/(self) restriction
                    if let Some(TokenTree::Group(g)) = tokens.get(i) {
                        if g.delimiter() == Delimiter::Parenthesis {
                            i += 1;
                        }
                    }
                    continue;
                }
                if s == "struct" || s == "enum" {
                    i += 1;
                    break;
                }
                if s == "union" {
                    return Err("VarMapValue cannot be derived on unions".to_string());
                }
                return Err(format!("unexpected token: `{}`", s));
            }
            _ => i += 1,
        }
    }

    // Type name
    let name = match tokens.get(i) {
        Some(TokenTree::Ident(id)) => id.to_string(),
        _ => return Err("expected type name".to_string()),
    };
    i += 1;

    // Reject generics — we'd need to thread them through
    if let Some(TokenTree::Punct(p)) = tokens.get(i) {
        if p.as_char() == '<' {
            return Err("VarMapValue does not support generic types".to_string());
        }
    }
    let type_id = fnv1a_32(&name);
    let out = format!(
        r#"
        impl VarMapValue for {name} {{
            type Decoded<'a> = &'a {name};
            const TYPE_ID: u32 = {type_id}u32;

            fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {{
                let align = MemAlign::from_align(::core::mem::align_of::<{name}>()).unwrap();
                let bytes: &[u8] = unsafe {{::core::slice::from_raw_parts(self as *const {name} as *const u8,::core::mem::size_of::<{name}>())}};
                builder.build(bytes, align, Self::TYPE_ID)
            }}

            fn from_value<'a>(value: &Value<'a>) -> Option<Self::Decoded<'a>> {{
                let buf = value.as_bytes::<{name}>()?;
                unsafe {{ Some(&*(buf.as_ptr() as *const {name})) }}
            }}
        }}
        const _STATIC_ASSERT_FOR_{name}_: fn() = || {{
            fn assert_copy<T: Copy>() {{}}
            assert_copy::<{name}>();
            const _ALIGN_CHECK: () = {{
                let align = ::core::mem::align_of::<{name}>();
                assert!(align >= 1 && align <= 16, "VarMapValue: type alignment must be between 1 and 16 bytes");
            }};            
        }};
        "#,
        name = name,
        type_id = type_id,
    );

    out.parse().map_err(|e| format!("failed to create implementation for {}: {}", name, e))
}
