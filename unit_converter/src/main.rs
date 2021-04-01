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

fn convert(l: &[(&str, f64, f64)], from: &str, to: &str, val: f64) -> Option<f64> {
    let &from_i = l.iter().find(|&&e| e.0 == from)?;
    let &to_i = l.iter().find(|&&e| e.0 == to)?;

    Some((val - from_i.2) / from_i.1 * to_i.1 + to_i.2)
}

fn main() {
    let mut args = env::args();

    if args.len() != 5 {
        println!(
            "{} <type> <from> <to> <val>\nConvert from a type to another type.",
            args.nth(0).unwrap()
        );
        return;
    }

    args.next(); // Skip location arg

    let name = args.next().unwrap();
    let from = args.next().unwrap();
    let to = args.next().unwrap();
    let val = arg_to_int!(args.next().unwrap(), f64);

    let result = match match name.as_str() {
        "temperature" => {
            let arr = [
                ("°C", 1f64, 0f64),
                ("°F", 1.8f64, 32f64),
                ("K", 0f64, 273.15f64),
            ];
            convert(&arr, from.as_str(), to.as_str(), val)
        }
        "distance" => {
            let arr = [
                ("m", 1f64, 0f64),
                ("ft", 3.2808f64, 0f64),
                ("yd", 1.0936f64, 0f64),
            ];
            convert(&arr, from.as_str(), to.as_str(), val)
        }
        "time" => {
            let arr = [
                ("s", 1f64, 0f64),
                ("min", 1.66666666667e-2f64, 0f64),
                ("h", 2.77777777778e-4f64, 0f64),
            ];
            convert(&arr, from.as_str(), to.as_str(), val)
        }
        _ => {
            println!("Unrecognized catagory.");
            return;
        }
    } {
        Some(v) => v,
        None => {
            println!("Unrecognized unit.");
            return;
        }
    };

    println!("{:.5}", result);
}
