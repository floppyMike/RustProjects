use std::slice;
use std::str;

fn main() {
    let e: &'static str = "2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382178525166427";

    if std::env::args().len() != 2 {
        println!("e_to_nth_digit <num>\nPrint the constant to num digit. Goes till 100.");
        return;
    }

    let num = match usize::from_str_radix(std::env::args().nth(1).unwrap().as_str(), 10) {
        Ok(n) => n,
        Err(_) => {
            println!("Couldn't parse the given num.");
            return;
        }
    };

    if num == 1 {
        println!("{}", e.chars().next().unwrap());
        return;
    }

    if num > 100 {
        println!("Requested num to large.");
        return;
    }

    let slice = unsafe {
        let s = slice::from_raw_parts(e.as_ptr(), num + 1);
        str::from_utf8(s).unwrap()
    };

    println!("{}", slice);
}
