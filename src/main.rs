use std::io::{self, Read};
use structopt::StructOpt;

// TODO: 
// - read files from the command line
// - implement all options from strings
// - be utf-8 aware
// - support various text encodings (if i want to get crazy)


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "n", long = "number", default_value = "4")]
    number: usize,
}

fn main() {

    let opt = Opt::from_args();

    // read from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut char_run = String::new();
    for byte in handle.bytes() {
        let byte = byte.unwrap();

        if byte as u32 >= '!' as u32 && byte as u32 <= '~' as u32 {
            char_run.push(byte as char);
        } else {
            if char_run.len() >= opt.number {
               println!("{}", char_run);
            }
            char_run.clear();
        }
    }
}
