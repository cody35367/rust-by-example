use std::fmt; // Import `fmt`

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

/* Works but in the next lesson I learned: Implementing fmt::Display for a 
structure where the elements must each be handled sequentially is tricky. The
problem is that each write! generates a fmt::Result. Proper handling of this
requires dealing with all the results. Rust provides the ? operator for exactly
this purpose. */
impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        let xbytes = self.x.to_be_bytes();
        let ybytes = self.y.to_be_bytes();
        write!(f, "x:").expect("Failed to convert Point2D to bin");
        for b in xbytes {
            write!(f, " {:0>8b}", b).expect("Failed to convert Point2D to bin");
        }
        write!(f, ", y:").expect("Failed to convert Point2D to bin");
        for b in ybytes {
            write!(f, " {:0>8b}", b).expect("Failed to convert Point2D to bin");
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    println!("What does Point2D look like in binary: {:b}?", point);

    let c_val = Complex{real: 3.3, imag: 7.2};
    println!("Display: {}", c_val);
    println!("Debug: {:?}", c_val);
}