use std::fmt::Debug;
use crate::utils::file_helpers::{read_full_file, read_sample_file};

pub trait Day<T: Debug> {
    fn id(&self) -> &'static str;

    fn run(&self, lines: Vec<String>) -> T;

    fn run_sample(&self) -> T {
        let lines = read_sample_file(self.id()).unwrap();
        self.run(lines)
    }
    fn run_full(&self) -> T {
        let lines = read_full_file(self.id()).unwrap();
        let result = self.run(lines);
        println!("{} solution is {:?}", self.id(), result);
        result
    }
}


