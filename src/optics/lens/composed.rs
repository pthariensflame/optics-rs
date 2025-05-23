use crate::HasSetter;
use crate::optics::lens::Lens;
use crate::{HasGetter, HasTotalGetter, LensImpl};
use core::convert::Infallible;
use core::marker::PhantomData;

/// A composed `Lens` type, combining two optics into a single `Lens`.
///
/// This struct is automatically created by composing two existing optics, and is **not** intended
/// to be directly constructed outside the crate. Instead, it is generated through composition of
/// two optics via the corresponding `ComposableXXX` traits, where each optic can be any
/// valid optic type where the result is a `Lens`.
///
/// A `ComposedLens` not only combines two optics into a single lens, but it also inherently
/// acts as a `Prism` and `Optic`. This behavior arises from the fact that a `Lens` is itself a
/// more specific form of an optic, and prism and thus any `Lens` composition will also be usable as
/// a `Prism` and an `Optic`.
///
/// # Construction
///
/// This struct **cannot** be manually constructed by users. Instead, it is created via
/// composition of two optics using the appropriate `ComposableXXX` trait for each optic type.
/// The `ComposedLens` structure is provided internally by the crate after you compose valid optics.
///
/// # See Also
///
/// - [`Lens`] — the core optic type that the `ComposedLens` is based on
/// - [`Prism`] — the optic type that `ComposedLens` also acts as
/// - [`Optic`] — the base trait that all optic types implement
/// - [`crate::composers::ComposableLens`] — a trait for composing [`Lens`] optics another [`Optic`]
/// - [`crate::composers::ComposablePrism`] — a trait for composing [`Prism`] optics another [`Optic`]
/// - [`crate::composers::ComposableIso`] — a trait for composing [`Iso`] optics into another [`Optic`]
/// - [`crate::composers::ComposableFallibleIso`] — a trait for composing [`FallibleIso`] optics into another [`Optic`]
struct ComposedLens<L1: Lens<S, I>, L2: Lens<I, A>, S, I, A> {
    optic1: L1,
    optic2: L2,
    _phantom: PhantomData<(S, I, A)>,
}

impl<L1, L2, S, I, A> ComposedLens<L1, L2, S, I, A>
where
    L1: Lens<S, I>,
    L2: Lens<I, A>,
{
    fn new(optic1: L1, optic2: L2) -> Self {
        ComposedLens {
            optic1,
            optic2,
            _phantom: PhantomData,
        }
    }
}

impl<S, I, A, L1, L2> HasGetter<S, A> for ComposedLens<L1, L2, S, I, A>
where
    L1: Lens<S, I>,
    L2: Lens<I, A>,
{
    type GetterError = Infallible;

    fn try_get(&self, source: &S) -> Result<A, Self::GetterError> {
        let i = self.optic1.try_get(source)?;
        self.optic2.try_get(&i)
    }
}

impl<S, I, A, L1, L2> HasSetter<S, A> for ComposedLens<L1, L2, S, I, A>
where
    L1: Lens<S, I>,
    L2: Lens<I, A>,
{
    fn set(&self, source: &mut S, value: A) {
        let mut i = self.optic1.get(source);
        self.optic2.set(&mut i, value);
        self.optic1.set(source, i);
    }
}

#[must_use]
pub fn new<S, A, I, L1: Lens<S, I>, L2: Lens<I, A>>(
    l1: L1,
    l2: L2,
) -> LensImpl<S, A, impl Lens<S, A>> {
    ComposedLens::new(l1, l2).into()
}
