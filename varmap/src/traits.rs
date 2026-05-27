use crate::{Value, ValueBuilder, ValueMut};

/// Types that can be stored in and read from a varmap.
///
/// Implemented for all built-in storable types and for custom `Copy` types via
/// `#[derive(VarMapValue)]`.
pub trait VarMapValue {
    /// Type returned by [`VarMap::get`](crate::VarMap::get) and related getters.
    type Decoded<'a>;

    /// Type discriminator for custom values (`0` for built-in types).
    const TYPE_ID: u32;

    /// Encodes `self` into a [`Value`] using `builder` for arena-backed payloads.
    fn to_value<'a>(&self, builder: &'a mut ValueBuilder<'a>) -> Value<'a>;

    /// Decodes `self` from a [`Value`].
    ///
    /// The arena lifetime `'a` comes from [`Value`], not from the reference to it.
    fn from_value<'a>(value: &Value<'a>) -> Option<Self::Decoded<'a>>;

    /// Updates a stored value in place when supported for this type.
    ///
    /// Returns `false` if the slot does not hold this type or the type cannot be updated in place.
    fn update_in_place<F>(value: &mut ValueMut<'_>, f: F) -> bool
    where
        F: FnOnce(&mut Self),
        Self: Sized,
    {
        let _ = (value, f);
        false
    }
}

/// Enum key type for [`EnumVarMap`](struct@crate::EnumVarMap).
///
/// Usually implemented with `#[derive(EnumVarMap)]` rather than by hand.
pub trait EnumVarMapKey: Copy {
    /// Number of enum variants (and slots in the map).
    const INDEX_COUNT: u16;

    /// Index of the slot for this variant.
    fn to_index(self) -> u16;
}
