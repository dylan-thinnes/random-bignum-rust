use rand::Rng;
use std::env;

fn parse_input(args: Vec<String>) -> (u32, u64) {
    let n: u32;
    let mut base: u64 = 10;

    if args.len() <= 1 {
        eprintln!("No digits specified. Generating random number of digits between 100 and 1000...");
        n = rand::thread_rng().gen_range(100, 1000);
        eprintln!("{} chosen as number of digits.", n);
    } else {
        if args.len() > 2 {
            let input = args[2].trim();
            base = input.parse().expect("Could not parse a base from the second argument.\
                                         Please input a number!");
        }

        let input = args[1].trim();
        n = input.parse()
            .expect("Could not parse number of digits from {}.\
                     Please input a number!");
    }

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
