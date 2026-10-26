use std::io;

fn main() {

    // variables
    let x = "Hello"; // can't be changed - immutable
    let mut y = "world!"; // can be changed - mutable
    y = "Rusteceans!";

    // output
    println!("{} {}", x, y);

    // data types
    let num = 5;
    let double = 5.99;
    let bool = true;
    let letter = 'A';
    let text = "Nadia";
    println!("You owe me {} dollars {}", double, text);

    //type  casting 
    let cast_double : f64 = 5.99;

    // constants
    const PI: f64 = 3.14159;

    // if..else if...else
    if 7 > 5 {
        println!("True")
    } else if 7 < 5 {
        println!("False")
    } else  {
        println!("Equal")
    }

    // rust match
    let day = 3;

    print!("The day is ");

    match day  {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day")
    }

    //storing the result of match in a variable
    let day = 4;

    let day_name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 | 7 => "Weekend",
        _ => "Invalid day"
    };
    println!("The day is {}", day_name);


    // loops
    // loop {
    //     println!("This will repeat forever!")
    // }

    let mut a = 1;

    loop {
        
        // println!("{}", a);

        if a == 10000 {
            break
        };

        a += 1;

       
    };

    println!("the loop stopped at {}", a);


    // while loop
    let mut count = 1;

    while count <= 10000 {
        count += 1;
        println!("{}", count);

        // skip a value
        if count == 43 {
            continue;
        };

        // conditionally stop the while loop
        if count == 10000 {
            break;
        };
    };

    println!("count stopped at {}", count);


    // for loop
    for i in 1..6 {
        println!("i is: {}", i);
    }

    // inclusive range
    for i in 1..=6 {
        println!("i is: {}", i);
    }

    //break and continue
    for i in 1..=10 {
        if i == 3 {
            continue;
        };

        if i == 8 {
            break;
        };

        println!("i is: {}", i);
    }

    hello();
    hellotwo("Bob");
    input();
}


// functions
fn hello() {
    println!("Hello from a function!");
}


// function parameters
fn hellotwo(name: &str) {
    println!("Hello, {}!", name);
}

//input 
fn input() {
    println!("Hello!");
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("hi there, {}", input);  
}