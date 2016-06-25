extern crate git2;

use git2::*;

use std::env;
//use std::iter::IntoIterator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let repo = Repository::open(&args[1]).expect("Couldn't open repo");

    // Find the master branch
    let master_ref = repo.find_reference("refs/heads/master").unwrap();

    // Go from reference -> commit
    let master_commit = match master_ref.peel(ObjectType::Commit)
        .unwrap()
        .into_commit() {
        Ok(commit) => commit,
        Err(_) => panic!("Couldn't resolve to a commit")
    };

    let master_tree = master_commit.tree().unwrap();

    // Iterate through its tree
    for te in &master_tree {
        let entry_name = te.name().unwrap();
        println!("{}", entry_name);
    }
}
