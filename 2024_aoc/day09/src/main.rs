use std::{collections::VecDeque, fmt::{Debug, Display}};

use utils::{took, run_task};

fn main() {
    let input = include_str!("../input_test.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}


#[derive(Clone, Copy)]
enum Dense {
    Data{start: usize, len: usize, id: usize},
    Free{start: usize, len: usize}
}

impl Dense {

    fn is_data(&self) -> bool {
        use Dense::*;
        match self {
            Data { .. } => true,
            Free { .. } => false
        }
    }

    fn len(&self) -> usize {
        use Dense::*;
        match self {
            Data { len, .. } => *len,
            Free { len, .. } => *len
        }
    }


    fn start(&self) -> usize {
        use Dense::*;
        match self {
            Data { start, .. } => *start,
            Free { start, .. } => *start
        }
    }

    fn end(&self) -> usize {
        use Dense::*;
        match self {
            Data { start, len, .. } => start + len - 1,
            Free { start, len } => start + len -1
        }
    }

    fn id(&self) -> Option<usize> {

        use Dense::*;
        match self {
            Data { id, .. } => Some(*id),
            Free { .. } => None
        }
    }

    fn free(&self) -> Self {
        Dense::Free { start: self.start(), len: self.len() }
    }
}

impl Debug for Dense {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dense::Data { id, .. } => write!(f, "D({}, {}, {})", self.start(), self.end(), id)?,
            Dense::Free { .. } => write!(f, "F({}, {})", self.start(), self.end())?,
        }
        Ok(())
    }
}

fn print_disc(disc: &[Dense]) {



    for d in disc.iter() {
        for _ in 0..d.len() {
            print!("{}", d.id().map_or(".".into(), |d| format!("{d}")));
        }
    }
    println!();

}

fn parse(input: &str) -> Vec<Dense> {

    let mut id = 0;
    let mut pos = 0;
    let mut is_data = true;

    input.strip_suffix("\n").unwrap().chars().filter_map(|c| {
        let len = c.to_digit(10).unwrap() as usize;

        let des = if is_data {
            Some(Dense::Data { start: pos, len, id })
        } else if len == 0 {
            None
        } else {
            Some(Dense::Free { start: pos, len })
        };

        if is_data {
            id += 1;
        }

        is_data = !is_data;
        pos += len;

        des
    }).collect()

}



fn part1(input: &str) -> usize {
    use Dense::*;
    let mut dense = VecDeque::from(parse(input));
    let mut i = 0;
    let mut j = dense.back().unwrap().end();
    let mut front = dense.pop_front();
    let mut back = dense.pop_back();
    let mut checksum = 0;


    loop {
        if front.is_some() && i > front.unwrap().end() {
            front = dense.pop_front();
        }

        if front.is_none() {
            front = back;
            back = None;
        }

        if i > j {
            println!("pointers meet");
            break
        }

        if front.is_none() {
            println!("front is none");
            break
        }

        if back.is_some() && j < back.unwrap().start() {
            back = dense.pop_back();
        }

        if let Some(Data { id, .. }) = front {
            checksum += i * id;
            i += 1;
        } else if let Some(Free {..}) = back {
            j -= 1;
        } else if back.is_none() {
            i += 1;
        } else if let Some(Free {..}) = front {
            let id = back.unwrap().id().unwrap();
            checksum += i * id;
            i += 1;
            j -= 1;
        }
    }
    checksum
}

fn part2(input: &str) -> usize {
    let mut dense = parse(input);
    let mut last_id = dense.iter().filter_map(|d| d.id()).max().unwrap();

    while let Some(cur_i) = dense.iter().enumerate().filter_map(|(i, d)| {
        if d.is_data() && d.id().unwrap() <= last_id {
            Some(i)
        } else {
            None
        }
    }).last() {
        let cur = dense[cur_i];
        if let Some(free_i) = dense.iter().enumerate().filter_map(|(i, d)| {
            if !d.is_data() && d.len() >= cur.len() && d.start() < cur.start() {
                Some(i)
            } else {
                None
            }
        }).next() {
            // Remove current from dense
            let cur = dense.remove(cur_i);
            let cur_free = cur.free();

            dense.insert(cur_i, cur_free);

            // remove free
            let free = dense.remove(free_i);

            let cur = Dense::Data {
                start: free.start(),
                len: cur.len(),
                id: cur.id().unwrap()
            };

            // inser cur to its place
            dense.insert(free_i, cur);

            let free = Dense::Free {
                start: free.start() + cur.len(),
                len: free.len() - cur.len()
            };

            if free.len() > 0 {
                dense.insert(free_i + 1, free);
            }
        }

        if last_id == 0 {
            break
        } else {
            last_id -= 1;
        }
    }


    let mut checksum = 0;
    let mut i = 0;
    for d in dense {
        while i <= d.end() {
            if d.is_data() {
                let id = d.id().unwrap();
                checksum += i * id;
            }
            i += 1;
        }
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 1928);
    }

    #[test]
    fn day09_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 6340197768906);
    }

    #[test]
    fn day09_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 2858);
    }

    #[test]
    fn day09_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 6363913128533);
    }
}
