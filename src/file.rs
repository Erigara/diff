use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader, Error, Lines};

type UnwprapedLineIterator =
    std::iter::Map<Lines<BufReader<File>>, fn(Result<String, Error>) -> String>;

pub fn file_to_line_iter(file_name: &str) -> UnwprapedLineIterator {
    match File::open(file_name) {
        Ok(f) => {
            let reader = BufReader::new(f);
            reader
                .lines()
                .map(Result::unwrap as fn(s: Result<String, Error>) -> String)
        }
        Err(err) => {
            panic!(err);
        }
    }
}

pub fn file_to_hash_vector(file_name: &str) -> Vec<u64> {
    match File::open(file_name) {
        Ok(f) => {
            let reader = BufReader::new(f);
            reader.lines().map(|l| hash_string(&l.unwrap())).collect()
        }
        Err(err) => {
            panic!(err);
        }
    }
}

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
