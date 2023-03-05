use rkyv::{
    niche::option_nonzero::ArchivedOptionNonZeroU64,
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Fallible,
};

use super::Id;

type ArchivedOptionId = ArchivedOptionNonZeroU64;

/// An rkyv wrapper that niches `Option<Id<T>>` into an optimized layout.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
/// use rkyv::{Archive, Archived};
/// use twilight_model::id::{Id, IdNiche, marker::UserMarker};
///
/// #[derive(Archive)]
/// struct BasicExample {
///     id_opt: Option<Id<UserMarker>>,
/// }
///
/// #[derive(Archive)]
/// struct NichedExample {
///     #[with(IdNiche)]
///     id_opt: Option<Id<UserMarker>>,
/// }
///
/// assert!(size_of::<Archived<BasicExample>>() > size_of::<Archived<NichedExample>>);
/// ```
pub struct IdNiche;

impl<T> ArchiveWith<Option<Id<T>>> for IdNiche {
    type Archived = ArchivedOptionId;
    type Resolver = ();

    #[inline]
    #[allow(unsafe_code)]
    unsafe fn resolve_with(
        field: &Option<Id<T>>,
        _: usize,
        _: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        ArchivedOptionId::resolve_from_option(field.map(Id::into_nonzero), out);
    }
}

impl<S: Fallible + ?Sized, T> SerializeWith<Option<Id<T>>, S> for IdNiche {
    #[inline]
    fn serialize_with(_: &Option<Id<T>>, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized, T> DeserializeWith<ArchivedOptionId, Option<Id<T>>, D> for IdNiche {
    #[inline]
    fn deserialize_with(field: &ArchivedOptionId, _: &mut D) -> Result<Option<Id<T>>, D::Error> {
        Ok(field.as_ref().map(|&id| Id::from_nonzero(id)))
    }
}
