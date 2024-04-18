fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`
            // notive that this `if` is on the right side of a let binding
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            // This expression must return an `i32` as well
            n / 2
        }; // all let bindings need a semicolon!

    println!("{} -> {}", n, big_n);

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three!");
            continue;
        }
        
        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough!");
            break;
        }
    }

    // Nesting and labels

    'outer: loop {
        println!("Entered the outer loop!");

        'inner: loop {
            println!("Entered the inner loop!");

            // This would break only the iner loop
            // break

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
        // and not only will it never be reached,
        // the rust compiler *warns* you of this!
        // very impressive
    }

    println!("Exited the outer loop");

    // and you can return results from loops by putting the variable after
    // the break statement

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // getting used to while with a fizzbuzz example

    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    // and now for something completely different

    for a in 1..101 { // alternatively for a in 1..=100
        if a % 15 == 0 {
            println!("fizzbuzz");
        } else if a % 3 == 0 {
            println!("fizz");
        } else if a % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", a);
        }
    }

}
