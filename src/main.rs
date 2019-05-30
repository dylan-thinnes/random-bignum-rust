use rand::Rng;
use std::env;

fn parse_input(args: Vec<String>) -> (u32, u64) {
    let n: u32 = match args.get(0) {
        Some(s) => s.trim().parse()
                    .expect("Could not parse number of digits from {}.\
                             Please input a number!"),
        None => {
            eprintln!("No digits specified. Defaulting to 100.");
            100
        }
    };

    let base: u64 = match args.get(1) {
        Some(s) => s.trim().parse()
                    .expect("Could not parse a base from the second argument.\
                             Please input a number!"),
        None => {
            eprintln!("No base specified. Defaulting to 10.");
            10
        }
    };

    return (n, base);
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
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
