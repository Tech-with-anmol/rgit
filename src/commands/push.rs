use git2::{Cred, PushOptions, RemoteCallbacks, Repository};

pub fn push(remote_name: &str, branch: &str) {
    let repo = match Repository::open(".") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open repo: {}", e);
            return;
        }
    };

    let mut remote = match repo.find_remote(remote_name) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to find remote '{}': {}", remote_name, e);
            return;
        }
    };

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username, _| {
        if let Some(user) = username {
            println!("Using credentials for user: {}", user);
        } else {
            println!("Using default credentials");
        }
        Cred::ssh_key_from_agent(username.unwrap_or("git"))
    });
    callbacks.transfer_progress(|p| {
        println!("Progress: {} objects, {} bytes", p.received_objects(), p.received_bytes());
        true
    });

    let mut options = PushOptions::new();
    options.remote_callbacks(callbacks);

    let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
    match remote.push(&[&refspec], Some(&mut options)) {
        Ok(_) => println!("Successfully pushed to remote '{}'", remote_name),
        Err(e) => eprintln!("Failed to push to remote '{}': {}", remote_name, e),
    }
}

