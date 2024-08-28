/// Represents the dual number a+b*epsilon, where epsilon \neq 0 satisfies epsilon^2 = 0.
#[derive(Debug, Clone, Copy, PartialEq,  PartialOrd)]
struct DualNumber {
    a: f64,
    b: f64
}

impl DualNumber {
    pub fn new(a: f64, b: f64) -> Self {
        DualNumber { a, b }
    }

    pub fn exp(self, n: i64) -> Self {
        let mut x = self;
        for i in 1..n {
            x *= self;
        }
        x
    }
}

impl std::ops::MulAssign for DualNumber {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Add for DualNumber {
    type Output = DualNumber;

    fn add(self, rhs: Self) -> Self::Output {
        DualNumber {
            a: self.a + rhs.a,
            b: self.b + rhs.b
        }
    }
}

impl std::ops::Mul for DualNumber {
    type Output = DualNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        //(a+bE)(c+dE) = ac + bdE^2 + adE + bcE = ac + (ad + bc)E
        DualNumber {
            a: self.a * rhs.a,
            b: (self.a * rhs.b) + (self.b * rhs.a)
        }
    }
}

impl std::ops::Sub for DualNumber {
    type Output = DualNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        DualNumber {
            a: self.a - rhs.a,
            b: self.b - rhs.b
        }
    }
}

impl std::ops::Div for DualNumber {
    type Output = DualNumber;

    fn div(self, rhs: Self) -> Self::Output {
        DualNumber {
            a: self.a / rhs.a,
            b: (self.b * rhs.a - self.a * rhs.b) / (rhs.a * rhs.a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = DualNumber::new(2., 0.);
        let b = DualNumber::new(3., 0.);
        let c = DualNumber::new(4., 1.);
        //dc = 1 + 6c^5 = 6145 
        let result = a*b + c + c.exp(6); //4106
        assert_eq!(result, DualNumber::new(4106., 6145.));
    }
}
