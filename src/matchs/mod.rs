#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    RGB(i32, i32),
}

pub fn some_matchs() {
    let number: i32 = 10;
    match number {
        0 => println!("ooo no"),
        1...5 => println!("emmmmmmm"),
        6 | 7 | 10 => println!("bucuobucuo"),
        _ => println!("shenmegui"),
    }

    let bbb: bool = true;
    let bin = match bbb {
        true => 1,
        false => 0,
    };
    println!("{} -> {}", bbb, bin);

    let pair: (i32, i32) = (1, 2);
    match pair {
        (0, y) => println!("x is zero, y is {:?}", y),
        (x, 0) => println!("x is {:?}, y is zero", x),
        _ => println!("emmm {:?}", pair),
    }

    let color = Color::Red;
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::RGB(a, b) => println!("a:{} b:{}", a, b),
    }

    let reference = &4;
    match reference {
        &val => println!("reference is {:?}", val),
    }

    match *reference {
        val => println!("reference is {:?}", val),
    }

    let value = 5;
    let mut mut_value = 10;

    match value {
        ref val => println!("value reference is {:?}", val),
    }
    match mut_value {
        ref mut val => {
            println!("mut value reference is {:?}", val);
            *val += 1;
            println!("mut value is {}", val);
        }
    }

    let pair = (1, 2);
    match pair {
        (x, y) if x == y => println!("equal le"),
        (x, y) if x > 0 && y < 5 => println!("bucuo jius zheli"),
        (x, y) if x < 0 && y > 5 => println!("buhui la"),
        (x, _) if x == 0 => println!("x zero"),
        _ => println!("emmmmm"),
    }

    let age = || 15;
    match age() {
        n @ 0 => println!("aaaaa {}", n),
        n @ 1...20 => println!("jiushizheli {}", n),
        _ => println!("haha"),
    }

    let num = Some(7);
    if let Some(i) = num {
        println!("let some {}", i);
    } else {
        println!("let some error");
    }
}
