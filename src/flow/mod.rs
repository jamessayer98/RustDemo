pub fn ifelse() {
    let n = 20;
    if n < 0 {
        print!("number is negative");
    } else if n > 0 {
        print!("number is positive");
    } else {
        print!("number is zero");
    }
    let big = {
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            n * 10
        } else {
            println!(", and is a big number, reduce by two");
            n / 2
        }
    };
    println!("big is {}", big);
}

pub fn some_loop() {
    println!("let begin a loop");
    let mut count: i32 = 0;
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("count is {}", count);
        if count >= 5 {
            println!("quit");
            break;
        }
    }

    let mut i: i32 = 0;
    'outer: loop {
        println!("enter outer loop");
        i += 1;
        'inner: loop {
            println!("enter inner loop");
            if i == 1 {
                break;
            } else {
                break 'outer;
            }
        }
        println!("break from inner {}", i);
    }
    println!("exited the outer loop {}", i);

    let mut n: i32 = 0;
    let rn = loop {
        n += 1;
        if n == 5 {
            break n * 2;
        }
    };
    println!("n:{} rn:{}", n, rn);

    let mut w: i32 = 0;
    while w < 3 {
        println!("w is {}", w);
        w += 1;
    }

    for i in 1..4 {
        println!("i is {}", i);
    }
}
