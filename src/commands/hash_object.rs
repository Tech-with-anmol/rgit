use std::{fs, path::Path, io::Write};
use sha1::{Sha1, Digest};
use flate2::{write::ZlibEncoder, Compression};

pub fn hash_object(file: &str) {
    let content = fs::read(file).unwrap();
    let data = format!("blob {}\0{}", content.len(), String::from_utf8_lossy(&content));
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    let dir = &hash[0..2];
    let file_name = &hash[2..];
    let object_dir = Path::new(".git/objects").join(dir);
    let object_file = object_dir.join(file_name);
    fs::create_dir_all(&object_dir).unwrap();

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();
    fs::write(object_file, compressed).unwrap();
    println!("{}", hash);
}

