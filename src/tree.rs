extern crate git2;

use git2::*;

pub fn prefix(root: &str, end: &str) -> String {
    if  root.is_empty() { end.to_string() }
    else { [root, end].join("/").to_string() }
}

fn walk_recursor<F>(t: Tree, root: &str, repo: &Repository, visitor: &F)
    where F: Fn(&str, &TreeEntry)
{
    // Perform a classic, recursive depth-first traversal of the tree.
    for te in &t {
        let element_name = te.name().unwrap();
        let full_name = prefix(root, element_name);

        if te.kind().unwrap() == ObjectType::Tree {

            let subtree = match te.to_object(repo)
                .unwrap()
                .into_tree() {
                Ok(t) => t,
                Err(_) => panic!("Couldn't unwrap TreeEntry to Tree")
            };

            walk_recursor(subtree, &full_name, repo, visitor);
        }
        else {
            visitor(root, &te);
        }
    }
}

pub fn walk<F>(t : Tree, repo: &Repository, visitor: F)
    where F: Fn(&str, &TreeEntry)
{
    walk_recursor(t, "", repo, &visitor)
}
