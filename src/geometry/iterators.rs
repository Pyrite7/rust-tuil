use std::ops::AddAssign;

use crate::Vec2;


/// Iterates over all Vec2's in the rectangle between this and the origin (default) vector.
/// Iterates like text is read: primarily increment x from left to right, and when x goes beyond max, increment y and reset x.
pub struct Vec2Iter<Elem: AddAssign + Default + PartialOrd + Clone + Copy> {
    max: Vec2<Elem>,
    current: Vec2<Elem>,
    step: Vec2<Elem>,
}

impl<Elem: AddAssign + Default + PartialOrd + Clone + Copy> Vec2Iter<Elem> {
    pub fn new(max: &Vec2<Elem>, step: &Vec2<Elem>) -> Self {
        Self { max: *max, current: Vec2::default(), step: *step }
    }

    pub fn to_row_aware(self) -> RowAwareVec2Iter<Elem> {
        RowAwareVec2Iter { iterator: self }
    }
}

impl<Elem: AddAssign + Default + PartialOrd + Clone + Copy> Iterator for Vec2Iter<Elem> {
    type Item = Vec2<Elem>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.current.y >= self.max.y {
            None
        } else {
            Some(self.current)
        };

        self.current.x += self.step.x;
        if self.current.x >= self.max.x {
            self.current.y += self.step.y;
            self.current.x = Elem::default();
        }

        res
    }
}

impl<Elem: StepSize + AddAssign + Default + PartialOrd + Clone + Copy> Vec2<Elem> {
    pub fn rect_iter(&self) -> Vec2Iter<Elem> {
        Vec2Iter::new(self, &Elem::step())
    }

    pub fn row_aware_iter(&self) -> RowAwareVec2Iter<Elem> {
        self.rect_iter().to_row_aware()
    }
}


pub trait StepSize: Sized {
    /// The Vec2Iter step size for this type, for convenience.
    fn step() -> Vec2<Self>;
}

macro_rules! impl_step_size_for_int {
    ($t:ty) => {
        impl StepSize for $t {
            fn step() -> Vec2<Self> {
                Vec2::new(1,1)
            }
        }
    };
}

impl_step_size_for_int!(u8);
impl_step_size_for_int!(u16);
impl_step_size_for_int!(u32);
impl_step_size_for_int!(u64);
impl_step_size_for_int!(u128);
impl_step_size_for_int!(i8);
impl_step_size_for_int!(i16);
impl_step_size_for_int!(i32);
impl_step_size_for_int!(i64);
impl_step_size_for_int!(i128);

/// A newtype of Vec2Iter which is an iterator over (vec: Vec2, b: bool) where b is whether vec is at the end of the current row.
pub struct RowAwareVec2Iter<Elem: AddAssign + Default + PartialOrd + Clone + Copy> {
    iterator: Vec2Iter<Elem>,
}

impl<Elem: AddAssign + Default + PartialOrd + Clone + Copy> Iterator for RowAwareVec2Iter<Elem> {
    /// A tuple containing whatever a corresponding Vec2Iter would return, and a bool representing whether this location is at the end of the current row.
    type Item = (Vec2<Elem>, bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
            .map(|vec| {
                let next_next = self.iterator.current;
                let is_end_of_row = vec.y != next_next.y;
                (vec, is_end_of_row)
            })
    }
}

