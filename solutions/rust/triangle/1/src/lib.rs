use std::ops::Add;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: PartialEq + PartialOrd + Clone + Default,
    for<'a> &'a T: Add<&'a T, Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Self> {
        Self::extract_sides(&sides).map(|(a, b, c)| Triangle { a, b, c })
    }

    fn extract_sides(sides: &[T; 3]) -> Option<(T, T, T)> {
        let default = T::default();
        let (a, b, c) = (&sides[0], &sides[1], &sides[2]);

        if a == &default || b == &default || c == &default {
            return None;
        }
        if (a + b) < *c || (a + c) < *b || (b + c) < *a {
            return None;
        }

        Some((a.clone(), b.clone(), c.clone()))
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.a == self.c
    }
}
