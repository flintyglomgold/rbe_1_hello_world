// A brief overview of scalar types
// I note that chars are 4 bytes each.
// Noting also the unit type ()

fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer   = 5i32;

    let default_float   = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type = 4294967296i64; // ok this inferred type business is wild

    // learning about mutability
    let mut mutable = 12;
    mutable = 21;

    // mutable = true; // can't do this

    let mutable = true; // can do this
    // this is called shadowing, apparently
}
