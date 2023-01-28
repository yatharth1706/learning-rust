fn main() {
    let x = 2.0; // f64 by default

    let y : f32 = 3.0; // f32

    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Sum: {sum}");
    println!("Product: {product}");
    println!("Subtraction: {difference}");
    println!("Quotient: {quotient}");
    println!("Truncated: {truncated}");
    println!("Remainder: {remainder}");


    // boolean examples:
    let isMigrationHappened = false;
    let areUComing : bool = true;

    println!("IsMigration Happend: {isMigrationHappened}");
    println!("Are u coming?: {areUComing}");
    
    if isMigrationHappened {
        println!("IsMigration Happend: {isMigrationHappened}");
    }

    if areUComing {
        println!("Are u coming?: {areUComing}");
    }

    let a = 'a';
    let b : char = 'B';
    let rocket_icon = '🚀';

    println!("Char a: {a}");
    println!("Char b: {b}");
    println!("Char rocket icon: {rocket_icon}");

    // tuples

    let tup = (1,4,20);

    let (x,y,z) = tup;

    println!("Val of x, y, z from tuple are : {x}, {y}, {z} respectively");

}
