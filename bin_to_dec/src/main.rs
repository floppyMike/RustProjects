use std::env;

fn main() {
    let mut args = env::args();

    if args.len() != 3 {
        println!(
            "{} <-b or -d> <bin or dec>\nCalculate decimal to binary or otherway around.",
            args.nth(0).unwrap()
        );
        return;
    }

    args.next(); // Skip location arg

    match args.next().unwrap().as_str() {
        "-b" => println!(
            "{}",
            match isize::from_str_radix(args.next().unwrap().as_str(), 2) {
                Ok(n) => n,
                Err(_) => {
                    println!("Unrecognized binary.");
                    return;
                }
            }
        ),
        "-d" => println!(
            "{:b}",
            match isize::from_str_radix(args.next().unwrap().as_str(), 10) {
                Ok(n) => n,
                Err(_) => {
                    println!("Unrecognized decimal.");
                    return;
                }
            }
        ),
        s => {
            println!("Unrecognized option \"{}\".", s);
            return;
        }
    };
}
