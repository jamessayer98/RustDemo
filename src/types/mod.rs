use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element: {}", slice[0]);
    println!("slice len: {}", slice.len());
}

pub fn some_slice() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    println!("first element: {}", xs[0]);
    println!("slice len: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of array as a alice");
    analyze_slice(&ys[1..100]);
}

#[derive(Debug)]
struct Nil;

struct Pair(i32, i32);

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

pub fn some_struct() {
    let point: Point = Point { x: 1, y: 2 };
    println!("point ({}, {})", point.x, point.y);

    let Point { x: myx, y: myy } = point;
    let rectangle = Rectangle {
        p1: Point { x: myx, y: myy },
        p2: point,
    };
    println!("recrangle: {:?}", rectangle);

    let _nil = Nil;
    println!("nil is:{:?}", _nil);
    let _pair = Pair(1, 2);
    println!("pair x:{:?} y:{:?}", _pair.0, _pair.1);

    let Pair(ii, jj) = _pair;
    println!("destructing: ii:{}, jj:{}", ii, jj);
}

#[allow(dead_code)]
enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String },
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("is a engineer"),
        Person::Scientist => println!("is a scientist"),
        Person::Height(i) => println!("he is height:{}", i),
        Person::Weight(w) => println!("he is weight:{}", w),
        Person::Info { name } => println!("his name is:{}", name),
    }
}

pub fn some_enum() {
    let eng = Person::Engineer;
    let sci = Person::Scientist;
    let h = Person::Height(10);
    let w = Person::Weight(20);
    let info = Person::Info {
        name: "dave".to_string(),
    };

    inspect(eng);
    inspect(sci);
    inspect(h);
    inspect(w);
    inspect(info);
}

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            List::Cons(num, ref tail) => {
                println!("emm: {}", num);
                1 + tail.len()
            }
            List::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => format!("{} {}", head, tail.stringify()),
            List::Nil => format!("Nil"),
        }
    }
}

pub fn some_list() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    println!("list len:{}", list.len());
    println!("list string:{}", list.stringify());
    list = list.prepend(5);
    println!("list len:{}", list.len());
    println!("list string:{}", list.stringify());
}
