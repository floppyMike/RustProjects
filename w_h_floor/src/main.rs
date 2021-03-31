use std::env;

fn main() {
    let mut args = env::args();

    if args.len() != 4 {
        println!(
            "{} <width> <height> <cost>\nCalculate tile costs base on width and height in meters and cost per meter in euro.",
            args.nth(0).unwrap()
        );
        return;
    }

    args.next(); // Skip location iter

    let num = {
        let mut n: [f64; 3] = [0f64; 3];

        for (i, (num, arg)) in n.iter_mut().zip(args).enumerate() {
            *num = match arg.parse::<f64>() {
                Ok(v) => v,
                Err(_) => {
                    println!("Argument #{} doesn't have a number.", i);
                    return;
                }
            }
        }

        n
    };

    println!("{:.2}€", num[0] * num[1] * num[2]);
}
