// condensing the following into one exercise .rs file:
// 3.2.1 use
// 3.2.2 C-like
// 3.2.3 Testcase: linked-list
// many enum shenanigans in one place -- how apropos
#![allow(dead_code)]
use crate::List::*;

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// an enum with an explicit discriminator
enum Color {
    Red   = 0xff0000,
    Green = 0x00ff00,
    Blue  = 0x0000ff,
}

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        Nil
    }

    // Consume a list and return the same list with a new element at its start
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self)) // cute
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref rail
        // see https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        //
        // did you get all that?
        // me neither
        match *self {
            // can't take ownership of the fail, because `self` is borrowed;
            // instead take a reference to the tail (unless you're using Rust 2018 or later)
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a heap allocated string
    fn stringify(&self) -> String {
        match * self {
            Cons(head, ref tail) =>{
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // The `use` declaration can be used so manual scoping isn't needed:
    use crate::Status::{Poor, Rich}; // out of order, no less
    use crate::Work::*; // or just use everything!

    // Equivalent to Status::Poor
    let status = Poor;
    // Equivalent to Work::Civilian
    let work = Civilian;

    match status {
        //still no scoping because we used `use`
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }

    println!("zero is {}", Number::Zero as i32 ); // noting `as` keyword here
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

