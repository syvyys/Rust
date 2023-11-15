use std::fmt::Debug;
// task 2
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, other: Self) -> Self::Output {
        return Complex {
            re: self.re + other.re,
            im: self.im + other.im
        };
    }
}

impl<T: Sub<Output = T>> Sub for Complex<T> {
    type Output = Complex<T>;

    fn sub(self, other: Self) -> Self::Output {
        return Complex {
            re: self.re - other.re,
            im: self.im - other.im
        };
    }
}

impl<T: Mul<Output = T> +
        Sub<Output = T> +
        Add<Output = T> + Copy> Mul for Complex<T> {
    type Output = Complex<T>;

    fn mul(self, other: Self) -> Self::Output {
        return Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re
        };
    }
}

impl<T: Div<Output = T> +
        Sub<Output = T> +
        Mul<Output = T> +
        Add<Output = T> + Copy> Div for Complex<T> {
    type Output = Complex<T>;

    fn div(self, other: Self) -> Self::Output {
        let square = other.re * other.re + other.im + other.im; // TODO: check for zero
        return Complex {
            re: (self.re + other.re + self.im * other.im) / square,
            im: (self.im * other.re - self.re * other.im) / square
        }
    }
}

impl<T: AddAssign<T>> AddAssign for Complex<T> {
    fn add_assign(&mut self, other: Self) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl<T: SubAssign<T> + Copy> SubAssign for Complex<T> {
    fn sub_assign(&mut self, other: Self) {
        self.re -= other.re;
        self.im -= other.im;
    }
}

impl<T: Mul<Output = T> +
        Sub<Output = T> +
        Add<Output = T> + Copy> MulAssign for Complex<T> {
    fn mul_assign(&mut self, other: Self) {
        let (re, im) = (self.re, self.im);
        self.re = re * other.re - im * other.im;
        self.im = re * other.im + im * other.re;
    }
}

impl<T: Div<Output = T> +
        Mul<Output = T> +
        Sub<Output = T> +
        Add<Output = T> + Copy + Debug> DivAssign for Complex<T> {
    fn div_assign(&mut self, other: Self) {
        let (re, im) = (self.re, self.im);
        let square = other.re * other.re + other.im * other.im; // TODO: check for zero
        self.re = (re * other.re + im * other.im) / square;
        self.im = (im * other.re - re * other.im) / square;
    }
}

#[test]
fn test_operations() {
    let val1: Complex<i32> = Complex {re: 0, im: 0};
    let val2: Complex<i32> = Complex {re: 5, im: 1};

    assert_eq!(val2 + val1, val2);
    assert_eq!(val2 - val1, val2);
    assert_eq!(val1 * val2, val1);
    assert_eq!(val1 / val2, val1);
}

#[test]
fn test_assign_operations() {
    let mut val1: Complex<i32> = Complex {re: 1, im: 1};
    val1 += Complex {re: 1, im: 1};
    assert_eq!(val1, Complex {re: 2, im: 2});

    val1 -= Complex {re: 1, im: 1};
    assert_eq!(val1, Complex {re: 1, im: 1});

    val1 *= Complex {re: 5, im: 10};
    assert_eq!(val1, Complex {re: -5, im: 15});

    val1 /= Complex {re: 2, im: -1};
    assert_eq!(val1, Complex {re: -5, im: 5});
}

fn main() {

}
