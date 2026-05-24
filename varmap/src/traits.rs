use crate::{Value, ValueBuilder};

pub trait VarMapValue {
    type Decoded<'a>;

    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a>;
    fn from_value<'a>(value: &'a Value<'a>) -> Option<Self::Decoded<'a>>;
}
