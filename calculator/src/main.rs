extern crate mexprp;
use std::env;

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        println!(
            "{} <expression>\nCalculate math expression.",
            args.nth(0).unwrap()
        );
        return;
    }

    args.next(); // Skip location arg

    let result = match mexprp::eval::<f64>(args.next().unwrap().as_str()) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", &e);
            return;
        }
    };

    println!("{}", result);
}
