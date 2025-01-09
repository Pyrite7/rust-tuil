use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};


/// A simple, generically implemented 2-dimensional vector struct which can be used for positioning logic.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Vec2<Elem> {
    pub x: Elem,
    pub y: Elem,
}

impl<Elem> Vec2<Elem> {
    pub fn new(x: Elem, y: Elem) -> Self {
        Self { x, y }
    }    

    pub fn transmogrifuckify(&self) -> Vec2<&Elem> {
        Vec2::new(&self.x, &self.y)
    }

    pub fn into<Elem2: From<Elem>>(self) -> Vec2<Elem2> {
        Vec2::new(self.x.into(), self.y.into())
    }
}








impl<Elem> Add for Vec2<Elem>
where Elem: Add {
    type Output = Vec2<<Elem as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl<Elem> Add for &Vec2<Elem>
where Elem: Add + Clone {
    type Output = Vec2<<Elem as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x.clone() + rhs.x.clone(), self.y.clone() + rhs.y.clone())
    }
}
impl<Elem> Add<&Vec2<Elem>> for Vec2<Elem>
where Elem: Add + Clone {
    type Output = Vec2<<Elem as Add>::Output>;

    fn add(self, rhs: &Self) -> Self::Output {
        Vec2::new(self.x + rhs.x.clone(), self.y + rhs.y.clone())
    }
}
impl<Elem> Add<Vec2<Elem>> for &Vec2<Elem>
where Elem: Add + Clone {
    type Output = Vec2<<Elem as Add>::Output>;

    fn add(self, rhs: Vec2<Elem>) -> Self::Output {
        Vec2::new(self.x.clone() + rhs.x, self.y.clone() + rhs.y)
    }
}


impl<Elem> Sub for Vec2<Elem>
where Elem: Sub {
    type Output = Vec2<<Elem as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl<Elem> Sub for &Vec2<Elem>
where Elem: Sub + Clone {
    type Output = Vec2<<Elem as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x.clone() - rhs.x.clone(), self.y.clone() - rhs.y.clone())
    }
}
impl<Elem> Sub<&Vec2<Elem>> for Vec2<Elem>
where Elem: Sub + Clone {
    type Output = Vec2<<Elem as Sub>::Output>;

    fn sub(self, rhs: &Self) -> Self::Output {
        Vec2::new(self.x - rhs.x.clone(), self.y - rhs.y.clone())
    }
}
impl<Elem> Sub<Vec2<Elem>> for &Vec2<Elem>
where Elem: Sub + Clone {
    type Output = Vec2<<Elem as Sub>::Output>;

    fn sub(self, rhs: Vec2<Elem>) -> Self::Output {
        Vec2::new(self.x.clone() - rhs.x, self.y.clone() - rhs.y)
    }
}


impl<Elem, Rhs: Clone> Mul<Rhs> for Vec2<Elem>
where Elem: Mul<Rhs> {
    type Output = Vec2<<Elem as Mul<Rhs>>::Output>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Vec2::new(self.x * rhs.clone(), self.y * rhs.clone())
    }
}
impl<Elem, Rhs: Clone> Mul<Rhs> for &Vec2<Elem>
where Elem: Mul<Rhs> + Clone {
    type Output = Vec2<<Elem as Mul<Rhs>>::Output>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        Vec2::new(self.x.clone() * rhs.clone(), self.y.clone() * rhs.clone())
    }
}


impl<Elem: Neg> Neg for Vec2<Elem> {
    type Output = Vec2<<Elem as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}


impl<Elem: AddAssign> AddAssign for Vec2<Elem> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<Elem: AddAssign + Clone> AddAssign<&Self> for Vec2<Elem> {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x.clone();
        self.y += rhs.y.clone();
    }
}


impl<Elem: SubAssign> SubAssign for Vec2<Elem> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl<Elem: SubAssign + Clone> SubAssign<&Self> for Vec2<Elem> {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x.clone();
        self.y -= rhs.y.clone();
    }
}


impl<Elem: MulAssign<Rhs>, Rhs: Clone> MulAssign<Rhs> for Vec2<Elem> {
    fn mul_assign(&mut self, rhs: Rhs) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
    }
}
impl<Elem: MulAssign<Rhs>, Rhs: Clone> MulAssign<Rhs> for &mut Vec2<Elem> {
    fn mul_assign(&mut self, rhs: Rhs) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::iterators::Vec2Iter;

    #[test]
    fn add_works() {
        let v1 = Vec2::new(1, 2);
        let v2 = Vec2::new(3, 4);
        let borrow1 = &v1;
        let borrow2 = &v2;

        let res = Vec2::new(4, 6);

        assert_eq!(v1 + v2, res);
        assert_eq!(v1 + borrow2, res);
        assert_eq!(borrow1 + v2, res);
        assert_eq!(borrow1 + borrow2, res);
    }

