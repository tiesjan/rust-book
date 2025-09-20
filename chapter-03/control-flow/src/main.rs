fn main() {
    let condition = true;
    // `if` is an expression and the resulting value can be assigned
    let number = if condition { 6 } else { 5 };
    println!("The value of number is: {number}");

    // `if` expression *must* be a bool
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!();

    let mut counter = 0;
    // `loop` expression assigned to the `result` variable
    let result = 'counting_up: loop {  // loop label for outer
        println!("count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;  // breaks inner loop
            }
            if counter == 2 {
                break 'counting_up counter;
            }
            remaining -= 1;
        }

        counter += 1;
    };
    println!("End count = {result}");

    println!();

    // `while` loop for conditional looping
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    println!();

    // Looping through collections and iterators
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}