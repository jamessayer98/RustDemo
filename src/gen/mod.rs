use std::fmt::{Debug, Display};

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<U, T> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

trait Red {}

struct GenRed;

impl Red for GenRed {}

fn red<T: Red>(_: &T) -> &str {
    "red"
}

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug {:?}", t);
    println!("Display {}", t);
}

fn compare_types<T, U>(t: &T, u: &U)
where
    T: Debug,
    U: Debug,
{
    println!("t {:?}", t);
    println!("u {:?}", u);
}

trait Contains {
    type A;
    type B;

    fn contains(&self, n1: &Self::A, n2: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

struct Container(i32, i32);

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, n1: &i32, n2: &i32) -> bool {
        (&self.0 == n1) && (&self.1 == n2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

pub fn some_gen() {
    let g = Val { val: 1.1 };
    let gv = GenVal { gen_val: 123i32 };

    println!("g:{:?} gv:{:?}", g.value(), gv.value());

    let empty = Empty;
    let null = Null;
    empty.double_drop(null);

    let is_red = GenRed {};
    println!("is red: {}", red(&is_red));

    let s = "words";
    let arr = [1, 2, 3];
    let v = vec![1, 2, 3];
    compare_prints(&s);
    compare_types(&arr, &v);

    let n1 = 3i32;
    let n2 = 10i32;
    let container = Container(n1, n2);
    println!(
        "does container contain {} and {}: {}",
        n1,
        n2,
        container.contains(&n1, &n2)
    );
    println!("the difference: {}", difference(&container));
}
