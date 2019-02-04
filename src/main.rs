use std::io::{self, Read};

fn main() {

    // read from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut char_run = String::new();
    for byte in handle.bytes() {
        let byte = byte.unwrap();

        if byte as u32 >= '!' as u32 && byte as u32 <= '~' as u32 {
            char_run.push(byte as char);
        } else {
            if char_run.len() >= 4 {
               println!("{}", char_run);
            }
            char_run.clear();
        }
    }
}
