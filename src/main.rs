fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    shadowing();
    shadowing_types()
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn shadowing_types() {
    let spaces = "      ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}

/*fn shadowing_mut_diferences() {
    let mut spaces = "      ";
    spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
*/
