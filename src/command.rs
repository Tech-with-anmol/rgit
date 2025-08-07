use crate::commands::{
    cat_file, help,commit_tree, hash_object, init, ls_tree, write_tree, clone, push
};

pub enum Command {
    Init,
    Help, 
    CatFile { sub_command: String, object: String },
    HashObject { file: String },
    LsTree { tree_sha: String },
    WriteTree,
    CommitTree { tree_sha: String, message: String, commit_sha: String  },
    Clone { url: String, directory: Option<String> },
    Push { remote: String, branch: String },
}

impl Command {
    pub fn parse_args(args: &[String]) -> Result<Self, String> {
        match args.get(1).map(|arg| arg.as_str()) {
            Some("init") => Ok(Self::Init),
            Some("help") => Ok(Self::Help),
            Some("commit-tree") if args.len() >= 4 => Ok(Self::CommitTree {
                tree_sha: args[2].clone(),
                commit_sha: args[4].clone(),
                message: args[6].clone(),
            }),
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
            Some("clone") if args.len() >= 3 => Ok(Self::Clone { 
                url: args[2].clone(),
                directory: args.get(3).cloned(),
            }),
            Some("push") if args.len() >= 3 => Ok(Self::Push {
                remote: args[2].clone(),
                branch: args[3].clone(),
            }),

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
            },
            Self::Help => help(), 
            Self::CommitTree { tree_sha, message, commit_sha } => {
                commit_tree(tree_sha, message, Some(commit_sha));
            },
            Self::Clone { url, directory } => {
                clone(url, directory.as_deref());
            },
            Self::Push { remote, branch } => {
                push(remote, branch);
            }
            
        }
    }
}

