#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::{env::args, io::Read};
#[allow(unused_imports)]
use std::fs;
use std::result::Result::Ok;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::Sha1;
use std::path::Path;
use std::io::Write;

enum Command {
    Init,
    CatFile {
        subCommand: String,
        object: String,
    },
    HashObject {
        file: String,
    },
    LsTree {
        tree_sha: String,
    },
    WriteTree,
}

impl Command {
    fn parse_args(args: &[String]) -> Result<Self, String> {
        if args.len() < 2 {
            return Err("Not enough arguments".into());
            
        }
        match args.get(1).map(|arg| arg.as_str()) {
            Some("init") => Ok(Self::Init),
            Some("cat-file") => {
                if args.len() < 4 {
                    return Err("Usge: rgit cat-file -p <object>".into());
                }
                Ok(Self::CatFile { subCommand: args[2].clone(), object: args[3].clone() })
            },
            Some("hash-object") => {
                if args.len() < 3 || args[2] != "-w"{
                    return Err("Usage: rgit hash-object -w <file>".into());
                }
                Ok(Self::HashObject { file: args[3].clone() })
                
            },
            Some("ls-tree") => {
                if args.len() < 3 || args[2] != "--name-only"{
                    return Err("Usage: rgit ls-tree --name-only <tree_sha>".into());
                }
                Ok(Self::LsTree { tree_sha: args[3].clone()})
            },
            Some("write-tree") => {
                if args.len() < 1 {
                    return Err("Usage: rgit write-tree".into());
                }
                Ok(Self::WriteTree)
            },
            _ => Err(format!("Unknown command: {}", args[1])),
        }
    }    


