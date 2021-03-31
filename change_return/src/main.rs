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

    if args.len() != 2 {
        println!(
            "{} <change>\nCalculate what change to return.",
            args.nth(0).unwrap()
        );
        return;
    }

    args.next(); // Skip location arg

    let mut num = arg_to_int!(args.next().unwrap(), f64);

    let money = [
        100f64, 50f64, 20f64, 10f64, 5f64, 2f64, 1f64, 0.5f64, 0.2f64, 0.1f64, 0.05f64, 0.01f64,
    ];

    for m in money.iter() {
        let u = (num / *m) as u64;
        num = (num - u as f64 * *m) + 0.00000000001f64; // Handle imperfect presision. Doesn't work if over 10⁹ steps taken.

        print!("{}€: {}\n", *m, u);
    }
}
