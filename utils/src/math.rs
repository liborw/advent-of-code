use num::{Num, PrimInt, Unsigned};

pub fn gcd<T: Num + Copy>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

pub fn lcm<T: Num + Copy>(a: T, b: T) -> T {
    a * b / gcd(a, b)
}


pub fn digits<T>(mut num: T) -> impl Iterator<Item = u8>
where
    T: PrimInt + Unsigned,
{
    std::iter::from_fn(move || {
        if num == T::zero() {
            None
        } else {
            let digit = (num % T::from(10).unwrap()).to_u8().unwrap();
            num = num / T::from(10).unwrap();
            Some(digit)
        }
    })
}

pub enum Op {
    /// Addition
    Add,
    /// Multiplication
    Mul,
    /// Substraction
    Sub,
    /// Division
    Div,
    /// Modulo
    Mod,
    // Cancatination ie 123 con 456 123456
    Con,
}


impl Op {
    pub fn apply<T: Num + Copy>(&self, lhs: T, rhs: T) -> T {
        match *self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
            Op::Sub => lhs - rhs,
            Op::Div => lhs / rhs,
            Op::Mod => lhs % rhs,
            Op::Con => panic!("Con not implemented"),
        }
    }
}
