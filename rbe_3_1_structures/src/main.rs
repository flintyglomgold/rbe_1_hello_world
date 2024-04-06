#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct AbsoluteUnit; // just a beefy lad

// A tuple struct
struct Pair(i32, f32); // fields not named

// A struct with two named fields
struct Point{
    x: f32,
    y: f32,
}

// You're never gonna guess whether structs can be used as fields
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    // not let name = "Peter", that's not a String
    // that's a string slice.
    // String::from("blah") is how you get a String
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // point instatiation
    // like my old speech and debate coach
    // "HEY GLOMGOLD INSTANTIATE YOUR POINT BRO"
    let point: Point = Point { x: 10.3, y: 0.4 };
    // just to run that back that's
    //    ^       ^      ^
    //   name    type   constructor

    // Make a new point by using struct update syntax
    // to use the fields of our othe one
     let bottom_right = Point { x: 5.2, ..point }; //wild
     // those two decimal points really do let you just
     // say "and everything else is the same as blorp"

     println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

     // Destructuring the point using a `let` binding
     let Point { x: left_edge, y: top_edge } = point;

     let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    
    let _unit = AbsoluteUnit;

    let pair = Pair (1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}