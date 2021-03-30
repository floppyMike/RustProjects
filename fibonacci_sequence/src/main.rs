use std::env;

fn fib(n: u32) -> u128 { // Inefficient... but easy to read!
    if n < 2 {
        return n as u128;
    }

    fib(n - 1) + fib(n - 2)
}

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        println!(
            "{} <till>\nPrints the fibonacci sequence till a number.",
            args.nth(0).unwrap()
        );
        return;
    }

    let num = match args.nth(1).unwrap().parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Number not recognized.");
            return;
        }
    };

    println!("{}", fib(num));
}
