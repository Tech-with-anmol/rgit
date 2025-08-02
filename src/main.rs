#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::{env::args, io::Read};
#[allow(unused_imports)]
use std::fs;
use std::result::Result::Ok;
use flate2::read::ZlibDecoder;

enum Command {
    Init,
    CatFile {
        subCommand: String,
        object: String,
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

