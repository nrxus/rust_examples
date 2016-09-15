use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct MinMax(i64, i64);

#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

struct List(Vec<i32>);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Structure(number) = *self;
        write!(f, "{}", number)
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let MinMax(min, max) = *self;
        write!(f, "({}, {})", min, max)
    }
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:b}, y: {:b}", self.x as i64, self.y as i64)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;

        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            try!(write!(f, "{}: ", count));
            try!(write!(f, "{}", v));
            if count != vec.len() - 1 {
                try!(write!(f, ", "));
            }
        }

        write!(f, "]")
    }
}

fn hello_world_example() {
    println!("Hello, world!");
}

fn comment_example() {
    let x = 5 + /*+ 90 + */5;
    println!("Is `x` 10 or 100? x = {}", x);
}

fn format_example() {
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
    println!("This struct `{}` WILL print...", Structure(3));
    let pi = 3.141592;
    println!("Pi is roughly {pi:.dec_places$}", pi = pi, dec_places = 2);
}

fn debug_format_example() {
    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name",
             "Slater",
             "Christian",
             actor = "actor's");
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));
}

fn display_example() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2 { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Binary: {:b}", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}

fn try_example() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

fn main() {
    hello_world_example();
    comment_example();
    format_example();
    debug_format_example();
    display_example();
    try_example();
}
