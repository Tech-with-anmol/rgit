use git2::Repository;

pub fn clone(url: &str, directory: Option<&str>) {
    let _repo = match directory {
        Some(dir) => {
            match Repository::clone(url, dir) {
                Ok(repo) => repo,
                Err(e) => {
                    eprintln!("Failed to clone repository to {}: {}", dir, e);
                    return;
                }
            }
        },
        None => {
            match Repository::clone(url, ".") {
                Ok(repo) => repo,
                Err(e) => {
                    eprintln!("Failed to clone repository to current directory: {}", e);
                    return;
                }
            }
        },
    };
}