    #[test]
    fn add_assign_works() {
        let mut v1 = Vec2::new(1, 2);
        let v2 = Vec2::new(3, 4);
        let borrow1 = &mut v1;
        let borrow2 = &v2;

        *borrow1 += v2;
        assert_eq!(v1, Vec2::new(4, 6));
        v1 += v2;
        assert_eq!(v1, Vec2::new(7, 10));
        v1 += borrow2;
        assert_eq!(v1, Vec2::new(10, 14));
    }

    #[test]
    fn sub_works() {
        let v1 = Vec2::new(10, 17);
        let v2 = Vec2::new(3, 4);
        let borrow1 = &v1;
        let borrow2 = &v2;

        let res = Vec2::new(7, 13);

        assert_eq!(v1 - v2, res);
        assert_eq!(v1 - borrow2, res);
        assert_eq!(borrow1 - v2, res);
        assert_eq!(borrow1 - borrow2, res);
    }

    #[test]
    fn sub_assign_works() {
        let mut v1 = Vec2::new(10, 17);
        let v2 = Vec2::new(3, 4);
        let borrow1 = &mut v1;
        let borrow2 = &v2;

        *borrow1 -= v2;
        assert_eq!(v1, Vec2::new(7, 13));
        v1 -= v2;
        assert_eq!(v1, Vec2::new(4, 9));
        v1 -= borrow2;
        assert_eq!(v1, Vec2::new(1, 5));
    }

    #[test]
    fn neg_works() {
        let v = Vec2::new(2, -5);

        let res = Vec2::new(-2, 5);

        assert_eq!(-v, res);
    }

    #[test]
    fn mul_works() {
        let v = Vec2::new(2, 3);
        let s = 5;
        let borrow_v = &v;
        let borrow_s = &s;

        let res = Vec2::new(10, 15);

        assert_eq!(v * s, res);
        assert_eq!(v * borrow_s, res);
        assert_eq!(borrow_v * s, res);
        assert_eq!(borrow_v * borrow_s, res);
    }

    #[test]
    fn mul_assign_works() {
        let mut v = Vec2::new(2, 5);
        let s = 3;
        let mut borrow_v = &mut v;
        let borrow_s = &s;

        borrow_v *= borrow_s;
        assert_eq!(v, Vec2::new(6, 15));
        v *= borrow_s;
        assert_eq!(v, Vec2::new(18, 45));
        v *= s;
        assert_eq!(v, Vec2::new(54, 135));
    }

    #[test]
    fn iterator_works() {
        let size = Vec2::new(5, 2);
        let mut iter = Vec2Iter::new(&size, &Vec2::new(1, 1));

        assert_eq!(iter.next(), Some(Vec2::new(0, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(1, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(2, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(3, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(4, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(0, 1)));
        assert_eq!(iter.next(), Some(Vec2::new(1, 1)));
        assert_eq!(iter.next(), Some(Vec2::new(2, 1)));
        assert_eq!(iter.next(), Some(Vec2::new(3, 1)));
        assert_eq!(iter.next(), Some(Vec2::new(4, 1)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn rect_iter_works() {
        let size = Vec2::new(5, 2);
        let mut iter = size.rect_iter();

        assert_eq!(iter.next(), Some(Vec2::new(0, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(1, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(2, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(3, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(4, 0)));
        assert_eq!(iter.next(), Some(Vec2::new(0, 1)));
        assert_eq!(iter.next(), Some(Vec2::new(1, 1)));
        assert_eq!(iter.next(), Some(Vec2::new(2, 1)));
        assert_eq!(iter.next(), Some(Vec2::new(3, 1)));
        assert_eq!(iter.next(), Some(Vec2::new(4, 1)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn row_aware_iter_works() {
        let size = Vec2::new(5, 2);
        let mut iter = size.row_aware_iter();

        assert_eq!(iter.next(), Some((Vec2::new(0, 0), false)));
        assert_eq!(iter.next(), Some((Vec2::new(1, 0), false)));
        assert_eq!(iter.next(), Some((Vec2::new(2, 0), false)));
        assert_eq!(iter.next(), Some((Vec2::new(3, 0), false)));
        assert_eq!(iter.next(), Some((Vec2::new(4, 0), true)));
        assert_eq!(iter.next(), Some((Vec2::new(0, 1), false)));
        assert_eq!(iter.next(), Some((Vec2::new(1, 1), false)));
        assert_eq!(iter.next(), Some((Vec2::new(2, 1), false)));
        assert_eq!(iter.next(), Some((Vec2::new(3, 1), false)));
        assert_eq!(iter.next(), Some((Vec2::new(4, 1), true)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}

