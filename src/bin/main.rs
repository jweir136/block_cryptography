extern crate block_cryptography;

use block_cryptography::hashing::hash_digest;
use ring::digest::{SHA256, Context};
use std::fs::File;

fn main() {
    let file = File::open("test.txt").unwrap();    
    let context = Context::new(&SHA256);

    let digest = hash_digest(file, context);   
    println!("{:?}", digest);
    //println!("{:?}", file);
}
