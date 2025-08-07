use sha1::{Digest, Sha1};
use chrono::Local;
use hex;
use std::fs::{self, File};
use std::io::Write;
use flate2::{write::ZlibEncoder, Compression};

pub fn commit_tree(tree_sha: &str, message: &str, parent_sha: Option<&str>) {
    let now = Local::now();
    let timestamp = now.timestamp();
    let timezone_offset = now.format("%z").to_string();

    let author = "Anmol Singh <anmol@localhost>";
    let mut commit_body = format!("tree {}\n", tree_sha);

    if let Some(parent) = parent_sha {
        commit_body.push_str(&format!("parent {}\n", parent));
    }

    commit_body.push_str(&format!(
        "author {} {} {}\ncommitter {} {} {}\n\n",
        author, timestamp, timezone_offset,
        author, timestamp, timezone_offset,
    ));
    commit_body.push_str(message);
    commit_body.push('\n');

    let full_header = format!("commit {}\0", commit_body.len());
    let full_data = [full_header.as_bytes(), commit_body.as_bytes()].concat();

    let mut hasher = Sha1::new();
    hasher.update(&full_data);
    let hash = hasher.finalize();
    let hex_sha = hex::encode(hash);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&full_data).unwrap();
    let compressed_data = encoder.finish().unwrap();

    let dir = &hex_sha[0..2];
    let file = &hex_sha[2..];
    let object_dir = format!(".git/objects/{}", dir);
    let object_path = format!("{}/{}", object_dir, file);

    fs::create_dir_all(&object_dir).unwrap();
    let mut f = File::create(&object_path).unwrap();
    f.write_all(&compressed_data).unwrap();

    println!("{}", hex_sha);
}

