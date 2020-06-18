use sha1::{Sha1, Digest};
use md5::{Md5};
use sha2::{Sha256, Sha512};
use std::{env, fs, io};
extern crate ssdeep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut file = fs::File::open(&path).expect("Unable to open");
    let mut md5hasher = Md5::new();
    io::copy(&mut file, &mut md5hasher).expect("Unable to hash");
    let md5hash = md5hasher.result();

    let mut file = fs::File::open(&path).expect("Unable to open");
    let mut sha1hasher = Sha1::new();
    io::copy(&mut file, &mut sha1hasher).expect("Unable to hash");
    let sha1hash = sha1hasher.result();

    let mut file = fs::File::open(&path).expect("Unable to open");
    let mut sha256hasher = Sha256::new();
    io::copy(&mut file, &mut sha256hasher).expect("Unable to hash");
    let sha256hash = sha256hasher.result();

    let mut file = fs::File::open(&path).expect("Unable to open");
    let mut sha512hasher = Sha512::new();
    io::copy(&mut file, &mut sha512hasher).expect("Unable to hash");
    let sha512hash = sha512hasher.result();

    let ssdeephash = ssdeep::hash_from_file(&path).unwrap();

    println!("File:   {}", &path);
    println!("MD5:    {:x}", md5hash);
    println!("SHA1:   {:x}", sha1hash);
    println!("SHA256: {:x}", sha256hash);
    println!("SHA512: {:x}", sha512hash);
    println!("SSDEEP: {}", ssdeephash);
}