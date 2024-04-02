use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
    // Raditude
    cool: &'static str,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // write! is like format! but it will write the formatted string
        // into a buffer (the first argument)
        // Hey this is Will Elder and I think this comment means that this
        // exercise was written before testcase list.
        // I've been writing write! for a few exercies now....
        write!(f, "{}: {:.3}°{} {:.3}°{} - cool? {}", // look I did copy the degree symbol
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c, self.cool)
    }    
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    // exercise left to the reader (me!)
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB: ({0}, {1}, {2}) 0x{0:0>2x}{1:0>2x}{2:0>2x}",
            self.red, self.green, self.blue)
    }
    // this seems more elegant (but perhaps less Rust-like?) than expected
    // based on the hints
    // the hints are suggesting RGB = (R*65536)+(G*256)+B which
    // I mean it's fine but I just learned about that cool :x stuff so
    // anyway I like this even though it's lower case
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722, cool: "very" }, // Sráid Uí Chonaill
        City { name: "Oslo", lat: 59.95, lon: 10.75, cool: "heck yes" },
        City { name: "Vancouver", lat: 49.25, lon: -123.1, cool: "rockin" },
        City { name: "Los Angeles", lat: 34.05, lon: -118.25, cool: "rad"},
        City { name: "Melbourne", lat: -37.81, lon: 144.96, cool: "tubular"},
    ]{
        println!("{}", city);
    }

    for color in [
        Color{ red: 128, green: 255, blue: 90 }, // bright green
        Color{ red: 0, green: 3, blue: 254}, // bright blue
        Color{ red: 0, green: 0, blue: 0}, // black
    ]{
        println!("{}", color);
    }
}
