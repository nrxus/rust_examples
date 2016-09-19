#![allow(dead_code)]

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle { p1: Point { x: x1, y: y1 }, p2: Point { x: x2, y: y2 } } = *self;
        f32::abs((x2 - x1) * (y2 - y1))
    }
}

fn square(point: Point, width: f32) -> Rectangle {
    let p2 = Point {
        x: point.x + width,
        y: point.y + width,
    };
    Rectangle {
        p1: point,
        p2: p2,
    }
}

fn struct_examples() {
    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);
    let Point { x: my_x, y: my_y } = point;
    let rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    println!("rectangle area: {:.2}", rectangle.area());
    println!("other rectangle area : {:.2}",
             Rectangle {
                     p1: Point {
                         x: 3f32,
                         y: 10f32,
                     },
                     p2: Point {
                         x: 7f32,
                         y: 12f32,
                     },
                 }
                 .area());

    let _nil = Nil;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("square: {:?}", square(Point { x: 1.3, y: 3.7 }, 3.2));
}

enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 },
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("Is an engineer!"),
        Person::Scientist => println!("Is a scientist!"),
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}", i),
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        }
    }
}

fn enum_examples() {
    let person = Person::Height(18);
    let amira = Person::Weight(10);
    let dave = Person::Info {
        name: "Dave".to_owned(),
        height: 72,
    };
    let rebecca = Person::Scientist;
    let rohan = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn use_examples() {
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("the poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn c_enum_examples() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

use List::*;

enum List {
    Cons(u32, Box<List>),
    ListNil,
}

impl List {
    fn new() -> List {
        ListNil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            ListNil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            ListNil => format!("Nil"),
        }
    }
}

fn linked_list_example() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn constant_examples() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}

fn main() {
    struct_examples();
    enum_examples();
    use_examples();
    c_enum_examples();
    linked_list_example();
    constant_examples();
}
