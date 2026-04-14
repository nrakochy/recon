use error::Error;
use scanner::scan;
use std::env::args;

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage);
    }

    let target = args[1].as_str();
    scan(target)
}
