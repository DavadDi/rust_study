// 用于声明依赖外部库 rand
extern crate rand;

// use 主要适用于命名空间的缩写，类似于c++的using关键字
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

	// let 为 rust 变量赋值的关键字， 默认情况为赋值的变量是 immutable，不能够被修改 
    let secret_number = rand::thread_rng().gen_range(1, 101);  // [1, 101)

	// loop 对应于无限循环
    loop {
        println!("Please input your guess.");

		// 定义一个 String 变量，但是该变量是可以是 mutable，值可以被修改
		// 完整类型为： Struct std::string::String
        let mut guess = String::new();

		// 采用 std::io::stdin().read_line() 函数，从用户输入读取变量，并赋值到 guess 变量中。  
		// pub fn stdin() -> Stdin (pub struct Stdin { /* fields omitted */ })
		// fn Stdin::read_line(&self, buf: &mut String) -> Result<usize>
		// type Result<T> = Result<T, Error>;
		// std::result::Result
		// pub enum Result<T, E> {
		//		Ok(T),
		//		Err(E),
		//}
		
		// fn expect(self, msg: &str) -> T
		// Panics if the value is an Err, with a panic message including the passed message, and the content of the Err.

		// &mut guess 中的 &mut 则用于指明，guess是可以修改变量
        io::stdin().read_line(&mut guess)    
            .expect("Failed to read line");  // .expect 为 readline 出错后的，panic 打印的信息
            
        // guess: u32 用于指明 guess变量的类型为u32
        // rust 允许 重新定义 guess 为 u32 类型，用于覆盖前面定义的 String 类型的 guess  
        
        // fn trim(&self) -> &str,  trim 用于删除字符串前后的空格，空格依赖于 White_Space
        // fn str::parse<F>(&self) -> Result<F, F::Err>  Parses this string slice into another type.
        
        // 使用 match 来处理， result的两个 枚举值， Ok or Err 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // "{}" 变量 guess 占位符 
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;  // 如果guess ok，则退出
            }
        }
    }
}