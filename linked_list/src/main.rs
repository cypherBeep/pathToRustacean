use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    list.push_front('a');
    list.push_front('c');
    list.push_front('f');
    list.push_front('v');
    list.push_front('h');
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
