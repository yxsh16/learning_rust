fn main() {
    //println!("Hello World");

    let x: i8 = 2; // This is an immutable variable
    let y: i8 = 5;
    let z = -7;
    println!("{}", x + y + z); // Only signed integers can be operated together

    let mut a = 32; // This is a mutable variable
    println!("{}", a + 32);

    let is_present: bool = true;
    let is_pass: bool = false;

    // Strings
    let my_str: String = String::from("This is a string");
    let str2: &str = "This is a string";
    println!("{}", my_str);

    // Printing nth character of a string
    //print!("{}", my_str.chars().nth(0));  // This won't work

    let char1: Option<char> = my_str.chars().nth(2);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index"),
    }
}
