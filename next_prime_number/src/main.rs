use std::io;

fn console_line() -> io::Result<String> {
    let mut s = String::new();
    match io::stdin().read_line(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn is_prime(num: u128) -> bool {
    // 2 is first real prime and test till half + 1 since no prime is there
    for i in 2..num / 2 + 1 {
        if num % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut num = 1u128;
    loop {
        println!("Current prime: {}\nPrint next? (y,n)", num);
        match console_line() {
            Err(_) => {
                println!("Couldn't read console.");
                return;
            }
            Ok(s) => match s.chars().next().unwrap() {
                'y' => (),
                'n' => break,
                _ => {
                    println!("Unknown answer.");
                    return;
                }
            },
        };

        for i in num + 1.. {
            if is_prime(i) {
                num = i;
                break;
            }
        }
    }
}
