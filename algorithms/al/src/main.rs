use linked::myList;
use linked::address;
use linked::append;

fn main() {
    let mut head = myList {
        value: 8,
        next: address::Nil,
    };
    head.append(9);
    head.append(10);
    head.append(11);
    head.list();
    println!("The size of the list is {}", head.count());
    head.update(4, 20);
    head.update(0, 6);
    head.list();
   
}
