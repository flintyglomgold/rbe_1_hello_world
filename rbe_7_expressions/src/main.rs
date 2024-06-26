// a brief discussion of expressions

fn main() {

    // variable binding
    let x = 5;

    // expression;
    /*
    x;
    x + 1;
    15;
    */
    // These lines will produce warnings but are technically
    // valid, compileable code.

    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        // This expression will be assigned to `y`
        x_cubed + x_squared + x
    };

    let z = {
        // The semicolon supresses this expression, and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
