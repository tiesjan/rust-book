// Statements don't return a value, expressions do return an (evaluated) value

fn main() {
    // `let x = ...;` is a statement, `plus_one(5)` an expression
    let x = plus_one(5);

    // Scope block is also an expression
    let y = {
        let x = 3;
        // No semicolon, which separates expressions from statements
        x + 1
    };

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

// Function definitions are also statements
fn plus_one(x: i32) -> i32 {
    // Including the semicolon would make it a statement and make the return value of the function
    // the unit type `()` by default; this would not compile, as an `i32` is expected
    x + 1
}