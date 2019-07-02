fn main() {
    println!("Hello, world!");
    another_function(10, 11);

    let z = {
        let x = 3;
        x + 1
    };

    println!("The value of z is: {}", z);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

