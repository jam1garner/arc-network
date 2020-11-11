#![feature(proc_macro_hygiene)]

use std::fs::File;
use network_reader::Networked;

const PORT: u16 = 43022;

#[skyline::main(name = "arc-network")]
pub fn main() {
    std::thread::spawn(||{
        Networked::new_buffered(File::open("rom:/data.arc").unwrap(), ("0.0.0.0", PORT))
            .unwrap()
            .listen()
            .unwrap();
    });
}
