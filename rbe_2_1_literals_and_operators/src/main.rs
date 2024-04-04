// review: integers can be expressed in hex, octal, or binary
// new: underscores can be inserted in numeric literals to improve readability
// eg: 1_000 is the same as 1000
// new: scientific E-notation, eg. 1e9 instead of 1_000_000_000


fn main() {
    // Adding
    println!("123 + 45 = {}", 123u32 + 45);

    // Subtracting
    println!("1 - 2 = {}", 1i32 - 2);

    //println!("1 - 2 = {}", 1u32 - 2);

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // More bools that a sourdough factory, babyyyyy
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    //going off script here
    println!("true XOR false is {}", true ^ false);

    // oh xor shows up in bitwise that's cool too
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One MILLION dollars is ${}", 1_000_000u32);
}
