fn main() {
   let (_, h ,v) = include_str!("../input.txt")
                    .lines()
                    .map(|l| l.split_once(" ").unwrap())
                    .fold((0, 0, 0), |(a, h, v), (t, k)| {
                        match (t, k.parse::<i32>().unwrap() ){
                            ("forward", k) => (a, h + a * k, v + k),
                            ("down", k) => (a + k, h, v),
                            ("up", k) => (a - k, h, v),
                            _ => unreachable!()
                        }
                    });

   println!("{}", h * v);

}
