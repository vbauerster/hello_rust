use gcd::*;

fn main() {
    let numbers: Vec<_> = std::env::args()
        .skip(1)
        // use std::str::FromStr;
        // .filter(|arg| u64::from_str(arg).is_ok())
        .filter_map(|arg| arg.parse::<u64>().ok())
        .collect();

    // println!("{:?}", numbers);

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
