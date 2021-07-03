
pub mod skip_list {
    pub mod node_link;
    use crate::skip_list::node_link::{Link, Node};

    pub struct SkipList{
        head : Link,
    }

    impl SkipList {
        pub fn new() -> Self {
            SkipList { head : Link::new(Node::new_head()), }
        }

        pub fn insert(&mut self, elem: i32) {
            let new_node_link = &mut Link::new(Node::new(elem));
            let mut level = self.get_skip_count();

            // Increase the height of the list as necessary
            for level in self.get_skip_count() + 1..=new_node_link.get_skip_height(){
                self.set_skip(new_node_link.clone(), level)
            }

            // start at the head node
            let mut target_skip = self.head.clone();

            while level >= 1 {

                if target_skip.is_some() &&
                    target_skip.get_skip(level).is_some() &&
                    target_skip.get_skip(level).get_elem() < elem {
                    // move right
                    target_skip = target_skip.get_skip(level).clone();
                } else {
                    // splice skip link
                    if new_node_link.get_skip_height() >= level {
                        target_skip.splice_skip_node(&mut new_node_link.clone(), level);
                    }

                    // move down
                    level = level - 1;
                }
            }

            // iterate over core list
            while target_skip.next().is_some() &&
                target_skip.next().get_elem() < elem {
                target_skip = target_skip.next().clone();
            }

            //let mut target = target_skip.upgrade();
            target_skip.splice_core_node(new_node_link);
        }

        pub fn delete(&mut self, elem: i32) {
            let mut level = self.get_skip_count();
            let mut search_skip = self.head.clone();

            // remove the skip links

            while level >= 1 {
                if search_skip.get_skip(level).is_some() {

                    if search_skip.get_skip(level).get_elem() == elem {
                        // remove located skip
                        search_skip.remove_skip_node(level);

                    } else if search_skip.get_skip(level).get_elem() < elem {
                        // move right
                        search_skip = search_skip.get_skip(level).clone();

                    } else {
                        // move down
                        level = level - 1;
                    }
                } else {
                    level = level - 1;
                }
            }

            // remove the core list

            while search_skip.next().is_some() {
                if search_skip.next().get_elem() < elem {
                    search_skip = search_skip.next().clone();
                } else {
                    if search_skip.next().get_elem() == elem {
                        search_skip.remove_core_node();
                    }
                    break;
                }
            }

        }

        pub fn contains(&self, elem: i32) -> bool {
            let mut found = false;

            let mut level = self.get_skip_count();
            let mut search_skip = self.head.clone();

            while level >= 1 {
                if search_skip.get_skip(level).is_some() {

                    if search_skip.get_skip(level).get_elem() == elem {
                        // found
                        found = true;
                        break;
                    } else if search_skip.get_skip(level).get_elem() < elem {
                        // move right
                        search_skip = search_skip.get_skip(level).clone();
                    } else {
                        // move down
                        level = level - 1;

                        if level >= 1 {
                            search_skip = search_skip.get_skip(level).clone();
                        }
                    }
                } else {
                    level = level - 1;
                }
            }

            // search the core list
            while !found && search_skip.next().is_some() {
                if search_skip.next().get_elem() < elem {
                    search_skip = search_skip.next().clone();
                } else {
                    if search_skip.next().get_elem() == elem {
                        found = true;
                    }
                    break;
                }
            }

            return found;
        }

        pub fn is_empty(&self) -> bool {
            !self.head.next().is_some()
        }

        fn set_skip(&mut self, skip_link: Link, level: usize) {
            self.head.set_skip(skip_link, level)
        }

        fn get_skip(&self, level: usize) -> Link {
            self.head.get_skip(level)
        }

        fn get_skip_height(&self) -> usize {
            self.head.get_skip_height()
        }

        fn get_skip_count(&self) -> usize {
            self.head.get_skip_count()
        }

    }

}

#[cfg(test)]
mod tests {
    use crate::skip_list::SkipList;

    #[test]
    fn ctor_test() {

        let mut skip_list = SkipList::new();

        assert_eq!(skip_list.is_empty(), true);
    }

    #[test]
    fn insert_delete_one_test() {

        let mut skip_list = SkipList::new();
        skip_list.insert(1);
        skip_list.delete(1);

        assert_eq!(skip_list.is_empty(), true);
    }

    #[test]
    fn invalid_delete_test() {

        let mut skip_list = SkipList::new();
        skip_list.delete(1);

        assert_eq!(skip_list.is_empty(), true);
    }

    #[test]
    fn insert_hundred_test() {

        let mut skip_list = SkipList::new();

        for i in 1..=100 {
            skip_list.insert(i);
        }

        assert_eq!(skip_list.contains(100), true);
    }
}
