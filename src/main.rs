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
                eprintln!("Could not parse number of digits from {}. Please input a number!", input);
                return;
            },
        };
    } else {
        eprintln!("No digits specified. Generating random number of digits between 100 and 1000...");
        n = rand::thread_rng().gen_range(100, 1000);
        eprintln!("{} chosen as number of digits.", n);
    }
}
