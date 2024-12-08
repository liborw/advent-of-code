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


pub fn digits<T>(num: T) -> impl Iterator<Item = u8>
where
    T: PrimInt + Unsigned,
{
    let mut num = num;
    let mut d: Vec<u8> = Vec::new();

    while num > T::zero() {
        let digit = (num % T::from(10).unwrap()).to_u8().unwrap();
        num = num / T::from(10).unwrap();
        d.push(digit)
    }

    d.into_iter().rev()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits() {
        let mut d = digits(1234u32);
        assert_eq!(d.next(), Some(1));
        assert_eq!(d.next(), Some(2));
        assert_eq!(d.next(), Some(3));
        assert_eq!(d.next(), Some(4));
        assert_eq!(d.next(), None);
    }

}
