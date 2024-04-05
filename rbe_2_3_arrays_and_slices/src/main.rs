use std::mem;

// borrowing a hot slice
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}


fn main() {
    // Fixed-size array (type sig is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 750] = [0; 750]; // semicolon, noted

    // indexing starts at zero - Rust changes a lot but isn't buck wild
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // len returns the count of elements in the array - perhaps the length?
    println!("Number of elements in the array: {}", xs.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    // HEY SOMEONE BETTER BE CHECKING ALL THESE BORROWS
    // FEEL LIKE I'VE HEARD ABOUT THIS SOMEWHERE BEFORE
    println!("Borrow the whole array as a slice!");
    analyze_slice(&xs);

    // empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]); // cool macro
    assert_eq!(&empty_array, &[][..]); // same but more verbose

    // Arrays can be safely accessed using `.get` which returns an
    // `Option`. HEY THIS IS SOME HASKELL NONSENSE This can be matched
    // as shown below, or used with `.expect()` if you would like the
    // program to exit with a nice message instead of happily continue
    for i in 0..xs.len() + 1 { // how will i hold all these limes i mean elements
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval ),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on an array causes a compile time error.
    // println!("{}", xs[5]);
    // Out of bound indexing on a slice causes a runtime error
    // println!("{}", xs[..][5]);
    // it is not clear to me how this second thing is a slice
    // but I trust that the `..` and the two-dimensionality of this
    // array will become relevant when I understand slices, arrays,
    // and indexes more
}
