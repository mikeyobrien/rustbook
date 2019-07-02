struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    // using in match
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // enums
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The quit variant has no data to destructure");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, blue {}", r, g, b)
        }
    }

    // references
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

    println!("{}", sum_of_squares);

    // structs and tuples
    let ((feet, inches), Points { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
