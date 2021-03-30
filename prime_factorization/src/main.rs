use std::env;

fn main() {
    let mut args = env::args();

    if args.len() != 2 {
        println!(
            "{} <num>\nPrints the prime factors of a number.",
            args.nth(0).unwrap()
        );
        return;
    }

    let mut num = match args.nth(1).unwrap().parse::<u128>() {
        Ok(n) => n,
        Err(_) => {
            println!("Number not recognized.");
            return;
        }
    };

    let mut res = match num {
        1 => vec![1],
        _ => vec![],
    };

    while num > 1 {
        for i in 2..=num {
            if num % i == 0 {
                res.push(i);
                num /= i;
                break;
            }
        }
    }

    for x in res {
        print!("{} ", x);
    }
}
