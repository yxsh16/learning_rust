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

    // Ownership

    let var_x: i8 = 1; // Created on stack, owner is main fn
    let var_y: i8 = 2; // Created on stack, owner is main fn
    println!("{}", sum(var_x, var_y));

    // Scoping variables in same function:
    let some_var: i8 = 3;
    {
        let some_var2: i8 = 4; // cannot be accessed outside this scope
    }

    println!("{}", some_var2); // Throws error
}

fn sum(i: i8, j: i8) -> i8 {
    let var_z: i8 = i + j; // i and j are created seperatley on stack, owner is sum fn
    return var_z;
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

/*

Ownership

    * Memory is managed through a system of ownership with a set of rules that the compiler checks.
    * If any of the rules are violated, the program wont compile.
    * A heap data can have only have one owner and if owner no longer exists, the heap data is cleared.

    For the code:
    let s1: String::from("HELLO");
    let s2 = s1;
    println!("{}", s1);

    * It will throw error since heap data can have only one owner which initially was s1 but later s2 became owner
    * Which makes s1 invalid

            STACK                               HEAP
    |-------------------|               |-------------------|
    |     S1---------------------------------> H            |
    |                   |               |      E            |
    |     S2            |               |      L            |
    |                   |               |      L            |
    |                   |               |      O            |
    |                   |               |                   |
    |-------------------|               |-------------------|



    Two pointers pointing at same object (two owners) can not happen

            STACK                               HEAP
    |-------------------|               |-------------------|
    |     S1---------------------------------> H            |
    |                   |               |      E            |
    |     S2---------------------------------> L            |
    |                   |               |      L            |
    |                   |               |      O            |
    |                   |               |                   |
    |-------------------|               |-------------------|


    after let s2 = s1:

            STACK                               HEAP
    |-------------------|               |-------------------|
    |     S1(inavlid)   |               |      H            |
    |                   |               |      E            |
    |     S2---------------------------------> L            |
    |                   |               |      L            |
    |                   |               |      O            |
    |                   |               |                   |
    |-------------------|               |-------------------|


    * To overcome this problem, we can use s1.clone() method.
    * It will create a new string with s2 as owner with same content as that of s1.

    After using .clone():

            STACK                               HEAP
    |-------------------|               |-------------------|
    |     S1---------------------------------> HELLO        |
    |                   |               |                   |
    |     S2---------------------------------> HELLO        |
    |                   |               |                   |
    |                   |               |                   |
    |                   |               |                   |
    |-------------------|               |-------------------|






*/
