use error::Error;
use std::env::args;

#[cfg(feature = "async")]
use tokio::runtime;

#[cfg(feature = "async")]
use asynced::scan;

#[cfg(feature = "sync")]
use multithread::scan;

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        return Err(Error::CliUsage);
    }

    let target = args[1].as_str();

    #[cfg(feature = "sync")]
    {
        scan(target)
    }

    #[cfg(feature = "async")]
    {
        // e.g. set up a tokio runtime and call the async version
        let rt = runtime::Runtime::new().unwrap();
        rt.block_on(scan(target))
    }
}
