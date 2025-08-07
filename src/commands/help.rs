use colored::Colorize;
use colored::Color;



fn rainbow_ascii(ascii: &str) {
    let colors = [
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Cyan,
        Color::Blue,
        Color::Magenta,
    ];

    for (i, line) in ascii.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let color = colors[(i + j) % colors.len()];
            print!("{}", c.to_string().color(color));
        }
        println!();
    }
}


pub fn help() {
    let art = r#" 
     
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
                                                               

    "#;
    let _art2 = r#"
    
░▒▓███████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░▒▓████████▓▒░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░ 
░▒▓███████▓▒░░▒▓█▓▒▒▓███▓▒░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░  ░▒▓█▓▒░           
░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░ 
                                                     
                        
        "#;
    rainbow_ascii(art);
    println!("{}", "Usage: rgit <command> [options]".cyan());
    println!("{}", "\nAvailable commands:".green());
    println!("  init                Initialize a new repository");
    println!("  help                Show this help message");
    println!("  cat-file <subcmd> <object>  Display information about an object");
    println!("  hash-object -w <file>       Hash a file and store it inline");
    println!("  ls-tree --name-only <tree>  List the contents of a tree");
    println!("  write-tree                   Write the current index to a tree object");
    println!("  commit-tree <tree_sha> -p <commit_sha -m <message>    commit to a tree");
    println!("{}", "\nFor more information on a specific command, use 'rgit help <command>'.".yellow());
}
