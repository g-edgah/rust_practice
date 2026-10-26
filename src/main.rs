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
        1 => print!("Monday"),
        2 => print!("Tuesday"),
        3 => print!("Wednesday"),
        4 => print!("Thursday"),
        5 => print!("Friday"),
        6 | 7 => print!("Weekend"),
        _ => print!("Invalid day")
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
    print!("The day is: {}", day_name);


    // loops
    // loop {
    //     println!("This will repeat forever!")
    // }

    let mut a = 1;

    loop {
        
        println!("{}", a);

        if a == 10000 {
            break
        };
        
        a += 1;
    }

}