    fn excute(&self) {
        match self {
            Self::Init => {
             fs::create_dir(".git").unwrap();
             fs::create_dir(".git/objects").unwrap();
             fs::create_dir(".git/refs").unwrap();
             fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
             println!("Initialized git directory")

            }
            Self::CatFile {subCommand, object}=> {
               if subCommand != "-p" {
                   println!("Usage: rgit cat-file -p <object>");
                   return;
               } else {
                   
                   let path = format!(".git/objects/{}/{}", object.get(0..2).unwrap() ,object.get(2..).unwrap());
                   let file = fs::File::open(path).unwrap();
                   let mut decoder = ZlibDecoder::new(file);
                   let mut content = String::new();
                   decoder.read_to_string(&mut content).unwrap();
                   let null_index = content.find('\0').unwrap();
                   let content = content.get(null_index + 1..).unwrap().to_string();

                  print!("{}", content);
                }
            }
            Self::HashObject { file } => {
                let content = fs::read(file).unwrap_or_else(|_| {
                    eprintln!("Error reading file: {}", file);
                    std::process::exit(1);
                });
                let hasher_data = format!("blob {}\0{}", content.len(), String::from_utf8_lossy(&content));
                let mut hasher = Sha1::new();
                hasher.update(hasher_data.as_bytes());
                let hash = hasher.finalize();
                let hash_out = format!("{:x}", hash);
                let dir = &hash_out[0..2];
                let file_name = &hash_out[2..];
                let object_dir = Path::new(".git/objects").join(dir);
                let object_file = object_dir.join(file_name);
                fs::create_dir_all(&object_dir).unwrap_or_else(|_| {
                    eprintln!("Error creating directory: {}", object_dir.display());
                    std::process::exit(1);
                });
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(hasher_data.as_bytes()).unwrap_or_else(|_| {
                    eprintln!("Error writing to zlib encoder");
                    std::process::exit(1);
                });
                let compressed_data = encoder.finish().unwrap();
                fs::write(&object_file, compressed_data).unwrap_or_else(|_| {
                    eprintln!("Error writing to object file: {}", &object_file.display());
                    std::process::exit(1);
                });
                println!("{}", hash_out);

            }

            Self::LsTree {tree_sha} => {
                let sha_dir = &tree_sha[0..2];
                let sha_file = &tree_sha[2..];
                let sha_object_dir = Path::new(".git/objects").join(&sha_dir);
                let sha_object_file = sha_object_dir.join(sha_file);
                let file = fs::File::open(&sha_object_file).unwrap_or_else(|_| {
                    eprintln!("Error opening object file: {}", &sha_object_file.display());
                    std::process::exit(1);
                });
                let mut decoder = ZlibDecoder::new(file);
                let mut content = Vec::new();
                decoder.read_to_end(&mut content).unwrap_or_else(|e| {
                    eprintln!("Error reading from zlib decoder: {}", e);
                    std::process::exit(1);
                });
                let null_index = content.iter().position(|&a| a == 0).unwrap_or_else(|| {
                    eprintln!("Error finding null char in content");
                    std::process::exit(1);
                });
                let content = &content[null_index+1..];
                let mut i = 0;
                while i < content.len() {
                    let mode_end = content[i..].iter().position(|&a| a == b' ').unwrap_or_else(|| {
                        eprintln!("Error finding mode end in content");
                        std::process::exit(1);
                    }) + i;
                    let _mode = &content[i..mode_end];
                    i = mode_end + 1;
                    let name_end  = content[i..].iter().position(|&a| a == b'\0').unwrap_or_else(|| {
                        eprintln!("Error finding name end in content");
                        std::process::exit(1);
                    }) + i;
                    let name = &content[i..name_end];
                    i = name_end + 1;
                    let sha_end = i + 20;
                    let _sha = &content[i..sha_end];
                    println!("{}", String::from_utf8_lossy(name));
                    i = sha_end;
               }
               
            }

            Self::WriteTree => {
                let mut tree_entries = Vec::new();
                for entry in fs::read_dir(".").unwrap() {
                    tree_entries.sort_by_key(|entry| {
                    entry.iter().skip_while(|&&b| b != b' ').skip(1).take_while(|&&b| b != 0).cloned().collect::<Vec<u8>>()
                    });
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_file() {
                        let content = fs::read(&path).unwrap_or_else(|_|{
                            eprintln!("Error reading file: {}", path.display());
                            std::process::exit(1);
                        });
                        let hasher_data = format!("blob {}\0{}", content.len(), String::from_utf8_lossy(&content));
                        let mut hasher = Sha1::new();
                        hasher.update(hasher_data.as_bytes());
                        let hash = hasher.finalize();
                        let hash_out = format!("{:x}", hash);
                        let dir = &hash_out[0..2];
                        let file_name = &hash_out[2..];
                        let object_dir = Path::new(".git/objects").join(dir);
                        let object_file = object_dir.join(file_name);
                        fs::create_dir_all(&object_dir).unwrap_or_else(|_| {
                            eprintln!("Error creating directory: {}", object_dir.display());
                            std::process::exit(1);
                        });
                        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                        encoder.write_all(hasher_data.as_bytes()).unwrap_or_else(|_|{
                            eprintln!("Error writing to zlib encoder");
                            std::process::exit(1);
                        });
                        let compressed_data = encoder.finish();
                        fs::write(&object_file, compressed_data.unwrap()).unwrap_or_else(|_| {
                            eprintln!("Error writing to object file: {}", &object_file.display());
                            std::process::exit(1);
                        });
                        let mode = "100644";
                        let mut entry = Vec::new();
                        entry.extend_from_slice(format!("{} {}\0", mode, filename).as_bytes());
                        entry.extend_from_slice(&hex::decode(&hash_out).unwrap());
                        tree_entries.push(entry);

                    } else if path.is_dir() {
                        let dir_name = entry.file_name().to_string_lossy();
                        let mode = "40000";
                        let sub_tree_hash = write_tree(&path); 
                        let hasher_data = format!("40000 {}\0", dir_name).into_bytes();
                        hasher_data.extend_from_slice(&hex::decode(&sub_tree_hash).unwrap());

                        let mut hasher = Sha1::new();
                        hasher.update(hasher_data.as_bytes());
                        let hash = hasher.finalize();
                        let hash_out = format!("{:x}", hash);
                        let dir = &hash_out[0..2];
                        let file_name = &hash_out[2..];
                        let object_dir = Path::new(".git/objects").join(dir);
                        let object_file = object_dir.join(file_name);
                        fs::create_dir_all(&object_dir).unwrap_or_else(|_|{
                            eprintln!("Error creating directory: {}", object_dir.display());
                            std::process::exit(1);
                        });
                        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                        encoder.write_all(hasher_data.as_bytes()).unwrap_or_else(|_|{
                            eprintln!("Error writing to zlib encoder");
                            std::process::exit(1);

                        });
                        let compressed_data = encoder.finish();
                        fs::write(&object_file, compressed_data.unwrap()).unwrap_or_else(|_|{
                            eprintln!("Error writing to object file: {}", &object_file.display());
                            std::process::exit(1);  
                        });
                        let entry_data =  format!("{} {}\0{}", mode, dir_name, hash_out);
                        tree_entries.push(entry_data);

                    } else {
                        eprintln!("Unsupported file type: {}", path.display());
                        std::process::exit(1);
                    }


                    }
                let mut tree_content = Vec::new();
                for entry in &tree_entries {
                    tree_content.extend_from_slice(entry);
                }
                let header = format!("tree {}\0", tree_content.len());
                let mut store = Vec::new();
                store.extend_from_slice(header.as_bytes());
                store.extend_from_slice(&tree_content);

                let mut hasher = Sha1::new();
                hasher.update(&store);
                let tree_hash = hasher.finalize();
                let tree_hash_hex = format!("{:x}", tree_hash);

               }
            }

            
        }
        
    }
}    




fn main() {
    
    let args: Vec<String> = env::args().collect();
    match Command::parse_args(&args) {
        Ok(command) => {
            command.excute();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

