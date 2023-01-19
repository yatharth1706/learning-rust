fn main() {
    let mut x = 10;
    x = x + 1;

    println!("The value of x: {x}");

    {
        // Shadowing x variable
        let x = x * 2;
        println!("The value of x: {x} inside scope");
    }

    println!("The value of x: {x} outside scope");
}
