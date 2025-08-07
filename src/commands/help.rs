use colored::Colorize; 

pub fn help() {
    println!(r#" 
     
         _           _               _        _          _     
        /\ \        /\ \            /\ \     /\ \       /\_\   
       /  \ \      /  \ \           \ \ \    \_\ \     / / /   
      / /\ \ \    / /\ \_\          /\ \_\   /\__ \   / / /_   
     / / /\ \_\  / / /\/_/         / /\/_/  / /_ \ \ / /___/\  
    / / /_/ / / / / / ______      / / /    / / /\ \ \\____ \ \ 
   / / /__\/ / / / / /\_____\    / / /    / / /  \/_/    / / / 
  / / /_____/ / / /  \/____ /   / / /    / / /          / / /  
 / / /\ \ \  / / /_____/ / /___/ / /__  / / /          _\/_/   
/ / /  \ \ \/ / /______\/ //\__\/_/___\/_/ /          /\_\     
\/_/    \_\/\/___________/ \/_________/\_\/           \/_/     
                                                               

    "#);
    println!("{}", "Usage: rgit <command> [options]".green());
    println!("{}", "\nAvailable commands:".green());
    println!("  init                Initialize a new repository");
    println!("  help                Show this help message");
    println!("  cat-file <subcmd> <object>  Display information about an object");
    println!("  hash-object -w <file>       Hash a file and store it inline");
    println!("  ls-tree --name-only <tree>  List the contents of a tree");
    println!("  write-tree                   Write the current index to a tree object");
    println!("\nFor more information on a specific command, use 'rgit help <command>'.");
}
