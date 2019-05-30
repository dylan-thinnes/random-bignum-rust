use rand::Rng;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut n: u32;
    let mut base: u64 = 10;
    if args.len() == 0 {
        eprintln!("No digits specified. Generating random number of digits between 100 and 1000...");
        n = rand::thread_rng().gen_range(100, 1000);
        eprintln!("{} chosen as number of digits.", n);
    } else {
        if args.len() > 2 {
            let input = args[2].trim();
            let parse_try = input.parse();
            base = match parse_try {
                Ok(num) => num,
                Err(_)  => {
                    eprintln!("Could not parse a base from {}. Please input a number!", input);
                    return;
                },
            };
        }

        let input = args[1].trim();
        let parse_try = input.parse();
        n = match parse_try {
            Ok(num) => num,
            Err(_)  => {
                eprintln!("Could not parse number of digits from {}. Please input a number!", input);
                return;
            },
        };
    }

    let mut r: u64;
    let mut gen = rand::thread_rng();
    loop {
        if n >= 19 {
            r = gen.gen_range(1, 10u64.pow(19));
            print!("{:019}", r);
            
            n -= 19;
        } else {
            loop {
                if n == 0 { break; }
                r = gen.gen_range(0, 10);
                print!("{}", r);

                n -= 1;
            }
            break;
        }
    }
    println!("")
}
