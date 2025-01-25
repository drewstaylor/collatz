// Collatz Conjecture

use std::env;

// pub static DEBUG: bool = false;
pub static DEBUG: bool = true;

fn even_or_odd(num: &u32) -> u32 {
    num % 2
}
fn even(num: u32) -> u32 {
    num / 2
}
fn odd(num: u32) -> u32 {
    3 * num + 1
}

fn collatz(mut num: u32) {
    while num != 1 {
        if DEBUG {
            println!("Processing: {}", num);
        }
        let n_type = even_or_odd(&num);
        match n_type {
            0 => num = even(num),
            1 => num = odd(num),
            _ => println!("Failed: {}", n_type),
        }
    }
    if DEBUG {
        println!("End: {}", num);
    }
    assert_eq!(num, 1u32);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let size: u32 = args.len() as u32;
    let start: u32 = if size == 3 {
        args[&args.len() - 2].parse::<u32>().unwrap_or(1_u32)
    } else {
        1_u32
    };
    let end: u32 = if size == 3 {
        args[&args.len() - 1].parse::<u32>().unwrap_or(100_u32)
    } else {
        100_u32
    };
    if start < 1 || end < start {
        return println!("Invalid coordinates, Start: {}, End: {}", start, end);
    }
    for i in start..end + 1 {
        if DEBUG {
            println!("Start: {}", &i);
        }
        collatz(i);
    }
}
