
fn main() {
    println!("run it as cargo test");
}


fn solve(vec: &mut [usize], target: usize) -> usize {
    vec.reverse();
    solve_core(vec, target)
}

fn solve_core(vec: &[usize], value: usize) -> usize {

    match (vec.is_empty(), value) {
        (true, 0) => 1,
        (false, 0) => 0,
        (true, _) => 0,
        (false, a) => {
            let b = vec[0];

            let mut v = 0;
            if let Some(new_v) = a.checked_sub(b) {
                v += solve_core(&vec[1..], new_v)
            };

            if a % b == 0 {
                v += solve_core(&vec[1..], a / b);
            };

            v
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let res = solve(&mut [2, 5, 2, 2, 2, 5, 5, 2], 187);
        assert_eq!(res, 2);
    }

    #[test]
    fn solve_final_test() {

        let mut input = [7, 3, 4, 3, 5, 3, 3, 7, 4, 5, 3, 4, 4, 2, 3, 6, 2, 3, 5, 2, 4, 5, 6, 2, 2,
        4, 5, 4, 4, 3, 4, 5, 5, 4, 3, 5, 3, 3, 2, 3, 2, 3, 4, 5, 2, 4, 3, 3, 6, 4,
        2, 3, 4, 7, 2, 3, 5, 4, 4, 6];
        let res = solve(&mut input, 5632358);
        assert_eq!(res, 21765);
    }

}
