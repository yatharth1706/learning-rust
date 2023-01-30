fn print_number(x: i32){
    println!("the value entered in params is: {x}");
}

fn main() {
    println!("Main function");

    another_function();

    print_number(10);
}

fn another_function(){
    println!("Another function called");
}
