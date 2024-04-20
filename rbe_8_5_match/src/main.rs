#[allow(dead_code)]

enum Color{
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

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

    // destructuring enums

    let color = Color::RGB(122, 17, 40); //#7A1128, a dark brown red. Topical!

    println!("What color is it?");

    match color{
        Color::Red      => println!("The color is red!"),
        Color::Blue     => println!("The color is blue!"),
        Color::Green    => println!("The color is green"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, Saturation: {}, Lightness: {}", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, Magenta: {}, Yellow: {}, Black: {}", c, m, y, k),
    }

    // destructuring and dereferencing pointers and refs

    let reference = &4;

    // the & signifies that there is a reference being assigned.

    match reference {
        // If reference is pattern matched against &val,
        // you're comparing &i32 and &val
        // if the matching & are dropped you can get the i32

        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // Or you can avoid the &, and dereference before the match

    match *reference {
        val => println!("Got a value via destructuring: {:?}", val),
    }
    
    let not_a_reference = 3;

    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            // got a reference, gotta dereference it before you can
            // do stuff

            *m += 10; // not sure if I'm out of practice or too immersed in c++
            // but this is breaking my brain.
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    // dereferencing structs (using that Foo defined up top)

    let foo = Foo { x: (1, 2), y: 3};

    match foo {
        Foo { x: (1, b), y } => println!("First of x is one, b = {}, y = {}", b, y),
        
        // or rearrange things! it's a struct!
        Foo { y: 2, x: i } => println!("y is two, x is {:?}", i),

        // rearrange things *and* ignore them! Like things in my schedule!
        Foo { y, .. } => println!("y = {} and the rest have been ignored", y),
    }
    let faa = Foo { x: (1, 2), y: 3 };

    // destructure structs outside of a match!
    let Foo { x: x0, y: y0} = faa;
    println!("Outside a struct, x0 = {x0:?}, y = {y0}");
    println!("Inside a struct it's too hard to read.");

    struct Bar{
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar { foo: Foo{ x: nested_x, y: nested_y } } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}
