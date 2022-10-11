use std::fs;

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
    let ncsu = "ncstate";
    let ncsu_hash = "2d4547b1e59358fbb8846022b58e6c915fa644ee1735788b6cdaf0f8ba4bc3b8ea778d092e786047b018086a0f14db7ba684eefb150a90492d7f9caf6b6bc114";
    let csc = "csc474";
    let csc_hash = "d07bbb7bf6ca344bd6632b90d38b07a77cdc03f0d18f9a8411b069bd716be5a13bba41cd205432b1bdcca9542c865ccc05e8c4f529f064de1f71c7ee5731beeb";
    let security = "security";
    let security_hash = "88466c23009271eb909e586c6707120f30a2dd6ae53fb025badb2f9d0f6765b90f124e3524d2d63719e8e668cf164411a61b1fea077ec1dd3b1db8889622095c";
    assert_eq!(hash_iteratively(ncsu), ncsu_hash);
    println!("NCSU Hash Matches!");
    assert_eq!(hash_iteratively(csc), csc_hash);
    println!("CSC Hash Matches!");
    assert_eq!(hash_iteratively(security), security_hash);
    println!("Security Hash Matches!");

    let target = "069f4c68a604551e25af06f1c8a365fc56a5617dd8021032487076fd6ee8fe88eec9a0a0c4aa1d719f3412d0bd010bd9f289950674fe7cad7f95bcbe58bedd4a";

    // file names are all args
    for filename in std::env::args().skip(1) {
        println!("Checking {filename}");
        match std::fs::read_to_string(&filename)
            .unwrap()
            .lines()
            .par_bridge()
            .find_any(|line| hash_iteratively(line) == target)
        {
            Some(answer) => {
                println!("Cracked hash: {answer}");
                fs::write("answer.txt", answer).expect("Unable to write file");
                return;
            }
            None => {
                println!("No match found in {filename}");
            }
        }
    }

    println!("No match found in any file");
}
