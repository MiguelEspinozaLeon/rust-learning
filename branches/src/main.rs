use std::io;
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


    //loops
    let mut counter = 0;
    loop {
        counter += 1;

        if counter == 10 {
            break;
        }
    }
    println!("counter value is {counter}");


    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    //while

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");


    //looping through collection

    let a = [10, 20, 30, 40, 50];
    
    //using while
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    } 

    //using for
    for element in a {
        println!("the value is: {element}");
    }

    //reverse range
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!");

    println!("Enter the temperature to convert: ");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: i32 = temp
        .trim()
        .parse()
        .expect("Input was not a number");

    println!("Enter the metric to convert it to: ");

    let mut metric = String::new();

    io::stdin()
        .read_line(&mut metric)
        .expect("Failed to read line");

    let metric: char = metric
        .trim()
        .parse()
        .expect("Input was not a char");

    let result = convert_temperatures(temp, metric);

    println!("The result is {result} degrees");


    println!("Which number of the fibonacci sequence?");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    
    let num: i32 = num
        .trim()
        .parse()
        .expect("Input was not a number");

    let result = fibonacci_number(num);

    print!("the {num}th number of the fibonacci sequence is: {result}");

} 



fn convert_temperatures(temp: i32 , metric: char) -> f32 {
    if metric == 'F' {
        (temp * 9/5 + 32) as f32
    } else if metric == 'C' {
        ((temp - 32) * 5/9) as f32
    } else {
        0.0
    }
}

fn fibonacci_number(num: i32) -> i32 {
    if num == 0 || num == 1 {
        num 
    } else {
      fibonacci_number(num - 1) + fibonacci_number(num - 2)  
    }

}
