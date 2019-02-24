use rand::Rng;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut n: u32;
    if args.len() > 1 {
        let input = args[1].trim();
        let parse_try = input.parse();
        n = match parse_try {
            Ok(num) => num,
            Err(_)  => {
                return;
            },
        };
    } else {
        n = rand::thread_rng().gen_range(100, 1000);
    }
}
