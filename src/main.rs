#[allow(unused_variables)]
fn main() {
    // initializing a string
    let mut s = String::new(); // creates a new, empty string
    let data = "initial contents"; // string literal
    let s = data.to_string(); // converts the string literal to a String
    let s = "initial contents".to_string(); // you can do it this way too
    let s = String::from("initial contents"); // or this way

    // updating a string
    let mut s = String::from("foo");
    s.push_str("bar"); // the argument is a string slice

    let mut s1 = String::from("foo");
    let s2 = "bar";
    let s3 = String::from("baz");
    s1.push_str(s2); // push a string literal (no ownership)
    s1.push_str(&s3); // or push a reference to a String
    println!("s2 is {}", s2); // this still works
    println!("s3 is {}", s3); // this still works
    println!("s1 is {}", s1);

    // add one char with `push`
    let mut l = String::from("lo");
    l.push('l');

    // combine two existing strings into a new string
    let string1 = String::from("hello, ");
    let string2 = String::from("world");
    let string3 = string1 + &string2; // s1 has been moved here and can no longer be used

    // this is the function `+` is using
    // fn add(self, s: &str) -> String {...}
    
    // but you can use a &String instead of a &str above because the compiler
    // coerces the &String into a &str. Rust's deref coercion turns &s2 into &s2[..]
    
    // add() takes ownership of `self`, so the first param is no longer usable here
    

    // to combine more than two strings, use the format! macro,
    // which doesn't take ownership of any parameters
    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");
    let some_string = format!("{}-{}-{}", a, b, c);
 
    // you can't index into strings, but you can create string slices (of bytes);
    // if the string's letters take more than one byte to store, this can cause
    // panic at runtime
    let h = "hello";
    let hi = &h[0..4]; // this creates a &str of the first 4 bytes of h

    // iterating over strings
    for c in "blah".chars() {
        println!("{}", c);
    }

    // this will print the numerical value of the byte
    for b in "blah".bytes() {
        println!("{}", b);
    }
}
