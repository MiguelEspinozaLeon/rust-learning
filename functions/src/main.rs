fn main() {

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");


    println!("Hello, world!");
    another_function();
    let sum = sum(5, 10);

    println!("The result of the sum of 5 and 10 is {sum}")
}

fn another_function() {
    println!("Another function.");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
