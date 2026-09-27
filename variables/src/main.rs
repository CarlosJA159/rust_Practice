use std::io;

fn five() -> i32 {
    5
}

//

fn main() {
   

    let mut x = 5; //adding a mut variable indicates it can change when prompting x with a new value
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //

    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    //

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    // 

    let y = {
        let x = 3;
        x + 1 //this is not a statement so it does not have a semicolon
    };

    println!("The value of y is: {y}");

    //

    let x = five();

    println!("The value of x is: {x}");

    //

     let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //

      let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //

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

    //

     let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //

     let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //

    another_function(5)

}

fn another_function(x:i32) {

    println!("Another function.");
    println!("The value of x is: {x}");

    print_labeled_measurement(5, 'h');

}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");

}
