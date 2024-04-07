enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum CartoonishlyLongEnumNameForThePurposeOfDemonstratingTypeAlaising {
    Add,
    Subtract,
    Multiply
}

type Operations = CartoonishlyLongEnumNameForThePurposeOfDemonstratingTypeAlaising;

impl CartoonishlyLongEnumNameForThePurposeOfDemonstratingTypeAlaising {
    fn run(&self, x: i32, y:i32) -> i32 {
        match self {
            Self::Add      => x + y,
            Self::Subtract => x - y,
            Self::Multiply => x * y,
        }

        // note that Self is also an alias, like Operations
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad       => println!("page loaded"),
        WebEvent::PageUnload     => println!("page unloaded"),
        WebEvent::KeyPress(c)    => println!("pressent '{}'.", c),
        WebEvent::Paste(s)       => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
}

// The tutorial notes that more info on enums can be found at:
// https://github.com/rust-lang/rust/pull/61682/#issuecomment-502472847
