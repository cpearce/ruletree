use std::slice;

struct Node {
    pub item: i32,
    pub children: Vec<Node>,
    pub rule_ids: Vec<i32>,
}

impl Node {
    pub fn new(item: i32) -> Node {
        Node{item: item, children: vec![], rule_ids: vec![]}
    }
    pub fn insert(&mut self, itemset: &[i32], rule_id: i32) {
        if itemset.len() == 0 {
            self.rule_ids.push(rule_id);
            return;
        }
        let index = self.child_index_mut(itemset[0]);
        self.children[index].insert(&itemset[1..], rule_id)
    }
    fn child_index_mut(&mut self, item: i32) -> usize {
        for i in 0..self.children.len() {
            if self.children[i].item < item {
                continue;
            }
            if self.children[i].item == item {
                return i
            }
            self.children.insert(i, Node::new(item));
            return i;
        }
        self.children.push(Node::new(item));
        self.children.len() - 1
    }
    pub fn matches(&self, itemset: &[i32], result: &mut Vec<i32>) {
        result.extend_from_slice(&self.rule_ids);
        let mut j = 0;
        for i in 0..itemset.len() {
            let item = itemset[i];
            while j < self.children.len() && self.children[j].item < item {
                j += 1;
            }
            if j == self.children.len() {
                break;
            }
            if self.children[j].item == item {
                self.children[j].matches(&itemset[i+1..], result);
            }
        }
    }
}

pub struct RuleTree {
    root: Node,
}

pub struct RuleMatches {
    rule_ids: Vec<i32>
}

impl RuleMatches {
    pub fn new(rule_ids: Vec<i32>) -> RuleMatches {
        RuleMatches {
            rule_ids
        }
    }
    pub fn len(&self) -> usize {
        self.rule_ids.len()
    }
    pub fn rule_id(&self, index: usize) -> i32 {
        self.rule_ids[index]
    }
}

fn is_sorted(itemset: &[i32]) -> bool {
    for i in 1..itemset.len() {
        if itemset[i-1] > itemset[i] {
            return false;
        }
    }
    true
}

impl RuleTree {
    fn new() -> RuleTree {
        RuleTree {
            root: Node::new(0),
        }
    }
    fn insert(&mut self, itemset: &[i32], rule_id: i32) {
        self.root.insert(itemset, rule_id);
    }
    fn matches(&self, itemset: &[i32]) -> RuleMatches {
        assert!(is_sorted(itemset));
        let mut rule_ids = vec![];
        self.root.matches(itemset, &mut rule_ids);
        RuleMatches::new(rule_ids)
    }
}

#[no_mangle]
pub extern "C" fn rule_tree_new() -> *mut RuleTree {
    let tree = Box::new(RuleTree::new());
    Box::into_raw(tree)
}

#[no_mangle]
pub extern "C" fn rule_tree_delete(tree: *mut RuleTree) {
    if tree.is_null() {
        panic!("rule_tree_delete passed a null tree!")
    }
    unsafe { Box::from_raw(tree) };
}

#[no_mangle]
pub extern "C" fn rule_matches_delete(matches: *mut RuleMatches) {
    if matches.is_null() {
        panic!("rule_matches_delete passed a null matches!")
    }
    unsafe { Box::from_raw(matches) };
}

#[no_mangle]
pub extern "C" fn rule_tree_insert(
    tree: *mut RuleTree,
    itemset_ptr: *const i32,
    itemset_len: usize,
    rule_id: i32,
) {
    if tree.is_null() {
        panic!("rule_tree_insert passed a null tree!")
    }
    if itemset_ptr.is_null() {
        panic!("rule_tree_insert passed null antecedent!");
    }
    let tree = unsafe { &mut *tree };
    let itemset = unsafe { slice::from_raw_parts(itemset_ptr, itemset_len) };
    tree.insert(itemset, rule_id);
}

#[no_mangle]
pub extern "C" fn rule_matches_len(
    matches: *const RuleMatches,
) -> usize {
    if matches.is_null() {
        panic!("rule_matches_len passed a null matches!")
    }
    let matches = unsafe { &*matches };
    matches.len()
}

#[no_mangle]
pub extern "C" fn rule_matches_element(
    matches: *const RuleMatches,
    index: usize,
) -> i32 {
    if matches.is_null() {
        panic!("rule_matches_len passed a null matches!")
    }
    let matches = unsafe { &*matches };
    matches.rule_id(index)
}

#[no_mangle]
pub extern "C" fn rule_tree_matches(
    tree: *const RuleTree,
    items_ptr: *const i32,
    items_len: usize,
) -> *mut RuleMatches {
    if tree.is_null() {
        panic!("rule_tree_matches passed a null tree!")
    }
    let tree = unsafe { &*tree };
    let itemset = unsafe { slice::from_raw_parts(items_ptr, items_len) };
    let matches = tree.matches(itemset);
    Box::into_raw(Box::new(matches))
}

#[cfg(test)]
mod tests {
    use super::RuleTree;
    #[test]
    fn it_works() {
        let mut tree = RuleTree::new();
        tree.insert(&[1,2,3,4], 1);
        tree.insert(&[2,3,4], 2);
        let m = tree.matches(&[1,2,3,4,5,6]);
        assert!(m.len() == 2)
    }
}
