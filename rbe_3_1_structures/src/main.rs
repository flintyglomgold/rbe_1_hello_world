#![allow(dead_code)]

use std::fmt;

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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// You're never gonna guess whether structs can be used as fields
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(input_rect: Rectangle) -> f32 {
    
    let Rectangle { top_left: upper_left, bottom_right: lower_right} = input_rect;
    let Point { x: x1, y: y1 } = upper_left;
    let Point { x: x2, y: y2 } = lower_right;
    (x2 - x1) * (y2 - y1)

    // an example of nested destructuring found on reddit from /u/connorcpu:
    /*
    let Rectangle { top_left: Point { x: x1, y: y1},
                    bottom_right: Point { x: x2, y: y2}} = input_rect;
    (x2 - x1) * (y2 - y1)

    his example also uses the `ref` keyword which I have not come across yet
    THE BORROW CHECKER LURKS IN THE DISTANCE

    */
}

fn square(input_point: Point, len: f32) -> Rectangle {
    let second_point = Point { x: input_point.x + len,
                               y: input_point.y + len };

    let my_square = Rectangle { top_left: input_point,
                                bottom_right: second_point };

    my_square
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

    let fresh_point = Point { x: 2.0, y: 4.5 };
    let second_fresh_point = Point { x: 5.0, y: 9.0 };

    let new_rectoblebobble = Rectangle {
        top_left: fresh_point,
        bottom_right: second_fresh_point,
    };

    let new_point = Point { x: 1.0, y: 1.0 };

    println!("Top left is {}, {}", new_rectoblebobble.top_left.x, new_rectoblebobble.top_left.y );

    println!("Bottom right is {}, {}", new_rectoblebobble.bottom_right.x, new_rectoblebobble.bottom_right.y );

    println!("The area of rectangle is {}", rect_area(new_rectoblebobble));

    println!("The area of a square of length {} is {}",
        5, rect_area(square(new_point, 5.0)));
    // I didn't cite where the new point is because I was
    // BLINDED BY THE LIGHT (of the borrow checker)
    // putting new_point in println() and also sending it in a function
    // is frowned upon. I will learn why in time.
}