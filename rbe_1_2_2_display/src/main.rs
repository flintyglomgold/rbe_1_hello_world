// Import the fmt module
use std::fmt;

// Defining a structure for which `fmt::Display` will be implemented.
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// Ok the Rust By Examples page is very blase about this
// but I think this is cool as heck
// Anyway we're manually implementing the traid `fmt::Display` for our type.
impl fmt::Display for Structure {
    // this exact signature is required:
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        // Write strictly the first element into the supplied output
        // stream `f`. Returns `fmt::Result which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntac which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
        // ^^^comments verbatim from RBE.
        // write is like println but to f, not to stdout
        // reading further down, here's a crucial revelation:
        // `self` is the instantiated structure, therefore
        // `self.0` refers to the i32 inside the structure
    }
}

// Lower on down you have two more structs that will also
// have display implemented: MinMax and Point2D.
// Just gonna work through these with some small note-taking
// Big thing to note here is that Debug can be auto derived
// in nearly all cases.

#[derive(Debug)]
struct MinMax(i64, i64); //integers, not floating point

// Implement Display for this thing
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
// something from distant pre-history (circa 2014?) makes me
// think that I learned that the last line of a Rust function
// doesn't need a semicolon?

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

/* things I am noting:
1. ok this struct's got some named members
2. comma after the second member? is this optional?
*/

// Damn this is so cool
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// This is the activity portion-----
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    println!("Hello, world!");
    println!("Printing structs!");
    
    let foo = Structure(25);

    println!("Here's a number inside a Structure: {}", foo);

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    // I'm also just mentally making notes about examples here
    // of 'good' Rust style, e.g. the spaces after big_range to
    // align the right side of the assignment operators

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 }; // :, not =
    // also struct contructors switch to {} when using named members!
    // compare to instantiating big_range and small_range

    let numnum = Complex {real: 5.0, imag: 7.64 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("Complex numbers!");
    println!("Display: {}", numnum);
}

