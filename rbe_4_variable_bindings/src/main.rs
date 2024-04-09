fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", an_integer);
    println!("An copied integer: {:?}", copied_integer);
    // haven't angered the borrow checker
    // YET
    println!("A bolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variables, but these
    // warnings can be silenced by yelling at your screen
    // alternatively, prefixing the variable name with `_`

    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
}
