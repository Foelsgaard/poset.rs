use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};

use poset::{Meet, Join, Lattice};

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Hash)]
#[derive(Debug)]
pub struct Interval<A, B> {a: A, b: B}

impl<A, B> Interval<A, B>
    where A: PartialOrd<A> + PartialOrd<B>,
          B: PartialOrd<A> + PartialOrd<B> {
    
    pub fn new(a: A, b: B) -> Option<Self> {
        if a <= b {
            Some(Interval{a: a, b: b})
        } else {
            None
        }
    }
    
    pub fn contains<C: PartialOrd<A> + PartialOrd<B>>(&self, c: &C) -> bool{
        let &Interval{ref a, ref b} = self;
        c >= a && c <= b
    }
}

impl<A, B> PartialOrd for Interval<A, B>
    where A: PartialOrd<A> + PartialOrd<B>,
          B: PartialOrd<A> + PartialOrd<B> {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.a.partial_cmp(&other.a),
               self.b.partial_cmp(&other.b)) {

            (Some(Equal), o) => o,
            (o, Some(Equal)) => o,
            (Some(Greater), Some(Less)) => Some(Less),
            (Some(Less), Some(Greater)) => Some(Greater),
            _ => None
        }
    }
}

impl<A, B> Join for Interval<A, B>
    where A: Lattice + PartialOrd<B>,
          B: Lattice + PartialOrd<A> {

    type Output = Option<Self>;

    fn join(self, other: Self) -> Option<Self> {
        let a = self.a.meet(other.a);
        let b = self.b.join(other.b);

        Interval::new(a, b)
    }
}

impl<A, B> Meet for Interval<A, B>
    where A: Lattice + PartialOrd<B>,
          B: Lattice + PartialOrd<A> {

    type Output = Option<Self>;

    fn meet(self, other: Self) -> Option<Self> {
        let a = self.a.join(other.a);
        let b = self.b.meet(other.b);

        Interval::new(a, b)
    }
}
