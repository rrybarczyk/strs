use std::io::{self, Read};
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "strs")]
struct Opt {
    #[structopt(short = "n", long = "number", default_value = "4")]
    number: usize,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn print_strs(number: usize, handle: &mut Read) -> io::Result<()> {
    let mut char_run = String::new();
    for byte in handle.bytes() {
        let byte = byte?;

        if byte as u32 >= '!' as u32 && byte as u32 <= '~' as u32 {
            char_run.push(byte as char);
        } else {
            if char_run.len() >= number {
                println!("{}", char_run);
            }
            char_run.clear();
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {

    let opt = Opt::from_args();

    for path in opt.files {
        let mut file = File::open(path)?;
        print_strs(opt.number, &mut file)?;
    }

    // // read from stdin
    // let stdin = io::stdin();
    // let mut handle = stdin.lock();


    Ok(())
}

