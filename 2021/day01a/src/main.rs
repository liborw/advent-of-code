
fn main() {

    println!(
        "{}",
        include_str!("../input_a.txt")
            .lines()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i16>>()
            .windows(2)
            .filter(|w| w[0] < w[1])
            .count()
    )

}
