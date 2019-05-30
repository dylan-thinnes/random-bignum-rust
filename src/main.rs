use rand::Rng;
use std::env;
use std::iter::Extend;

fn parse_input(args: Vec<String>) -> Result<(u32, u64), String> {
    let n: u32 = match args.get(0) {
        Some(s) => s.trim().parse()
                    .or(Err("Could not parse number of digits from first argument. \
                             Please input a number!"))?,
        None => {
            eprintln!("No digits specified. Defaulting to 100.");
            100
        }
    };

    let base: u64 = match args.get(1) {
        Some(s) => s.trim().parse()
                    .or(Err("Could not parse a base from the second argument. \
                             Please input a number!"))?,
        None => {
            eprintln!("No base specified. Defaulting to 10.");
            10
        }
    };

    return Ok((n, base));
}

fn base_table (n: u64) -> Result<Vec<char>,String> {
    if n > 64 { 
        Err("Base >= 64! Please specify a base between 2 and 64.")?;
    } else if n < 2  { 
        Err("Base <= 2! Please specify a base between 2 and 64.")?;
    }
    let mut table: Vec<u8> = vec![];
    if n > 36 {
        table.extend(b'A'..=b'Z');
        table.extend(b'a'..=b'z');
        table.extend(b'0'..=b'9');
        table.push(b'+');
        table.push(b'/');
    } else {
        table.extend(b'0'..=b'9');
        table.extend(b'a'..=b'z');
    }

    return Ok(table.iter().map(|&x| char::from(x)).collect());
}

fn run() -> Result<(), String> {
    let args: Vec<_> = env::args().skip(1).collect();
    let (mut n, base) = parse_input(args)?;
    let table = base_table(base)?;

    let mut r: u64;
    let mut gen = rand::thread_rng();

    if base == 10 {
        while n >= 19 {
            r = gen.gen_range(1, 10u64.pow(19));
            print!("{:019}", r);
            
            n -= 19;
        }
    }

    while n > 0 {
        r = gen.gen_range(0, base);
        print!("{}", table[r as usize]);

        n -= 1;
    }

    println!("");
    return Ok(());
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(s) => eprintln!("{}", s)
    }
}
