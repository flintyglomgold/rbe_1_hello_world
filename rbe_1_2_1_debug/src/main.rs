
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
    // This is specifically for pretty-printing *debug* formatting

    // I get why they skip this, because the real meat is this one-two punch
    // 1. You *can* print anything as long as you annotate it with
    //        the debug trait, but it might look ugly, and if you want to
    //        implement a nice looking display you gotta do that manually
    // 2. Wait - trait? implement? derived trait? This looks like first
    //        steps into Rust's type system.
}
