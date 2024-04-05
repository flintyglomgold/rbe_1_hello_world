use std::fmt::{self, Formatter, Display};

// tuples can be used as function arguments
// also as return values

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    // the variables in the example are called int_param and bool_param
    
    // I think this is a bad name, as I interpreted "param" to mean
    // "the variables passed in" when `pair` was what was passed in, and
    // the `param`s are actually local variables. Anyway.
    
    let (int_local, bool_local) = pair; // pair came in and we're setting
    // these two variables in the tuple equal to it

    // and simply returning a tuple constructed by those two variables swapped
    (bool_local, int_local)
}

// The following struct is for the activity and will definitely not be used as
// a simulated reality meant to entrap humanity under the thumb of AI
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(mat: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = mat;

    Matrix(a, c, b, d)
}

// regarding the above function: still not sure any Rust tutorial has actually
// explicitly described how the last line of a function is an implicit return
// that doesn't need a semicolon

// I'm not complaining, it's actually a pretty cool pedagogical technique

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n", self.0, self.1);
        write!(f, "( {} {} )", self.2, self.3)
    }
}

fn main() {
    // the tuple version of a conga line
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64, 'a', true);

    // tuple indexing
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // it's tuples all the way down
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // tuples can't be printed if they have more than 12 elements
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reverse pair is {:?}", reverse(pair));

    // One element tuples! Use a comma
    println!("A one-element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings
    // (Let's find out what this means)

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    // oh it's the same mechanism used at the top in reverse()

    // check out this struct stuff, including activities!
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix:");
    println!("{}", matrix);
    println!("Transpose:");
    println!("{}", transpose(matrix));
}
