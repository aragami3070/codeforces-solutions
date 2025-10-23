use crate::linked_list::LinkedList;

mod linked_list;

fn main() {
    let mut list = LinkedList::<u32>::new();

    list.push_back(32);
    list.push_back(52);
    list.push_back(72);
    list.push_front(2);

    list.insert(1, 911);

    println!("List len: {};", list.len());
    println!("{list:?};");

    for _ in 0..5 {
        let node_val = list.pop_back();
        if let Some(val) = node_val {
            println!("{val}")
        }
    }
    println!("{list:?};");
    list.push_front(2);
    list.push_front(52);
    list.push_front(2);
    println!("{list:?};");

    for _ in 0..1 {
        let node_val = list.pop_front();
        if let Some(val) = node_val {
            println!("{val}")
        }
    }
    println!("{list:?};");
}
