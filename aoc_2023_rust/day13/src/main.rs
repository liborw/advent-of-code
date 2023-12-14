use std::str::from_utf8;

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



fn find_simetry(img: &Image) -> Option<usize> {

    for l in img.data.iter() {
        println!("{}", from_utf8(l).unwrap());
    }




    (0..(img.size.1 - 2)).into_iter().find(|&col| {
        (0..img.size.0).all(|row| {
            // distance to neerest edge
            let len = col.min(img.size.1 - col - 1);
            (0..=len).all(|e| {
                println!("V{row}: {}-{}-{}   (e = {e}, len = {len})", col - e, col , col + e + 1 );
                let v = img.data[row][col - e] == img.data[row][col + e + 1];
                println!("V{row}: {}-{}-{} => {v}   (e = {e}, len = {len})", col - e, col , col + e + 1 );
                v
            })
        })
    }).map(|x| x + 1).or_else(|| {
        (0..(img.size.0 - 2)).into_iter().find(|&row| {
            (0..img.size.1).all(|col| {
                // distance to neerest edge
                let len = row.min(img.size.0 - row - 2);
                //println!("l: {l}, i: {i}, edge: {edge}");
                (0..=len).all(|e| {
                    let v = img.data[row - e][col] == img.data[row + e + 1][col];
                    println!("H{col}: {}-{}-{} => {v}   (e = {e}, len = {len})", row - e, row , row + e + 1);
                    v
                })
            })
        }).map(|x| (x + 1) * 100)
    })
}

fn parse(input: &str) -> Vec<Image> {
    input.split("\n\n").map(|blk| {
        let data: Vec<&[u8]> = blk.lines().map(|l| l.as_bytes()).collect();
        let size = (data.len(), data[0].len());
        Image{data, size}
    }).collect()
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().enumerate().take(7).filter_map(|(i, img)| {
        let u = find_simetry(&img);
        println!("{i}: {u:?}");
        u
    }).sum()
}

fn part2(input: &str) -> usize {
    1
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
        assert_eq!(part1(input), 1);
    }

    // #[test]
    // fn part2_test() {
    //     let input = include_str!("../input_test.txt");
    //     assert_eq!(part2(input), 1);
    // }

    // #[test]
    // fn part2_final_test() {
    //     let input = include_str!("../input.txt");
    //     assert_eq!(part2(input), 1);
    // }
}
