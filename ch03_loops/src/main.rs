fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop without if,else beal -> page 56
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFOFF!!!");

    // so slow and error prone way -> page 57
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index +=1;
    }

    // faster while in loop -> page 57f
    for element in a {
        println!("the valie is: {element}");
    }
}

