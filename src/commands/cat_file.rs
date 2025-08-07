use std::{fs::File, io::Read};
use flate2::read::ZlibDecoder;

pub fn cat_file(sub_command: &str, object: &str) {
    if sub_command != "-p" {
        eprintln!("Usage: rgit cat-file -p <object>");
        return;
    }

    let path = format!(".git/objects/{}/{}", &object[..2], &object[2..]);
    let file = File::open(path).unwrap();
    let mut decoder = ZlibDecoder::new(file);
    let mut content = String::new();
    decoder.read_to_string(&mut content).unwrap();
    let content = content.split_once('\0').unwrap().1;
    print!("{}", content);
}

