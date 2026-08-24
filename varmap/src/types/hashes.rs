use crate::*;

macro_rules! impl_varmap_hash {
    ($($name:ident, $bits:literal, $nbytes:literal),* $(,)?) => {
        $(
            #[doc = concat!(
                "A ", stringify!($bits), "-bit hash digest (`[u8; ", stringify!($nbytes), "]`) stored in the map arena."
            )]
            pub type $name = [u8; $nbytes];

            impl VarMapValue for $name {
                type Decoded<'a> = &'a $name;

                const TYPE_ID: u32 = 0;

                fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a> {
                    Value::new(
                        ValueKind::$name(builder.arena_mut().store(self, MemAlign::Bits8)),
                        builder.arena(),
                    )
                }

                fn from_value<'a>(value: &Value<'a>) -> Option<&'a $name> {
                    match value.kind() {
                        ValueKind::$name(index) => value.arena().get(*index)?.try_into().ok(),
                        _ => None,
                    }
                }

                fn update<F>(value: &mut ValueMut<'_>, f: F) -> bool
                where
                    F: FnOnce(&mut $name),
                {
                    let index = match *value.kind_mut() {
                        ValueKind::$name(index) => index,
                        _ => return false,
                    };
                    let Some(bytes) = value.arena_mut().get_mut(index) else {
                        return false;
                    };
                    let Ok(hash) = <&mut $name>::try_from(bytes) else {
                        return false;
                    };
                    f(hash);
                    true
                }
            }
        )*
    };
}

impl_varmap_hash! {
    Hash128, 128, 16,
    Hash160, 160, 20,
    Hash256, 256, 32,
    Hash384, 384, 48,
    Hash512, 512, 64,
}
