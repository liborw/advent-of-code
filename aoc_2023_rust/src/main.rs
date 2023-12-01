
pub trait Remainders {
    fn remainders(&self) -> RemaindersIter;
    fn remainders_rev(&self) -> RemaindersIter;
}

pub struct RemaindersIter<'a> {
    data: &'a str,
    index: usize,
    reverse: bool
}

impl<'a> Iterator for RemaindersIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.len() {
            return None
        }

        let i = self.index;
        self.index += 1;

        if self.reverse {
            Some(&self.data[(self.data.len() - i - 1)..])
        } else {
            Some(&self.data[i..])
        }
    }
}

impl Remainders for &str {
    fn remainders(&self) -> RemaindersIter {
        RemaindersIter{
            data: self,
            index: 0,
            reverse: false
        }
    }

    fn remainders_rev(&self) -> RemaindersIter {
        RemaindersIter{
            data: self,
            index: 0,
            reverse: true
        }
    }
}

fn main() {

    let result: i32 = include_str!("./input/input_01.txt").lines().into_iter()
                    .map(|l| read_line_a(l))
                    .map(|(first, last)| first * 10 + last)
                    .sum();

    println!("{result:?}");


    let result: i32 = include_str!("./input/input_01.txt").lines().into_iter()
                    .map(|l| read_line_b(l))
                    .map(|(first, last)| first * 10 + last)
                    .sum();


    println!("{result:?}");
}

fn parse_first_digit(s: &str) -> Option<i32> {
    if let Ok(v) = s[0..1].parse::<i32>() {
        Some(v)
    } else {
        if s.starts_with("one") {
            Some(1)
        } else if s.starts_with("two") {
            Some(2)
        } else if s.starts_with("three") {
            Some(3)
        } else if s.starts_with("four") {
            Some(4)
        } else if s.starts_with("five") {
            Some(5)
        } else if s.starts_with("six") {
            Some(6)
        } else if s.starts_with("seven") {
            Some(7)
        } else if s.starts_with("eight") {
            Some(8)
        } else if s.starts_with("nine") {
            Some(9)
        } else {
            None
        }
    }
}

fn read_line_a(s: &str) -> (i32, i32) {
    let chars: Vec<_> = s.chars().collect();
    let first = chars.iter().find_map(|c| c.to_string().parse().ok());
    let last = chars.iter().rev().find_map(|c| c.to_string().parse().ok());
    (first.unwrap(), last.unwrap())
}

fn read_line_b(s: &str) -> (i32, i32) {
    let first = s.remainders().find_map(|s| parse_first_digit(s));
    let last = s.remainders_rev().find_map(|s| parse_first_digit(s));
    (first.unwrap(), last.unwrap())
}

