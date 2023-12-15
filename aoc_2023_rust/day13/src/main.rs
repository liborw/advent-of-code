use std::{str::from_utf8, fmt::{Display, self}};
use took::took;

macro_rules! aoc_task {
    ($f:expr) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

struct Image<'a> {
    data: Vec<&'a [u8]>,
    size: (usize, usize)
}

impl<'a> Image<'a> {
    fn new(lines: Vec<&[u8]>) -> Image {
        Image{
            data: lines.clone(),
            size: (lines.len(), lines[0].len())
        }
    }
}

impl Display for Image<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        self.data.iter().for_each(|l| {
            s.push_str(from_utf8(l).unwrap());
            s.push('\n');
        });
        write!(f, "{}", s)
    }
}


fn simetry_error(img: &Image) -> Vec<(usize, usize)> {
    let mut err = Vec::new();

    (0..(img.size.1 - 1)).into_iter().for_each(|col| {
        let e = (0..img.size.0).map(|row| {
            // distance to neerest edge
            let len = col.min(img.size.1 - col - 2);
            (0..=len).map(|e| {
                let left = col - e;
                let right = col + e + 1;
                if img.data[row][left] == img.data[row][right] {
                    0
                } else {
                    1
                }
            }).sum::<usize>()
        }).sum();
        err.push((e, col + 1));
    });

    (0..(img.size.0 - 1)).into_iter().for_each(|row| {
        let e = (0..img.size.1).map(|col| {
            // distance to neerest edge
            let len = row.min(img.size.0 - row - 2);
            //println!("l: {l}, i: {i}, edge: {edge}");
            (0..=len).map(|e| {
                let left = row - e;
                let right = row + e + 1;
                if img.data[left][col] == img.data[right][col] {
                    0
                } else {
                    1
                }
            }).sum::<usize>()
        }).sum();
        err.push((e, (row + 1) * 100));
    });

    err
}



fn parse(input: &str) -> Vec<Image> {
    input.split("\n\n").map(|blk| {
        Image::new(blk.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>())
    }).collect()
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().map(|img| {
        simetry_error(&img).iter().find(|&(e, _)| *e == 0).unwrap().1
    }).sum()
}

fn part2(input: &str) -> usize {
    parse(input).into_iter().map(|img| {
        simetry_error(&img).iter().find(|&(e, _)| *e == 1).unwrap().1
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 405);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 27300);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 400);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 29276);
    }
}
