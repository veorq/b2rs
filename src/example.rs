extern crate b2rs;
use b2rs::b2xb;

use std::env;

pub fn main() {
    let args: Vec<_> = env::args().collect();

    let msg = args[1].as_bytes();

    let hash = b2xb::hash(msg, 256);
    for x in &hash {
        print!("{:02x}", *x);
    }
}
