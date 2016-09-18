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

fn main() {
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
