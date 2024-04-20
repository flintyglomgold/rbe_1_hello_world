fn main() {

    // match

    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // match on a single value
        1 => println!("One!"),
        // match on multiple values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime!"),
        // inclusive range
        13..=19 => println!("A teen"),
        //everything else
        _ => println!("Ain't special."),
    }

    let boolean = true;

    let binary = match boolean {
        // the arms of a match must be comprehensive
        false => 0,
        true  => 1,
    };

    println!("{} -> {}", boolean, binary);

    // destructuring tuples

    let triple = (0, -2, 2);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z)  => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)    => println!("First is `1` and the rest doesn't matter"),
        (.., 2)    => println!("Last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest don't matter."),
        _          => println!("It doesn't matter what they are."),
    }

    // destructuring arrays

    let array = [2, -2, 6];

    match array {
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, and array[2] = {}", second, third
        ),

        [1, _, third]      =>
            println!("array[0] = 1, array[2] = {}, and array[1] was ignored", third
        ),

        [-1, second, ..]   =>
            println!("array[0] = -1, array[1] = {}, and all others were ignored", second
        ),

        [3, second, tail @ ..] =>
            println!("array[0] = 3, array[1] = {}, and the other elements were {:?}",
                second, tail
        ),

        [first, middle @ .., last] =>
            println!("array[0] = {}, middle = {:?}, and the last element is {}",
                first, middle, last
        ),
    }
}
