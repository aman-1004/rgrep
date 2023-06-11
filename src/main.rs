use std::{env::{args, Args}, io::{self, BufRead}};
use regex::Regex;

fn main() {
    let mut argument: Args = args();
    let pattern: String = argument.nth(1).unwrap();
    // println!("{}", pattern);
    let regex_pattern = Regex::new(pattern.as_str()).unwrap();
    let reader = io::stdin().lock();

    for line in reader.lines() {
        let line_string = line.unwrap();
        if regex_pattern.is_match(&line_string) {
            println!("{}", line_string);
        }
    }
}
