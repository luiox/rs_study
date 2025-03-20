use std::ops::Add;

enum Direction {
    Up = 0,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct KeyVal(i32);

enum Event {
    KeyPress(KeyVal),
    MousePress(Point),
    MouseRelease(Point),
    MouseMove(Point),
}

fn apply(v: i32, f: fn(i32) -> i32) -> i32 {
    f(v)
}

fn cube(x: i32) -> i32 {
    x * x
}

struct Identifier();

fn sum1() -> i32 {
    let mut i = 1;
    let mut sum = 0;
    loop {
        sum += i;
        i += 1;

        if i > 100 {
            break;
        }
    }
    sum
}
fn sum2() -> i32 {
    let mut i = 1;
    let mut sum = 0;
    while i <= 100 {
        sum += i;
        i += 1;
    }
    sum
}

fn sum3() -> i32 {
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    sum
}

fn process_event(e: Event) -> i32 {
    match e {
        Event::KeyPress(KeyVal(v)) => {
            println!("KeyPress: {}", v);
            1
        }
        Event::MousePress(Point { x, y }) => {
            println!("MousePress: ({}, {})", x, y);
            2
        }
        Event::MouseRelease(Point { x, y }) => {
            println!("MouseRelease: ({}, {})", x, y);
            3
        }
        Event::MouseMove(Point { x, y }) => {
            print!("MouseMove:({}, {})", x, y);
            4
        }
    }
}

fn move_owner(s: String) {
    println!("{}", s);
}

fn borrow_val(s: &String) {
    println!("{}", s);
}

fn strcat(dest: &mut String, src: &String) {
    dest.push_str(src);
}
struct Id();

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn distance_to(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

fn main() {
    let p = Point::new(1, 2);
    let p2 = Point {
        x: p.x + 10,
        y: p.y,
    };
    println!("distance = {}", p.distance_to(&p2));
    println!("{:?}", p);

    let key = KeyVal(1);
    let id = Id {};
    let p2 = Point { x: p.x + 10, ..p };

    let n1 = 1;
    let n2: i64 = 2;
    const PI: f64 = 3.141592;
    static mut N4: i32 = 4;

    if n1 > n2 {
        println!("n1 > n2");
    } else {
        println!("n1 <= n2");
    }

    let t = (1, 2);
    println!("{}", apply(5, cube));

    println!("{}", sum2());

    let s = "s1".to_string();
    let a = -1.abs();


}
