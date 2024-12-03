use std::fmt::Debug;
use std::str::FromStr;

pub fn string_to_numbers<T: FromStr>(line: &str, delim: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    line.split(delim).map(|s| s.parse::<T>().unwrap()).collect()
}
