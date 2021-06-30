use simple_skiplist::skip_list::SkipList;

fn main() {
    let mut skip_list = SkipList::new();

    skip_list.insert(1);
    skip_list.insert(2);
    skip_list.insert(3);
    skip_list.delete(3);

    println!("UnExpected Found: {}", skip_list.contains(3));

}