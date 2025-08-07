use std::{fs, path::Path, io::Write};
use sha1::{Sha1, Digest};
use flate2::{write::ZlibEncoder, Compression};
use hex;  

pub fn write_tree(path: &Path) -> String {
    let mut entries = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name == ".git" {
            continue;
        }

        if path.is_file() {
            let content = fs::read(&path).unwrap();
            let data = format!("blob {}\0{}", content.len(), String::from_utf8_lossy(&content));
            let mut hasher = Sha1::new();
            hasher.update(data.as_bytes());
            let hash = hasher.finalize();
            let hash_hex = format!("{:x}", hash);

            let object_path = Path::new(".git/objects").join(&hash_hex[0..2]).join(&hash_hex[2..]);
            fs::create_dir_all(object_path.parent().unwrap()).unwrap();
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data.as_bytes()).unwrap();
            fs::write(&object_path, encoder.finish().unwrap()).unwrap();

            let mut entry_bytes = Vec::new();
            entry_bytes.extend_from_slice(format!("100644 {}\0", name).as_bytes());
            entry_bytes.extend_from_slice(&hex::decode(hash_hex).unwrap());
            entries.push(entry_bytes);
        } else if path.is_dir() {
            let subtree_sha = write_tree(&path);
            let mut entry_bytes = Vec::new();
            entry_bytes.extend_from_slice(format!("40000 {}\0", name).as_bytes());
            entry_bytes.extend_from_slice(&hex::decode(subtree_sha).unwrap());
            entries.push(entry_bytes);
        }
    }

    entries.sort_by_key(|e| {
        e.iter().skip_while(|&&b| b != b' ').skip(1).take_while(|&&b| b != 0).cloned().collect::<Vec<u8>>()
    });

    let mut tree_content = Vec::new();
    for entry in entries {
        tree_content.extend_from_slice(&entry);
    }

    let mut store = Vec::new();
    store.extend_from_slice(format!("tree {}\0", tree_content.len()).as_bytes());
    store.extend_from_slice(&tree_content);

    let mut hasher = Sha1::new();
    hasher.update(&store);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    let object_path = Path::new(".git/objects").join(&hash_hex[0..2]).join(&hash_hex[2..]);
    fs::create_dir_all(object_path.parent().unwrap()).unwrap();
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&store).unwrap();
    fs::write(object_path, encoder.finish().unwrap()).unwrap();

    hash_hex
}

