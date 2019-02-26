# strs
*strs* is a modern alternative to [*strings*](https://linux.die.net/man/1/strings).

```
strs 0.1.3
RJ Rybarczyk <rj@rybar.tech>
A modern alternative for strings

USAGE:
    strs [OPTIONS] [FILE]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --number <number>    Specify the minimum string length, where the number argument is a positive decimal integer.
                             [default: 4]
    -o, --offset <offset>    Write each string preceded by its byte offset from the start of the file. The format shall
                             be dependent on the single character used as the format option-argument:
                             
                             	d	The offset shall be written in decimal. [default]
                             	o	The offset shall be written in octal.
                             	x	The offset shall be written in hexadecimal.

ARGS:
    <FILE>...    Specify the files to process.
```
