use crate::commands::{
    cat_file, hash_object, init, ls_tree, write_tree,
};

pub enum Command {
    Init,
    CatFile { sub_command: String, object: String },
    HashObject { file: String },
    LsTree { tree_sha: String },
    WriteTree,
}

impl Command {
    pub fn parse_args(args: &[String]) -> Result<Self, String> {
        match args.get(1).map(|arg| arg.as_str()) {
            Some("init") => Ok(Self::Init),
            Some("cat-file") if args.len() >= 4 => Ok(Self::CatFile {
                sub_command: args[2].clone(),
                object: args[3].clone(),
            }),
            Some("hash-object") if args.len() >= 4 && args[2] == "-w" => Ok(Self::HashObject {
                file: args[3].clone(),
            }),
            Some("ls-tree") if args.len() >= 4 && args[2] == "--name-only" => Ok(Self::LsTree {
                tree_sha: args[3].clone(),
            }),
            Some("write-tree") => Ok(Self::WriteTree),
            Some(cmd) => Err(format!("Unknown command: {}", cmd)),
            None => Err("No command provided".into()),
        }
    }

    pub fn execute(&self) {
        match self {
            Self::Init => init(),
            Self::CatFile { sub_command, object } => cat_file(sub_command, object),
            Self::HashObject { file } => hash_object(file),
            Self::LsTree { tree_sha } => ls_tree(tree_sha),
            Self::WriteTree => {
                let tree_sha = write_tree(std::path::Path::new("."));
                println!("{}", tree_sha);
            }
        }
    }
}

