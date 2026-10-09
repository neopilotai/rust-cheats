use std::io::{self, Write};

fn main() {
    let mut query = String::new();
    print!("Search cheat sheets (empty to quit): ");
    let _ = io::stdout().flush();
    while io::stdin().read_line(&mut query).is_ok() {
        let query = query.trim();
        if query.is_empty() { break; }
        println!("Run `rust-cheats search {query}` to inspect results.");
        query = String::new();
        print!("Search cheat sheets (empty to quit): ");
        let _ = io::stdout().flush();
    }
}
