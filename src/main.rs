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

    let mut r: u64;
    loop {
        if n >= 19 {
            r = rand::thread_rng().gen_range(1, 10u64.pow(19));
            print!("{:019}", r);
            
            n -= 19;
        } else {
            loop {
                if n == 0 { break; }
                r = rand::thread_rng().gen_range(1, 10);
                print!("{}", r);

                n -= 1;
            }
            break;
        }
    }
    println!("")
}
