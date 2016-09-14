fn main() {
    // hello world
    println!("Hello, world!");

    // comment blocks
    let x = 5 + /*+ 90 + */5;
    println!("Is `x` 10 or 100? x = {}", x);

    // println format
    println!("{} days", 31);
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>0width$}", number = 1, width = 6);
    // this won't compile
    // println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");
    struct Structure(i32);
    impl std::fmt::Display for Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let Structure(number) = *self;
            write!(f, "{}", number)
        }
    }
    println!("This struct `{}` WILL print...", Structure(3));
    let pi = 3.141592;
    println!("Pi is roughly {pi:.dec_places$}", pi = pi, dec_places = 2);
}
