extern crate git2;

use git2::*;

// Iterates through all leaf nodes in a depth-first traversal.
pub struct TreeWalker<'t, 'r> {
    it_stack: Vec<TreeWithIter<'t>>, // Used as a stack for depth-first traversal
    repo : &'r Repository, // Needed for converting tree entries to objects
}

// For each subtree we're currently visiting, we need the subtree itself
// and an iterator to pass through it.
struct TreeWithIter<'t> {
    tree: Tree<'t>,
    it: TreeIter<'t>,
}


impl<'t, 'r> Iterator for TreeWalker<'t, 'r> {
    type Item = TreeEntry<'t>;

    fn next(&mut self) -> Option<TreeEntry<'t>> {
        // Perform a classic depth-first traversal of the tree.
        loop {
            let next;
            {
                // If we're out of iterators, we're done.
                let back_iterator = self.it_stack.last_mut();
                if back_iterator.is_none() { return None; }

                // Otherwise, grab the back one's next element
                let back_iterator = back_iterator.unwrap();
                next = back_iterator.it.next();
            }

            let next = match next {
                // If this iterator is empty, pop it.
                None => { self.it_stack.pop(); continue; },
                // Otherwise, continue with its element.
                Some(e) => e
            };

            if next.kind().unwrap() == ObjectType::Tree {
                // If it's a tree, push it onto the stack and start walking it.
                // HERE BE DRAGONS: TreeElement::to_object gives us an object
                // with the lifetime of the repo
                let subtree = match next.to_object(&self.repo)
                    .unwrap()
                    .into_tree() {
                    Ok(t) => t,
                    Err(_) => panic!("Couldn't unwrap TreeEntry to Tree")
                };

                self.it_stack.push(TreeWithIter{ tree: subtree, it: subtree.iter()});
                continue;
            }
            else {
                // Otherwise we can return it as usual.
                return Some(next);
            }
        }
    }
}

pub fn walk<'t, 'r>(t: Tree<'t>, r: &'r Repository) -> TreeWalker<'t, 'r> {
    TreeWalker{ it_stack: vec![TreeWithIter{ tree: t, it: t.iter()}], repo: r }
}

// We get the following error:
/*
src/tree.rs:46:42: 46:51 error: cannot infer an appropriate lifetime for lifetime parameter 'a in function call due to conflicting requirements [E0495]
src/tree.rs:46                 let subtree = match next.to_object(&self.repo)
                                                        ^~~~~~~~~
src/tree.rs:21:5: 61:6 help: consider using an explicit lifetime parameter as shown: fn next(&mut self) -> Option<TreeEntry<'r>>
*/
// Relevant docs:
// http://alexcrichton.com/git2-rs/git2/struct.TreeEntry.html
// http://alexcrichton.com/git2-rs/git2/struct.Object.html
// http://alexcrichton.com/git2-rs/git2/struct.Tree.html
