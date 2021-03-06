#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Rectangle: (length: {}, width: {})", self.length, self.width)
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.", rect1.area()
    );
    
    // 通过实现 std::fmt::Display
    println!("rec {}", rect1);

    // 通过 derive(Debug) 的方式实现
    println!("rec {:#?}", rect1);
}