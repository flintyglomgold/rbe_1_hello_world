fn main() {
    // So this example is mostly comments

    // println!("Hello, world!")

    /* It also reviews block comments!
    The example on Rust By Example has asterisks (*)
    running down here
    <------
    <------
    <------
    but it also points out that those aren't strictly necessary

    */

    // It also gives the example that you can manipulate
    // expressions with block comments inline

    // Check out the difference here:

    let x = 5 + /* 90 + */ 5;
    let y = 5 + 90 + 5;

    println!("x is equal to {}", x);
    println!("y is equal to {}", y);
}
