use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::cmp::Ordering;
use std::mem;

pub struct Node {
    elem: i32,
    skip_links: Vec<Link>,
    core_list: Link,
}

pub struct Link(Option<Rc<RefCell<Node>>>);

// ----------------------------------------------------------------------------
// Node
// ----------------------------------------------------------------------------

impl Node {
    pub fn new(elem: i32) -> Self {
        if fastrand::bool() {
            // This node will have SkipLinks of a random number up to MAX_LEVEL
            let mut target_height : usize = 1;
            for _ in 1..crate::MAX_LEVEL {
                if fastrand::bool() {
                    target_height = target_height + 1;
                } else {
                    break;
                }
            }
            Node { elem: elem, core_list: Link::new_empty(), skip_links : vec![Link::new_empty(); target_height as usize], }
        } else {
            Node { elem: elem, core_list: Link::new_empty(), skip_links : vec![Link::new_empty(); 0], }
        }
    }

    pub fn new_head() -> Self {
        Node { elem: i32::MIN, core_list: Link::new_empty(), skip_links : vec![Link::new_empty(); crate::MAX_LEVEL as usize], }
    }

    pub fn get_skip(&self, level: usize) -> Link {
        self.skip_links[level - 1].clone()
    }

    pub fn set_skip(&mut self, skip_link : Link, level : usize)  {
        self.skip_links[level - 1] = skip_link.clone();
    }

    pub fn has_skips(&self) -> bool {
        self.skip_links.capacity() > 0
    }

    pub fn get_skip_height(&self) -> usize {
        if !self.has_skips() {
            0
        } else {
            // todo compute based on is_some
            self.skip_links.len()
        }
    }

    pub fn get_skip_count(&self) -> usize {
        if !self.has_skips() {
            0
        } else {
            // todo compute based on is_some
            self.skip_links.iter().filter(|x| x.is_some()).count()
        }
    }

    pub fn next(&self) -> Link {
        self.core_list.clone()
    }

    pub fn next_skip(&self, bounds: i32, level: usize) -> Link {
        let skip = self.get_skip(level);

        if skip.is_some() {
            if skip.elem > bounds {
                return Link::new_empty();
            }
        }

        skip
    }

    pub fn get_elem(&self) -> i32 {
        self.elem
    }

    pub fn splice_skip_node(&mut self, new_node_link: &mut Link, level: usize) {

        if self.get_skip(level).is_some() {
            let old_node = mem::replace(&mut self.skip_links[level - 1],
                                        Link::new_empty());
            new_node_link.set_skip(old_node,level);
        }

        self.set_skip(new_node_link.clone(), level);
    }

    pub fn remove_skip_node(&mut self, level: usize) {
        let mut replace = Link::new_empty();
        let next = self.get_skip(level);

        if next.get_skip(level).is_some() {
            replace = next.get_skip(level);
        }

        self.set_skip(replace.clone(), level);
    }

    pub fn splice_core_node(&mut self, new_node_link: &mut Link) {

        if  self.core_list.is_some() {
            let old_node = mem::replace(&mut self.core_list,
                                        Link::new_empty());

            new_node_link.core_list = old_node.clone();
        }

        self.core_list = new_node_link.clone();

    }

    pub fn remove_core_node(&mut self) {

        let mut replace = Link::new_empty();

        if  self.core_list.is_some() &&
            self.core_list.core_list.is_some() {
            replace = self.core_list.core_list.clone();
        }

        self.core_list = replace;
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.elem.cmp(&other.elem)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.elem == other.elem
    }
}

impl Eq for Node {}

// ----------------------------------------------------------------------------
// Link
// ----------------------------------------------------------------------------

impl Link {
    pub fn new(node: Node) -> Link {
        Link(Some(Rc::new(RefCell::new(node))))
    }

    pub fn new_from_rc(rc: Rc<RefCell<Node>>) -> Link {
        Link(Some(rc))
    }

    pub fn new_empty() -> Link {
        Link(None)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }
}

impl Clone for Link {
    fn clone(&self) -> Self {
        if let Some(rc) = &self.0 {
            Link(Some(Rc::clone(rc)))
        } else {
            Link(None)
        }
    }
}

impl Deref for Link {
    type Target = Node;

    fn deref(&self) -> &Node {

        if let Some(n1) = &self.0 {
            unsafe { // wth
                let n2 = n1.as_ptr();
                let n3 = &*n2;
                n3
            }
        } else {
            panic!("this is a terrible mistake!");
        }
    }
}

impl DerefMut for Link {

    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Some(n1) = &self.0 {
            unsafe { // wth
                let n2 = n1.as_ptr();
                let n3 = &mut *n2;
                n3
            }
        } else {
            panic!("this is a terrible mistake!");
        }
    }
}

