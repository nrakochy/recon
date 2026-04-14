use error::Error;

mod constants;
mod model;
mod subdomains;

pub fn scan(_target: &str) -> Result<(), error::Error> {
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
