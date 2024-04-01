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

fn main() {
    println!("Hello, world!");
    println!("Printing structs!");
    
    let foo = Structure(25);

    println!("Here's a number inside a Structure: {}", foo);
}
