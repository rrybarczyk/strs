use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};
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

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

fn print_strs(number: usize, handle: &mut Read) -> Result<(), Error> {
    let mut char_run = String::new();
    for byte in handle.bytes() {
        // handle.bytes() returns Result. ? gets actual byte value
        let byte = byte?;

        if byte >= b'!' && byte <= b'~' {
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

pub fn run() -> Result<(), Error> {
    let opt = Opt::from_args();

    for path in opt.files {
        let mut file = File::open(path)?;
        print_strs(opt.number, &mut file)?;
    }

    Ok(())
}

pub fn open_file(path: &str) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    /// Tests error if file does not exist.
    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn app_error_from_read_error() {
        open_file("./dne.file").unwrap();
    }
}
