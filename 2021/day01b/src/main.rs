fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u16>>()
            .windows(3)
            .map(|w| w[0] + w[1] + w[2])
            .collect::<Vec<u16>>()
            .windows(2)
            .filter(|w| w[0] < w[1])
            .count()
    )
}
