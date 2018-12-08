pub fn some_closure() {
    let closure_annoted = |i: i32| i + 1;
    let closure_inferd = |i| i + 1;
    let i = 1;

    println!("annoted {}", closure_annoted(i));
    println!("inferd {}", closure_inferd(i));

    let one = || 1;
    println!("one {}", one());

    let print = || println!("emmm this is closure");
    print();

    let mut count = 1;
    let mut inc = || {
        count += 1;
        println!("count+1:{}", count);
    };
    inc();
    inc();

    let movable = Box::new(3);
    let consumer = || {
        println!("drop box {}", movable);
        std::mem::drop(movable);
    };
    consumer();

    let green = "green";
    let mut farewall = "goodbye".to_string();
    let dirty = || {
        println!("this is green {}", green);
        farewall.push_str(" hi");
        println!("this is end {}", farewall);
        std::mem::drop(farewall);
    };
    apply(dirty);

    let double = |x| 2 * x;
    println!("double {}", apply_to(double));

    let vec1 = vec![1, 2, 3];
    println!("vec any equal 2: {}", vec1.iter().any(|&x| x == 2));
    println!("vec any equal 2: {}", vec1.into_iter().any(|x| x == 2));
    let vec1 = vec![1, 2, 3];
    // let mut vec_iter = vec1.iter();
    println!(
        "vec find equal 2: {:?}",
        vec1.iter().cloned().find(|&x| x == 2)
    );

    let arr1 = [1, 2, 3];
    println!("arr any equal 2: {}", arr1.iter().any(|&x| x == 2));
    println!("arr any equal 2: {}", arr1.into_iter().any(|&x| x == 2));

    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < 1000)
        .filter(|&x| x % 2 == 1)
        .fold(0, |sum, x| sum + x);
    println!("sum of squared odd numbers: {}", sum_of_squared_odd_numbers);
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}
