use std::fmt::{self, Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
struct Structure(i32);

#[allow(dead_code)]
#[derive(Debug)]
struct Deep(Structure);

#[cfg(test)]
#[test]
fn test_debug_shuchu() {
    println!("{:?} month is a year!", 12);
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(3)));
}

#[allow(dead_code)]
#[derive(Debug)]
struct MinMax(i32, i32);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "min:{} max:{}", self.0, self.1)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "x:{} y:{}", self.x, self.y)
    }
}

#[test]
fn test_debug_shuchu2() {
    let minmax = MinMax(1, 2);
    println!("display:{}", minmax);
    println!("debug:{:?}", minmax);

    let point = Point { x: 1, y: 2 };
    println!("display:{}", point);
    println!("debug:{:?}", point);
}

#[allow(dead_code)]
#[derive(Debug)]
struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let List(ref vec) = *self;
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ",")?;
            }
            try!(write!(f, "{}", v));
        }
        write!(f, "]")
    }
}

#[test]
fn test_vec_shuchu() {
    let v = List(vec![1, 2, 3]);
    println!("display:{}", v)
}
