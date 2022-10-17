use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

use rustc_serialize::hex::ToHex;

use md5::Md5;
use sha2::{Digest, Sha256, Sha512};

use rayon::prelude::*;

fn hash_iteratively(input: &str) -> String {
    // hashes the input with md5 100 times, then with sha256 100 times, then with sha512 100 times
    let mut out = input.to_owned();
    for _ in 0..100 {
        out = Md5::digest(out).as_slice().to_hex();
    }

    for _ in 0..100 {
        out = Sha256::digest(out).as_slice().to_hex();
    }

    for _ in 0..100 {
        out = Sha512::digest(out).as_slice().to_hex();
    }

    out
}

fn main() {
    let target = "069f4c68a604551e25af06f1c8a365fc56a5617dd8021032487076fd6ee8fe88eec9a0a0c4aa1d719f3412d0bd010bd9f289950674fe7cad7f95bcbe58bedd4a";
    let filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: {} <filename>", std::env::args().nth(0).unwrap());
        std::process::exit(1);
    });

    let file = File::open(filename).expect("Could not open file");

    // loop and read from standard input, checking the hash
    let file_contents = BufReader::new(file);
    let answer = file_contents
        .lines()
        .map(Result::unwrap)
        .par_bridge()
        .find_any(|line| hash_iteratively(&line) == target);

    match answer {
        Some(answer) => {
            println!("Found answer: {}", answer);
            fs::write("answer.txt", answer).expect("Unable to write file");
        }
        None => println!("No answer found"),
    }
}
