mod b2b;

use std::env;

pub fn main() {
    let args: Vec<_> = env::args().collect();

    let msg = args[1].as_bytes();

    let hash = b2b::b2b(msg);
    for x in hash.iter().take(64) {
        print!("{:02x}", x);
    }
}
