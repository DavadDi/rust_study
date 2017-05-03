fn main() {

    // if expression
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    // if and else 返回的类型必须一致
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);


    // loop expression

    let mut count = 0;

    loop {
        count = count + 1;
        if count == 4 {
            break;
        }

        println!("print under loop test");
    }

    // while expression
    let mut number = 3;
    while number != 0  {
        println!("{}!", number);

        number = number - 1;
    }

    // for expression
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
