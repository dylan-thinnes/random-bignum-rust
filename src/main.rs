use rand::Rng;
use std::env;

fn parse_input(args: Vec<String>) -> (u32, u64) {
    let n: u32;
    let base: u64;

    n = match args.get(1) {
        Some(s) => s.trim().parse()
                    .expect("Could not parse number of digits from {}.\
                             Please input a number!"),
        None => {
            eprintln!("No digits specified. Generating random number of digits between 100 and 1000...");
            let random = rand::thread_rng().gen_range(100, 1000);
            eprintln!("{} chosen as number of digits.", random);
            random
        }
    };

    base = match args.get(2) {
        Some(s) => s.trim().parse()
                    .expect("Could not parse a base from the second argument.\
                             Please input a number!"),
        None => 10
    };

    return (n, base);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (mut n, base) = parse_input(args);

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
