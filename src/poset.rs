pub trait Join<Other = Self>
    where Self: PartialOrd<Other> {

    type Output;

    fn join(self, other: Other) -> Self::Output;
}

pub trait Meet<Other = Self>
    where Self: PartialOrd<Other> {

    type Output;

    fn meet(self, other: Other) -> Self::Output;
}

pub trait Lattice: Join<Output = Self> + Meet<Output = Self> + Sized {
}

impl<A> Meet for Option<A>
    where A: Meet<Output = Option<A>> {

    type Output = Self;

    fn meet(self, other: Self) -> Self {
        match (self, other) {
            (None, _) => None,
            (_, None) => None,
            (Some(a), Some(b)) => a.meet(b)
        }
    }
}

impl<A> Join for Option<A>
    where A: Join<Output = Option<A>> {

    type Output = Self;

    fn join(self, other: Self) -> Self {
        match (self, other) {
            (None, o) => o,
            (o, None) => o,
            (Some(a), Some(b)) => a.join(b)
        }
    }
}

impl<A> Lattice for Option<A>
    where A: Meet<Output = Option<A>> + Join<Output = Option<A>> {
}
