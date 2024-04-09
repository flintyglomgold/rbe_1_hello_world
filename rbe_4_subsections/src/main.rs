// A compiled .rs file containing all the subsections
// for Rust By Example Section 4

// 4.1 Mutability
// 4.2 Scope and Shadowing
// 4.3 Declare first
// 4.4 Freezing

fn main() {

    // MUTABILITY

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // This is an error as this variable is immutable:
    // _immutable_binding += 1;

    // SCOPE AND SHADOWING

    // these binding is in the main function
    let long_lived_binding = 1;
    let shadowed_binding = 1;

    // this block has a smaller scope than main
    {
        // this binding only exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    {
        println!("before being shadowed: {}", shadowed_binding);
        let shadowed_binding = "abc";
        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}
