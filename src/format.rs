use std::fmt;

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
