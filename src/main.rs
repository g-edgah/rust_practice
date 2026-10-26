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
    let cast_double : f64 = 5.99

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



}
