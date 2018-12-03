use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessit() {
    println!("Hello, world!");
    let mut gu = String::new();

    loop {
        println!("please input your guess!");
        io::stdin().read_line(&mut gu).expect("wocao!!");
        let rnd_num = rand::thread_rng().gen_range(1, 101);
        println!("emmmm: {}", rnd_num);

        // let rnd_str = rnd_num.to_string();
        // match gu.cmp(&rnd_str) {
        //     Ordering::Equal => println!("you win !"),
        //     Ordering::Greater => println!("too greater!"),
        //     Ordering::Less => println!("too less"),
        // }

        let gu: u32 = match gu.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match gu.cmp(&rnd_num) {
            Ordering::Equal => {
                println!("you win!");
                break;
            }
            Ordering::Greater => println!("too greater!"),
            Ordering::Less => println!("too less!"),
        }
    }
}
