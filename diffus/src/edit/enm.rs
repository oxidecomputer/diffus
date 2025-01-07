#[cfg_attr(feature = "serialize-impl", derive(serde::Serialize))]
#[derive(Debug, Eq, PartialEq)]
pub enum Edit<'a, T: ?Sized, Diff> {
    VariantChanged(&'a T, &'a T),
    AssociatedChanged {
        before: &'a T,
        after: &'a T,
        diff: Diff,
    },
}

impl<'a, T: ?Sized, Diff> Edit<'a, T, Diff> {
    pub fn is_variant_changed(&self) -> bool {
        if let Self::VariantChanged(_, _) = self {
            true
        } else {
            false
        }
    }

    pub fn is_associated_changed(&self) -> bool {
        if let Self::AssociatedChanged { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn variant_changed(&self) -> Option<(&'a T, &'a T)> {
        if let Self::VariantChanged(left, right) = self {
            Some((left, right))
        } else {
            None
        }
    }

    pub fn associated_change(&self) -> Option<&Diff> {
        if let Self::AssociatedChanged { diff, .. } = self {
            Some(diff)
        } else {
            None
        }
    }
}
