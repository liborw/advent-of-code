fn main() {
   let (h ,v) = include_str!("../input.txt")
                    .lines()
                    .map(|l| l.split_once(" ").unwrap())
                    .fold((0, 0), |(h, v), (t, k)| {
                        match (t, k.parse::<i32>().unwrap() ){
                            ("forward", k) => (h, v + k),
                            ("down", k) => (h + k, v),
                            ("up", k) => (h - k, v),
                            _ => unreachable!()
                        }
                    });

   println!("{}", h * v);

}
