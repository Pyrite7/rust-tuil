use std::ops::{Add, AddAssign};

use crate::Vec2;

use super::rect::Rect;


/// Iterates over all Vec2's in a rectangle.
/// Iterates like text is read: primarily increment x from left to right, and when x goes beyond max, increment y and reset x.
pub struct RectIter<Elem: AddAssign + Default + PartialOrd + Clone + Copy> {
    rect: Rect<Elem>,
    current: Vec2<Elem>,
    step: Vec2<Elem>,
}

impl<Elem: AddAssign + Default + PartialOrd + Clone + Copy> RectIter<Elem> {
    pub fn new(rect: Rect<Elem>, step: Vec2<Elem>) -> Self {
        Self { rect, current: rect.top_left_corner, step }
    }
}

impl<Elem: Add<Output = Elem> + AddAssign + Default + PartialOrd + Clone + Copy> Iterator for RectIter<Elem> {
    type Item = Vec2<Elem>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.current.y >= self.rect.top_right_corner().y {
            None
        } else {
            Some(self.current)
        };

        self.current.x += self.step.x;
        if self.current.x >= self.rect.top_right_corner().x {
            self.current.y += self.step.y;
            self.current.x = Elem::default();
        }

        res
    }
}

impl<Elem: StepSize + AddAssign + Default + PartialOrd + Clone + Copy> Vec2<Elem> {
    pub fn rect_iter(self) -> RectIter<Elem> {
        RectIter::new(Rect { top_left_corner: Vec2::default(), size: self }, Elem::step())
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



