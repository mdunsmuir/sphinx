extern crate sphinx;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

use sphinx::buffer::*;

fn main() {
    let mut args = args();
    args.next();

    match args.next() {
        None => panic!("expected a file name"),

        Some(file_name) => {
            let reader = match File::open(file_name) {
                err@Err(_) => panic!("{:?}", err),
                Ok(f) => BufReader::new(f),
            };

            let buf = Buffer::from_bytes(reader).unwrap();
            println!("{}", buf.marker_count(Linebreak));
        }
    }
}
