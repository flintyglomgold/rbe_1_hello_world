// So it says testcast: list
// but really it's introducing the ? operator

// maybe it's both, let's see

use std::fmt;

struct List(Vec<i32>);

// Rustaceans: omg stop implementing lists to learn Rust
// Also Rustaceans: here is a list in our first tutorial chapter

impl fmt::Display for List {
    //once again the necessary function for Display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?; // ooooh ? operator
        //but I mean it's just making sure we can print a "]"

        // now we iterate over the vec while enumerating
        for (count, _v) in vec.iter().enumerate() {
            if count != 0{
                write!(f, ", ")?;
            }
            // activity: altered this to include the index as well as the entry
            write!(f, "{0}: {1}", count, _v)?; 
        }

        write!(f, "]")
    }
}

// without the ? operator in the Display impl, this *should* work
// but you get a warning - about the specific issue at play
// Result may be Err, which should be handled - in this case, by `?`

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
