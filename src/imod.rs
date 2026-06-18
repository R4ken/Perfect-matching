use core::fmt;
use std::ops;

const M: i64 = 1e9 as i64 + 7;

#[derive(Clone, Copy, Debug)]
pub struct Imod {
    pub value: i64
}

impl Imod {

    pub fn inv(&self) -> Imod {
        let mut inv: i64 = 1;
        let mut pow= M - 2;
        let mut a = self.value;
        while pow > 0{
            if pow & 1 == 1 {
                inv *= a;
                inv %= M;
            }
            pow >>= 1;
            a *= a;
            a %= M;
        }
        Imod { value: inv }
    }
}

impl ops::Add for Imod {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Imod {value: (self.value + rhs.value) % M}
    }
}

impl ops::AddAssign for Imod{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        self.value %= M;
    }
}

impl ops::Sub for Imod {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Imod {value: (self.value + M - rhs.value) % M}
    }
}

impl ops::SubAssign for Imod {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = self.value + M - rhs.value;
        self.value %= M;
    }
}

impl ops::Mul for Imod {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Imod {value: (self.value * rhs.value + M) % M}
    }
}

impl ops::Div for Imod {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        debug_assert!(rhs.value != 0);
        self * rhs.inv()
    }
}

impl From<i32> for Imod {
    fn from(value: i32) -> Self {
        Imod { value: value as i64 }
    }
}

impl PartialEq for Imod {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl fmt::Display for Imod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Addition Tests
    
    #[test]
    fn test_add_normal() {
        assert_eq!(Imod::from(5) + Imod::from(10), Imod::from(15));
    }

    #[test]
    fn test_add_overflow() {
        // (M - 1) + 5 = 4
        let a = Imod::from((M - 1) as i32);
        let b = Imod::from(5);
        assert_eq!((a + b).value, 4);
    }

    #[test]
    fn test_add_assign_overflow() {
        let mut a = Imod::from((M - 2) as i32);
        a += Imod::from(3);
        assert_eq!(a.value, 1);
    }

    // Subtraction Tests

    #[test]
    fn test_sub_normal() {
        assert_eq!(Imod::from(10) - Imod::from(5), Imod::from(5));
    }

    #[test]
    fn test_sub_underflow() {
        // 3 - 5 = M - 2
        let a = Imod::from(3);
        let b = Imod::from(5);
        assert_eq!((a - b).value, M - 2);
    }

    #[test]
    fn test_sub_assign_underflow() {
        let mut a = Imod::from(10);
        a -= Imod::from(20);
        // 10 - 20 = -10 -> normalizes to M - 10
        assert_eq!(a.value, M - 10);
    }

    // Multiplication Tests

    #[test]
    fn test_mul_normal() {
        assert_eq!(Imod::from(3) * Imod::from(4), Imod::from(12));
    }

    #[test]
    fn test_mul_overflow() {
        // 1,000,000,000 * 2 = 2,000,000,000 
        // 2,000,000,000 % 1,000,000,007 = 999,999,993
        let a = Imod::from(1_000_000_000);
        let b = Imod::from(2);
        assert_eq!((a * b).value, 999_999_993);
    }

    #[test]
    fn test_mul_heavy_overflow() {
        // (M - 1) * (M - 1) should equal 1
        // This also verifies that your i64 doesn't overflow the primitive bounds
        // before the modulo is applied. (i64 max is ~9e18, M^2 is ~1e18, so it's safe).
        let a = Imod::from((M - 1) as i32);
        let b = Imod::from((M - 1) as i32);
        assert_eq!((a * b).value, 1);
    }

    // Inverse and Division Tests

    #[test]
    fn test_inv() {
        // Fermat's Little Theorem: a * a^(M-2) % M == 1
        let a = Imod::from(2);
        let inv_a = a.inv();
        assert_eq!((a * inv_a).value, 1);

        let b = Imod::from(123456789);
        let inv_b = b.inv();
        assert_eq!((b * inv_b).value, 1);
    }

    #[test]
    fn test_div() {
        // 10 / 2 = 5
        let a = Imod::from(10);
        let b = Imod::from(2);
        assert_eq!((a / b).value, 5);

        // Division with modulo logic: (1 / 2) mod M = (M + 1) / 2
        let one = Imod::from(1);
        let two = Imod::from(2);
        // M = 1_000_000_007, so (M+1)/2 = 500_000_004
        assert_eq!((one / two).value, 500_000_004);
    }

    // Type Conversion and Equality Tests

    #[test]
    fn test_from_i32() {
        let a: Imod = 42.into();
        assert_eq!(a.value, 42);
    }

    #[test]
    fn test_equality() {
        assert_eq!(Imod::from(100), Imod::from(100));
        assert_ne!(Imod::from(100), Imod::from(101));
    }
}