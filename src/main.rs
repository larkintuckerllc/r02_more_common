fn another_function() {
    println!("Another function.");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn abs(x: i32) -> i32 {
    if x >= 0 {
        return x;
    }
    x * -1
}

fn main() {
    // FUNCTIONS
    another_function();
    let x = 0;
    let y = plus_one(x);
    println!("The value of y is: {}", y); // 1
    let a = -1;
    let b = abs(a);
    println!("The value of b is: {}", b); // 1

    // COMMENTS
    /*
    Rust supports
    mult-line comments.
    */

    // IF EXPRESSIONS
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // LOOPS
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
