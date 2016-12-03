extern crate b2rs;
use b2rs::b2xb;
use std::env;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!{"usage: {} message length", args[0]}
        return
    }
    let msg = args[1].as_bytes();
    let length: u32 = args[2].trim().parse().expect("length needed");

    let hash = b2xb::hash(msg, length);
    for x in &hash { print!("{:02x}", *x); }
    println!("");
}
