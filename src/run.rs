use crate::runtime_error::{ArgError, Error};
use std::{fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "strs")]
struct Config {
    /// Minimum string length
    #[structopt(
        short = "n",
        long = "number",
        default_value = "4",
        help = "Specify the minimum string length, where the number argument is a positive decimal integer."
    )]
    number: usize,

    /// Files to process
    #[structopt(
        name = "FILE",
        parse(from_os_str),
        help = "Specify the files to process."
    )]
    files: Vec<PathBuf>,

    /// Byte offset from the start of the file
    #[structopt(
        short = "o",
        long = "offset",
        help = "Write each string preceded by its byte offset from the start of the file. The format shall be dependent on the single character used as the format option-argument:\n\n\td\tThe offset shall be written in decimal. [default]\n\to\tThe offset shall be written in octal.\n\tx\tThe offset shall be written in hexadecimal."
    )]
    offset: Option<OffsetFormat>,
}

#[derive(Debug)]
enum OffsetFormat {
    Decimal,
    Hexadecimal,
    Octal,
}

impl std::default::Default for OffsetFormat {
    fn default() -> Self {
        OffsetFormat::Decimal
    }
}

impl std::str::FromStr for OffsetFormat {
    type Err = ArgError;
    fn from_str(s: &str) -> Result<OffsetFormat, ArgError> {
        match s {
            "d" => Ok(OffsetFormat::Decimal),
            "x" => Ok(OffsetFormat::Hexadecimal),
            "o" => Ok(OffsetFormat::Octal),
            _ => {
                let details = "Offset must be 'd' (decimal), 'h' (hexadecimal), or 'o' (octal).";
                Err(ArgError::InvalidArgs {
                    details: details.to_string(),
                })
            }
        }
    }
}

fn search_strs(number:usize, offset: &Option<OffsetFormat>, handle: &mut Read) -> Result<(), Error> {
    let mut char_run = String::new();
    let mut offset_count = 0;

    for byte in handle.bytes() {
        // handle.bytes() returns Result. ? gets actual byte value
        let byte = byte?;

        if byte >= b'!' && byte <= b'~' {
            char_run.push(byte as char);
        } else {
            if char_run.len() >= number {
                match offset {
                    Some(OffsetFormat::Decimal) => println!("{}\t{}", offset_count, char_run),
                    Some(OffsetFormat::Hexadecimal) => {
                        println!("{:x}\t{}", offset_count, char_run)
                    }
                    Some(OffsetFormat::Octal) => println!("{:o}\t{}", offset_count, char_run),
                    None => println!("{}", char_run),
                };
            }
            char_run.clear();
        }
        offset_count += 1;
    }

    Ok(())
}

pub fn run() -> Result<(), Error> {
    let opt = Config::from_args();

    for path in &opt.files {
        let mut file = File::open(path)?;
        search_strs(opt.number, &opt.offset, &mut file);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    /// Tests error if file does not exist.
    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn app_error_from_read_error() {
        File::open("./dne.file").unwrap();
    }
}
