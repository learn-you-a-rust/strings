#[allow(unused_variables)]
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string
    
    println!("{}", s); // prints "hello, world!"
    
    let x = 5; // Because this value is an integer (and so of a known size),
               // it is pushed to the stack and bound to x
    let y = x; // Here, a copy of x is made and bound to y, also on the stack

    println!("x = {}, y = {}", x, y); // Copying on the stack is quick

    let s1 = String::from("Hello");
    let s2 = s1; // Here, the pointer, length, and capacity of the buffer is
                 // copied, but not the actual data, as it was above

    //println!("{}, world!", s1); // This won't work; s1 has been moved

    let s1 = String::from("hello");
    let s2 = s1.clone(); // To perform a deep copy, use the clone method

    println!("s1 = {}, s2 = {}", s1, s2);
}
