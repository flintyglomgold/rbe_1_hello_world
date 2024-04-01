    // A brief discussion of the different formatting macros!
    // format!, print!, println!, eprint!, and eprintln!
    // writing formatted text to string, writing it to stdout and to stderr
    // plus or minus a new line appended

fn main() {

    println!("Hello, world!");

    // The big block of text has some other interesting stuff in it tho
    // So there's formatting strings like you'd expect:

    println!("This is a {} {}.", "format", "string");
    println!("This is {2} a {1} {0}.", "string", "format", "also");

    // But also you can name the arguments in a format string
    // I think this is very cool

    println!("This is a {thing} with {bonus}!", thing="format string",
        bonus="named arguments");

    print!("\n"); // keeping output formatting nice

    println!("Here's some cool number formatting you can do in Rust:");

    // You can format numbers into different bases!
    let x = 234;
    println!("Base 10: {}", x);
    println!("Base 2: {:b}", x);
    println!("Base 8: {:o}", x);
    println!("Base 16: {:x}", x);

    print!("\n"); // keeping output formatting nice

    // There's some number padding stuff here which is cool
    let y = 13;
    println!("This is a string printed with a specified width:");
    println!("{number:>5}", number=y);

    println!("Here's some number padding with zeroes!");
    println!("{number:0>5}", number=y);

    println!("Or padding zeroes at the end: this is the same variable!");
    println!("{number:0<width$}", number=y, width=5);
    println!("That one also had a variable width!");

    // A quick example: Rust checks that the correct number of args are used
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Here is an example that you can uncomment and see how it won't compile
    // #[allow(dead_code)]
    // struct Structure(i32);
    // println!("This struct '{}' won't print", Structure(3));

    // It won't compile because `Structure` doesn't implement fmt::Display
    // which is necessary to be printed

    // new to Rust 1.58 (we are at 1.77.1 as of this writing)
    // is capturing the argument from the variable nearby

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // note that the format string has named arguments that are not specified
    // at the end of the format string itself

    // Exercise left to the reader:
    // Add a println macro call that controls the number of decimal places!
    let pi = 3.14159265358979;
    println!("Here's pi to 3 digits after the decimal!");
    println!("Pi is {number:.*}", 3, number=pi);

    println!("And again!");
    println!("Pi is {pi:.*}", 3);
    // note that this format string, like the f64 example above, captures
    // the context so that pi isn't needed in the additional arguments

}
