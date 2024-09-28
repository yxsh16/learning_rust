fn main() {
    //println!("Hello World");

    // Every variable by default is immutable.
    // Which is one of the reasons rust is faster than JS.

    let x: i8 = 2; // This is an immutable variable
    let y: i8 = 5;
    let z = -7;
    println!("{}", x + y + z); // Only signed integers can be operated together

    let mut a = 32; // This is a mutable variable
    a = a + 33;
    println!("{}", a);

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

    // Conditionals
    let is_even: bool = true;

    if is_even {
        println!("The num is even");
    } else if !is_even {
        println!("The num is odd");
    }

    // Loops
    for i in 0..5 {
        println!("{}", i);
    }

    // Functions

    let sentence = String::from("This is a sentence");
    let first_word = get_first_word(sentence);
    println!("The first word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut res = String::new();
    for c in sentence.chars() {
        if c == ' ' {
            break;
        }
        res.push(c);
    }
    return res;
}

/*

Memory management
Heap Vs Stack

Stack:
    * Variables with fixed size are stored in stack as stack frame
    * Stack has very fast allocation deallocation
    * Stack size is allocated at compile time

Heap:
    * Variables like strings/dynamic arr that can grow at runtime are stored in heap
    * Heaps are slower as compared to stacks
    * length, capacity and pointer to a heap data is stored in stack

*/
