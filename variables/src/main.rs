fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    
    // x = 6;  // not worked
    // println!("The value of x is: {}", x);

    // x shadowed the var x before 
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;  // constants
    
    // char type in rust, Unicode 变量值（Unicode Scalar Value）
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
