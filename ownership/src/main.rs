
/*fn main() {
    let mut s = String::from("Hello");

    s.push_str(", World!");

    println!("{}", s);

    //move

    /* 
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
    */

    //deep copy

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}*/
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward


    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    test();

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
    
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn test() {
    let s1 = String::from("hello");

    let len = calc_len(&s1); 

    println!("the length of {s1} is {len}")

    let mut s = String::from("hello");

    change(&mut s);
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("world");
}
