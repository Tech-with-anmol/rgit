#[allow(unused_imports)]
use std::env;
use std::{env::args, io::Read};
#[allow(unused_imports)]
use std::fs;
use flate2::read::ZlibDecoder;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file" || args[2] == "-p" {
       let object = &args[3];
       let path = format!(".git/objects/{}/{}", object.get(0..2).unwrap() ,object.get(2..).unwrap());
       let mut file = fs::File::open(path).unwrap();
       let mut decoder = ZlibDecoder::new(file);
       let mut content = String::new();
       decoder.read_to_string(&mut content).unwrap();
       let null_index = content.find('\0').unwrap();
       let content = content.get(null_index + 1..).unwrap().to_string();

       print!("{}", content);
    } else {
        println!("unknown command: {}", args[1])
    }
}
