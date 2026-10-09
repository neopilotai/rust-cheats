use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Search cheat sheets (empty to quit): ");
    let _ = io::stdout().flush();
    while io::stdin().read_line(&mut input).is_ok() {
        let query = input.trim();
        if query.is_empty() { break; }
        println!("Run `rust-cheats search {query}` to inspect results.");
        input.clear();
        print!("Search cheat sheets (empty to quit): ");
        let _ = io::stdout().flush();
    }
}
