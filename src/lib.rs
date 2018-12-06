extern crate chrono;
use chrono::prelude::*;

#[derive(Debug, PartialEq)]
pub struct LedgerTimeEntry {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub account: String,
    pub description: bool,
}

pub fn parse_entry(entry: String) -> Result<LedgerTimeEntry, &'static str> {
    let mut lines = entry.lines();
    let clock_in = match lines.next() {
        Some(value) => value,
        None => {
            return Err("Needs at least two lines!")
        }
    };
    let clock_out = match lines.next() {
        Some(value) => value,
        None => {
            return Err("Needs at least two lines!")
        }
    };

    let mut parts = clock_in.split("  ");
    let first = match parts.next() {
        Some(value) => value,
        None => {
            return Err("Needs at least two lines!")
        }
    };
    println!("Test {}", clock_in);
    Err("Implement")
}

#[cfg(test)]
mod test {
    use super::*;

//     #[test]
//     fn parse_one_entry() {
//         let contents = "\
// i 2018/12/05 14:20 BA:MISC  Reply about phone issues
// o 2018/12/05 14:35 BA:MISC";

//         parse_entry(contents.to_string());
//         // assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }

    #[test]
    fn parse_incomplete_entry() {
        let contents = "i 2018/12/05 14:20 BA:MISC  Reply about phone issues";
        let result = parse_entry(contents.to_string());
        assert_eq!(Err("Needs at least two lines!"), result);
    }
}
