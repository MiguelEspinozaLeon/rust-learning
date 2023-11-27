use std::io;

fn main() {
    //const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    //mutable variable let mut x = 5;
    
    let x: i32 = 5;
    let x: i32 = x + 1;
    let z: f64 = 2.0;
    let w: f32 = 3.0;
    let t = true;
    let f: bool = false;
    let c: char = 'z';
    println!("{}",z);
    println!("{}", t);
    {
        let x = x * 2;
        println!("the value of the inner scope is {x}");
    }
    println!("The value of x is {x}");

    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    //let (x, y, z) = tup;
    //println!("the value of y is {y}");

    /*let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    */
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];
    println!("The value of the element at index {index} is {element}");



    let first = arr[0];
    println!("first value of the array is {first}");
    for x in arr  {
        println!("{x}");
    }


}
