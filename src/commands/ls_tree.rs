use std::{fs::File, io::Read, path::Path};
use flate2::read::ZlibDecoder;

pub fn ls_tree(tree_sha: &str) {
    let path = Path::new(".git/objects").join(&tree_sha[0..2]).join(&tree_sha[2..]);
    let file = File::open(path).unwrap();
    let mut decoder = ZlibDecoder::new(file);
    let mut content = Vec::new();
    decoder.read_to_end(&mut content).unwrap();

    let data = &content[content.iter().position(|&b| b == 0).unwrap() + 1..];
    let mut i = 0;
    while i < data.len() {
        let mode_end = i + data[i..].iter().position(|&b| b == b' ').unwrap();
        let name_end = mode_end + 1 + data[mode_end + 1..].iter().position(|&b| b == 0).unwrap();
        let name = &data[mode_end + 1..name_end];
        println!("{}", String::from_utf8_lossy(name));
        i = name_end + 1 + 20; // skip 20-byte SHA1
    }
}

