extern crate sphinx;

use std::env::args;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::Error;
use std::collections::HashSet;
use std::io::stdout;

use sphinx::buffer::*;

fn main() {
    let mut args = args();
    args.next();

    match args.next() {
        None => panic!("expected a file name"),

        Some(file_name) => {
            let mut bytes = match File::open(file_name) {
                err@Err(_) => panic!("{:?}", err),
                Ok(mut f) => { f.bytes() },
            };

            let r_buf: Result<Buffer<u8, Marker>, Error> =
                Buffer::generic_load_2(512, 100, |mut chunk| {

                let mut taken = 0;
                while let Some(result) = bytes.next() {
                    match result {
                        Err(e) => return Err(e),

                        Ok(byte) => {
                            chunk.push(byte);
                            if byte == 10 {
                                chunk.mark_at(Linebreak, taken);
                            }
                        },
                    }

                    taken += 1;
                    if taken >= chunk.capacity() {
                        break;
                    }
                }

                if taken == 0 {
                    Ok(None)
                } else {
                    Ok(Some(chunk))
                }
            });

            match r_buf {
                Ok(buf) => {
                    /*
                    let mut out = stdout();
                    let data: Vec<u8> = buf.into_iter().cloned().collect();
                    out.write(&data);
                    */
                    println!("{}", buf.marker_count(Linebreak))
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}
