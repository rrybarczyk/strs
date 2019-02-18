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


#[derive(Debug, PartialEq)]
pub enum AppError {
    IoError(String),
}

impl From<io::Error> for AppError {
    fn from (error: io::Error) -> Self {
        AppError::IoError(error.to_string())
    }
}


fn print_strs(number: usize, handle: &mut Read) -> Result<(), AppError> {
    let mut char_run = String::new();
    for byte_result in handle.bytes() {
        // handle.bytes() returns Result. ? gets actual byte
        let byte = byte_result?;

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

pub fn run() -> Result<(), AppError> {

    let opt = Opt::from_args();

    for path in opt.files {
        let mut file = File::open(path)?;
        print_strs(opt.number, &mut file)?;
    }

    Ok(())
}

pub fn open_file(path: &str) -> Result<File, AppError> {
    let file = File::open(path)?;
    Ok(file)

}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;
    // use INVALID_FILE_READ;
    #[test]
    fn app_error_from_read_error() {
        // try to import a nonexistant file
        // assert that error is of type AppError
        let path = "./dne.file";
        assert_eq!(open_file(path).unwrap_err(), AppError::IoError("No such file or directory (os error 2)".to_string()));
    }
}
