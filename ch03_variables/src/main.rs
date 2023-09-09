
fn main() {
    let mut x: i8 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");    

    //shadownig -> page 34
    let x = x + 60;
    {
        //overflow protection -> page 38
        match i8::checked_mul(x, 2) {
            Some(num) => println!("The value of x in the inner scope is: {num}"),
            None => println!("To big for an 8bit integer")
        };
        
    }
    println!("The value of x is: {x}");

    //tuple -> page 40
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The values in the tupel are ({five_hundred}, {six_point_four}, {one})");

    //fn with retrun -> page 47f
    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(x);
    println!("The value of x is: {x}");
}

fn five() -> i8 {
    5
}

fn plus_one(x: i8) -> i8 {
    x + 1;
}