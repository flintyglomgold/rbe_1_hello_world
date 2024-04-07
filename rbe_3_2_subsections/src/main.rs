// condensing the following into one exercise .rs file:
// 3.2.1 use
// 3.2.2 C-like
// 3.2.3 Testcase: linked-list
// many enum shenanigans in one place -- how apropos
#![allow(dead_code)]

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
}

