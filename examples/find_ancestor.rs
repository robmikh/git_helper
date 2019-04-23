extern crate git_helper;

use std::env;
use git_helper::GitHelper;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let directory = &args[1];
    let pattern = &args[2];

    let git = GitHelper::create_from_dir(&directory);
    match git.find_ancestor_with_name(&pattern, 15) {
        Some(name) => println!("Found it! {}", name),
        _ => println!("Nope!"),
    }
}