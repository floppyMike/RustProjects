use std::env;

macro_rules! arg_to_int {
    ($s:expr, $t:ty) => {
        match $s.parse::<$t>() {
            Ok(v) => v,
            Err(_) => {
                println!("Argument \"{}\" not recognized.", $s);
                return;
            }
        };
    };
}

fn main() {
    let mut args = env::args();

    if args.len() != 4 {
        println!(
            "{} <loan> <interest> <n of months>\nCalculate fixed loan you pay for n of months.",
            args.nth(0).unwrap()
        );
        return;
    }

    args.next(); // Skip location arg

    let loan = arg_to_int!(args.next().unwrap(), f64);
    let interest = arg_to_int!(args.next().unwrap(), f64);
    let n = arg_to_int!(args.next().unwrap(), u16);

    println!(
        "{:.2}€",
        loan * interest * (1f64 + interest).powi(n as i32)
            / ((1f64 + interest).powi(n as i32) - 1f64)
    );
}
