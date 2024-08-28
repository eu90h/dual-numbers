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

    pub fn powi(self, n: i64) -> Self {
        let mut x = self;
        for _ in 1..n {
            x *= self;
        }
        x
    }

    pub fn log(self, base: f64) -> Self {
        DualNumber::new(self.a.log(base), 1.0/self.a * self.b)
    }

    pub fn exp(self) -> Self {
        DualNumber::new(self.a.exp(), self.a.exp() * self.b)
    }

    pub fn sin(self) -> Self {
        DualNumber::new(self.a.sin(), self.a.cos() * self.b)
    }

    pub fn cos(self) -> Self {
        DualNumber::new(self.a.cos(), self.a.sin() * self.b)
    }
}

impl std::ops::MulAssign for DualNumber {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::AddAssign for DualNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for DualNumber {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::DivAssign for DualNumber {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
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
        let result = a*b + c + c.powi(6); //4106
        assert_eq!(result, DualNumber::new(4106., 6145.));
    }

    #[test]
    fn log_and_sin() {
        let a = DualNumber::new(2., 1.);
        let b = DualNumber::new(5., 0.);
        let y = a.log(1.0_f64.exp()) + a * b - b.sin();
        assert!((y.a - 11.652) < 0.0001);
        assert!(y.b - 5.5 < 0.0001)
    }
}
