fn main() {

    // data move
    let x = 5;
    let y = x;  

    // 以为x,y都为简单类型，分配在stack上，重新赋值后仍然可以使用
    println!("x: {} y: {}", x, y);

    let s1 = String::from("hello");
    // move s1 owner to s2, s1不能够再使用
    let s2 = s1;

    // println!("s1 {}", s1)  // Error. s1 move to s2, 不再有效
    println!("s2 {}", s2);


    // data clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // ok, use clone
    
    // 函数参数和返回值都会发生ownership的转移
    let s = String::from("hello");  // s comes into scope.
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.
    
    let x = 5;                      // x comes into scope.
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.
}// Here, x goes out of scope, then s. But since s's value was moved, nothing
// special happens.


fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
