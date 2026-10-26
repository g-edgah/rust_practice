fn main() {
    let x = "Hello"; // can't be changed - immutable
    let mut y = "world!"; // can be changed - mutable
    y = "Rusteceans!";
    println!("{} {}", x, y);

    let num = 5;
    let double = 5.99;
    let bool = true;
    let letter = 'A';
    let text = "Nadia";
    println!("You owe me {} dollars {}", double, text);
}
