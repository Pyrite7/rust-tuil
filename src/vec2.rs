use std::{clone, ops::{Add, Mul, Neg, Sub}};




#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
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




#[cfg(test)]
mod tests {
    use super::*;

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
}

