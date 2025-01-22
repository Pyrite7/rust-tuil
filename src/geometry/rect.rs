
use std::ops::{Add, AddAssign};

use crate::{Converted, Vec2};

use super::iterators::{RectIter, StepSize};



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rect<Elem> {
    pub top_left_corner: Vec2<Elem>,
    pub size: Vec2<Elem>,
}

impl<Elem> Rect<Elem> {
    pub fn new(left_x: Elem, top_y: Elem, right_x: Elem, bottom_y: Elem) -> Self {
        Self { top_left_corner: Vec2::new(left_x, top_y), size: Vec2::new(right_x, bottom_y) }
    }
}

impl<Elem: Add<Output = Elem> + Copy> Rect<Elem> {
    pub fn top_right_corner(&self) -> Vec2<Elem> {
        self.top_left_corner + self.size
    }
}

impl<Elem: Add<Output = Elem> + Ord + Copy> Rect<Elem> {
    pub fn contains(&self, point: Vec2<Elem>) -> bool {
        point.x >= self.top_left_corner.x 
     && point.x < self.size.x + self.top_left_corner.x
     && point.y >= self.top_left_corner.y 
     && point.y < self.size.y + self.top_left_corner.y
 }
}

impl<Elem: StepSize + Add<Output = Elem> + AddAssign + Default + PartialOrd + Clone + Copy> IntoIterator for Rect<Elem> {
    type Item = Vec2<Elem>;

    type IntoIter = RectIter<Elem>;

    fn into_iter(self) -> Self::IntoIter {
        RectIter::new(self, Elem::step())
    }
}

impl<Elem, Elem2: TryFrom<Elem>> TryFrom<Rect<Elem>> for Converted<Rect<Elem2>> {
    type Error = <Elem2 as TryFrom<Elem>>::Error;

    fn try_from(value: Rect<Elem>) -> Result<Self, Self::Error> {
        let top_left_corner = value.top_left_corner.try_into()?;
        let size = value.size.try_into()?;
        Ok(Converted(Rect {top_left_corner, size}))
    }
}

