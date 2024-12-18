use crate::{
    edit::{self, enm},
    Diffable,
};
use newtype_uuid::{TypedUuid, TypedUuidKind};

/// Implements `Diffable` for `TypedUuid<T>`, if `T` implements `Diffable`.
impl<'a, T> Diffable<'a> for TypedUuid<T>
where
    T: TypedUuidKind + Diffable<'a>,
{
    type Diff = (&'a Self, &'a Self);

    #[inline]
    fn diff(&'a self, other: &'a Self) -> edit::Edit<'a, Self> {
        if self.uuid == other.uuid {
            edit::Edit::Copy(self)
        } else {
            edit::Edit::Change((self, other))
        }
    }
}
