use std::slice;
use std::str;

fn main() {
    const PI: &'static str = "3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706";

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("pi_to_nth_digit <num>\nShow decimals of a string till 100.");
        return;
    }

    let num = match usize::from_str_radix(&args[1], 10) {
        Ok(n) => n,
        Err(_) => {
            println!("Unrecognized number.");
            return;
        }
    };

    if num > PI.len() {
        println!("Decimal is up to 100.");
        return;
    }

    let slice = unsafe { str::from_utf8(slice::from_raw_parts(PI.as_ptr(), num)).unwrap() };

    println!("{}", slice);
}
