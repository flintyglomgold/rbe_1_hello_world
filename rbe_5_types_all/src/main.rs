#![allow(overflowing_literals)]
// why can't i hold all these literals

// pre-main work for ALIASING

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    let decimal: f32 = 65.4321;

    // no implicit conversion allowed
    //let integer: u8 = decimal;

    println!("{}", decimal);

    // Explicit conversion
    let integer = decimal as u8; //`as` <-- explicit
    let character = integer as char;

    // but don't go too fast!
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // trunctation: when casting any value to an *unsigned* type, T,
    // T::MAX + 1 is added or subtracted until the value fits
    // e.g. if the max value is 255, then 256 will get added or subtracted
    // until it works

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);
    // 1000 - 256 - 256 - 256 = 232;
    println!("  -1 as a u8 is: {}", (-1i8) as u8);
    // -1 + 256 = 255

    // note similarities for positive numbers to modulus
    println!("1000 mod 256 is {}", 1000 % 256);

    // when casting to a signed type the (bitwise) result is the same
    // as the first casting to the corresponding unsigned type. If the most
    // significant bit of that value is 1, then the value is negative.

    // Unless it already fits.
    println!(" 128 as an i16 is : {}", 128 as i16 );

    // but if it doesn't
    println!(" 127 as an i8 is : {}", 128 as i8 );
    println!(" 128 as an i8 is : {}", 128 as i8 );
    println!(" 250 as an i8 is : {}", 128 as i8 );
     //binary leading 1 means negative

    // since Rust 1.45, `as` performs a saturating cast
    // IT'S CAST THINGS WET OR DRY FOLKS
    // CALL NOW FOR
    // no wait hang on
    // If the float exceeds the upper or lower bound, then
    // the returned value will equal the bound crossed

    println!(" 300.0 as a u8 is : {}", 300.0_f32 as u8);
    println!("-100.0 as a u8 is : {}", -100.0_f32 as u8);
    println!("   nan as a u8 is : {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost that can be avoided
    // by using abilities the jedi consider ...unnatural.
    // these results may overflow and return **unsound values**.
    // Use these methods wisely and make sure you never concede the
    // high ground to Obi-Wan Kenobi

    println!("unsafe conversions ahead:");
    unsafe {
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>() );
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>() );
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>() );
    }

    // some extra fun, the unsafe -100 conversion results, on my machine in 0.
    // in the tutorial, it is 156. Time to check my compiler version, I guess?

    // suffixed literals
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed literals
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // INFERENCE

    let elem = 5u8;

    let mut vec = Vec::new(); // empty vector with no type??

    vec.push(elem);

    println!("{:?}", vec);

    // ALIASING

    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    println!("{} nanoseconds + {} inches = {} units?????", inches, nanoseconds, inches + nanoseconds);

}
