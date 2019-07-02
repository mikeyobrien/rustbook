fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything!"),
    }

    // named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50!"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 2;

    // multiple patterns
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // range matches
    match x {
        1...5 => println!("one to five"),
        6 => println!("six"),
        _ => println!("anything"),
    }
}
