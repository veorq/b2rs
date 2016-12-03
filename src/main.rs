extern crate b2rs;
use b2rs::b2xb;
use b2rs::b2b;
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

    let mut msg = [0u8; 256];
    for i in 0..256 { msg[i] = i as u8; }
    let mut key = [0u8; 64];
    for i in 0..64 { key[i] = i as u8; }

    let hash = b2b::hash_keyed(&msg[..256], &key[..64]);

    for x in &hash { print!("{:02x}", *x); }
    println!("");

}
