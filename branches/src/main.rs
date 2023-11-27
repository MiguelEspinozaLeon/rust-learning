fn main() {
    let number = 3;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("it was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("the value of number is {number}");
}
