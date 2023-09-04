use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("no argument");
        return;
    }
    let input = &args[1];
    if let Ok(mut num) = f64::from_str(input) {
        let mut output: Vec<u64> = Vec::new();
        loop {
            let floor = num.floor();
            let floor_int = floor as u64;
            output.push(floor_int);
            if output.len() >= 200 || num - floor < 0.00000000000001 {
                break;
            }
            num = 1.0 / (num - floor);
        }
        println!("{:?}", output);
    } else {
        eprintln!("parse error");
        return;
    }
}
