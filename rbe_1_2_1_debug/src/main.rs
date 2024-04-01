// ignore this arcane magic
#[derive(Debug)]
struct Structure(i32);

// ok don't ignore it
// it's just asking a struct to derive the Debug implementaion

// let's do it again!

#[derive(Debug)]
struct Deep(Structure);

// and this code will be referenced at the end of main:

#[derive(Debug)]
struct Person<'a> { //no idea what these ticks mean. I'll find out, I guess.
    name: &'a str,
    age: u8
}

fn main() {
    // the note is that {:?} is similar to {}
    // but how?
    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name",
        "Slater",
        "Christian",
        actor="actor's");

    // the answer is in the main Rust documentation:
    // https://doc.rust-lang.org/std/fmt/
    // In the docs for the std::fmt module:
    // {:?} is another modified for formatting like binary and hex
    // This is specifically for printing *debug* formatting

    // I get why they skip this, because the real meat is this one-two punch
    // 1. You *can* print anything as long as you annotate it with
    //        the debug trait, but it might look ugly, and if you want to
    //        implement a nice looking display you gotta do that manually
    // 2. Wait - trait? implement? derived trait? This looks like first
    //        steps into Rust's type system.

        // From what little I know about Rust I'm just glad the did it this way
    // intead of trying to untangle strings. Anyway let's print stuff!

    println!("Now {:?} will print!", Structure(3));

    // note the format string for printing Debug formatting

    // anyway this code runs with a warning

    println!("Also {:?} will print!", Deep(Structure(7)));

    // and in an earlier commit I said that this `{:?}` was pretty printing.
    // it's not! It's just debug printing. THIS: `{:#?}` is pretty printing!

    // Let's try it out!

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age }; // defined above main

    // Pretty print! 
    println!("{:#?}", peter);

}
