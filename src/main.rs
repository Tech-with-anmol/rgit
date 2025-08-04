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
use std::path::Path;
use std::io::Write;
use sha1::{Sha1, Digest};

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
                let mut content = String::new();
                decoder.read_to_string(&mut content).unwrap_or_else(|e| {
                    eprintln!("Error reading from zlib decoder: {}", e);
                    std::process::exit(1);
                });
                let null_index = content.find('\0').unwrap_or_else(|| {
                    eprintln!("Error finding null char in content");
                    std::process::exit(1);
                });
                let content = content.get(null_index + 1..).unwrap_or_else(|| {
                    eprintln!("Error getting content after null char");
                    std::process::exit(1);
                });
                let lines: Vec<&str> = content.lines().collect();
                for line in lines {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let _object_type = parts.get(0).unwrap_or(&"");
                    let _object_sha = parts.get(2).unwrap_or(&"");
                    let object_name = parts.get(1).unwrap_or(&"");
                    println!("{}", object_name);

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

