use simple_skiplist::skip_list::SkipList;

fn main() {
    let mut skip_list = SkipList::new();

    for insert in 1..=100 {
        skip_list.insert(insert);
    }

    for delete in 1..=100 {
        skip_list.delete(delete);
    }

    if skip_list.is_empty() {
        println!("empty as expected");
    } else {
       println!("error, not empty");
    }


}