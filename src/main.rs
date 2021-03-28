mod libpacman;
use core::str::next_code_point;

use libpacman::{count_ghosts, read_value};

fn main() {
    let n = read_value::<u32>().unwrap();
    println!("{}", count_ghosts(n));
}


