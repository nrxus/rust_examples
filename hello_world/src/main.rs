use std::fmt::{self, Formatter, Display, Result};

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

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Structure {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Structure(number) = *self;
        write!(f, "{}", number)
    }
}

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let MinMax(min, max) = *self;
        write!(f, "({}, {})", min, max)
    }
}

impl Display for Point2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x: {:b}, y: {:b}", self.x as i64, self.y as i64)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
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

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f,
               "{}: {:.3}°{} {:.3}°{}",
               self.name,
               self.lat.abs(),
               lat_c,
               self.lon.abs(),
               lon_c)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "RGB ({}, {}, {}) {:#04X}{:02X}{:02X}",
               self.red,
               self.green,
               self.blue,
               self.red,
               self.green,
               self.blue)
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

fn special_fmt_example() {
    for city in [City {
                     name: "Dublin",
                     lat: 53.347778,
                     lon: -6.259722,
                 },
                 City {
                     name: "Oslo",
                     lat: 59.95,
                     lon: -6.259722,
                 },
                 City {
                     name: "Vancouver",
                     lat: 49.25,
                     lon: -123.1,
                 }]
        .iter() {
        println!("{}", city);
    }

    for color in [Color {
                      red: 128,
                      green: 255,
                      blue: 90,
                  },
                  Color {
                      red: 0,
                      green: 3,
                      blue: 254,
                  },
                  Color {
                      red: 0,
                      green: 0,
                      blue: 0,
                  }]
        .iter() {
        println!("{}", color)
    }
}

fn main() {
    hello_world_example();
    comment_example();
    format_example();
    debug_format_example();
    display_example();
    try_example();
    special_fmt_example();
}
