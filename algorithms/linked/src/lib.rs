// Define an enum for address node
#[derive(Clone)]
pub enum address {
    address(Box<myList>),
    Nil,
}

// Define List Structure
#[derive(Clone)]
pub struct myList {
    value: u32,
    next: address,
}

// CRUD Methods for linked list
impl myList {

    // add an node to the list
    pub fn append(&mut self, elem: u32) {
        match self.next {
            address::address(ref mut next_address) => {
                next_address.append(elem);
            }
            address::Nil => {
                let node = myList {
                    value: elem,
                    next: address::Nil,
                };
                self.next = address::address(Box::new(node))
            }
        }
    }

    // delete a node from the list
    pub fn delete(&mut self, elem: u32) {
        match self.next {
            address::address(ref mut next_address) => {
                if next_address.value == elem {
                    println!("Deleting value {}", next_address.value);
                    self.next = next_address.next.clone();
                } else {
                    next_address.delete(elem);
                }
            }
            address::Nil => {
                if self.value == elem {
                    self.value = 0;
                } else {
                    println!("Element {} does not exist in the linked list", elem);
                }
            }
        }
    }

    // count the size of the list
    pub fn count(&self) -> u32 {
        match self.next {
            address::address(ref newaddress) => 1 + newaddress.count(),
            address::Nil => 0,
        }
    }
    pub fn list(&self) {
        if self.value == 0 {
            println!("The list is empty")
        } else {
            println!("{}", self.value);
            match self.next {
                address::address(ref next_address) => next_address.list(),
                address::Nil => {}
            }
        }
    }

    // update an item in the linked list
    pub fn update(&mut self, index: u32, elem: u32) {
        let mut i = 0;
        let mut j = self;
        if i == index {
            j.value = elem;
        }
        while i < index {
            // println!("{}", j.value);
            match j.next {
                address::address(ref mut next_address) => j = next_address,
                address::Nil => {}
            }
            i = i + 1;
        }
        j.value = elem;
    }
}