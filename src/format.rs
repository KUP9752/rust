use std::fmt;

#[allow(dead_code)]
pub fn main() {
  println!("{} days", 31);

  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  println!(
    "{subject} {verb} {object}",
    object = "the lazy dog",
    subject = "the quick brown fox",
    verb = "jumps over"
  );

  println!("Base 10:               {}", 69420); // 69420
  println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
  println!("Base 8 (octal):        {:o}", 69420); // 207454
  println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

  // right-jutify
  println!("{number:>5}", number = 1);

  // You can pad numbers with extra zeroes,
  println!("{number:0>5}", number = 1); // 00001
                                        // and left-adjust by flipping the sign. This will output "10000".
  println!("{number:0<5}", number = 1); // 10000

  // You can use named arguments in the format specifier by appending a `$`.
  println!("{number:0>width$}", number = 1, width = 5);

  // Rust even checks to make sure the correct number of arguments are used.
  println!("My name is {0}, {1} {0}", "Bond", "James");

  // Only types that implement fmt::Display can be formatted with `{}`. User-
  // defined types do not implement fmt::Display by default.
  #[derive(Debug)]
  struct Structure(i32);

  // {:?} Debug must be derived, or Display must be implemented
  impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.0)
    }
  }

  // This will not compile because `Structure` does not implement
  // fmt::Display.
  println!("This struct `{:}` won't print...", Structure(3));

  // this is for just debug printing
  println!("This struct `{:?}` won't print...", Structure(3));
  // this is for pretty debug printing
  println!("This struct `{:#?}` won't print...", Structure(3));

  // For Rust 1.58 and above, you can directly capture the argument from a
  // surrounding variable. Just like the above, this will output
  // "    1", 4 white spaces and a "1".
  let number: f64 = 1.0;
  let width: usize = 5;
  println!("{number:>width$}");

  // print Pi, using decimals
  let pi = 3.141592;
  println!("Pi is roughly {:.3}", pi); // 3.142
}

pub fn display() {
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
      write!(f, "(x: {}, y: {})", self.x, self.y)
    }
  }

  let minmax = MinMax(0, 14);

  println!("Compare structures:");
  println!("Display: {}", minmax);
  println!("Debug: {:?}", minmax);

  let big_range = MinMax(-300, 300);
  let small_range = MinMax(-3, 3);

  println!(
    "The big range is {big} and the small is {small}",
    small = small_range,
    big = big_range
  );

  let point = Point2D { x: 3.3, y: 7.2 };

  println!("Compare points:");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);

  // Error. Both `Debug` and `Display` were implemented, but `{:b}`
  // requires `fmt::Binary` to be implemented. This will not work.
  // println!("What does Point2D look like in binary: {:b}?", point);

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
  println!(
    "Display: {}",
    Complex {
      real: 6.7,
      imag: 5.4
    }
  );
  println!(
    "Debug: {:?}",
    Complex {
      real: 6.7,
      imag: 5.4
    }
  );
}
