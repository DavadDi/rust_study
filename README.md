# Rust 简明教程

## 1. Install

Linux|Mac
	
	$ curl https://sh.rustup.rs -sSf | sh
	$ source $HOME/.cargo/env
	
	
Updateing

	$ rustup update
	
Uninstalling
	
	$ rustup self uninstall

Version && Doc

	$ rustc --version
	$ rustup doc 
	$ rustup doc --book  or rustup doc --std
	

## 2. Tutorial

第二版图书：

* [Book Eng](http://rust-lang.github.io/book/second-edition/ch01-00-introduction.html)
* [Rust 程序设计语言（第二版）](https://kaisery.gitbooks.io/trpl-zh-cn/content/)

### 2.1 Hello Rust
 	
	$ cat hello_world/main.rs
	fn main() {
	    println!("Hello, world!");
	}

**fn** 定义函数的关键字， **main** 函数为可执行程序的入口，**println!** 是rust定义的宏， “!” 是表示宏定义的调用。

### 2.2 Hello Cargo!

Cargo 是 Rust 的构建系统和包管理工具。

使用 **Cargo** 建立可执行程序：

	cargo new hello_cargo --bin

	$ tree hello_cargo
	hello_cargo
	├── Cargo.toml
	└── src
	    └── main.rs
	    
	 $ cat hello_cargo/Cargo.toml
	[package]
	name = "hello_cargo"
	version = "0.1.0"
	authors = ["diweihua <weihua@teambition.com>"]
	
	[dependencies]

	$ cat hello_cargo/src/main.rs
	fn main() {
	    println!("Hello, world!");
	}
	
	$ cargo build
   	Compiling hello_cargo v0.1.0 (file:///Users/diwh/Documents/Dev_home/rust_study/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47 secs
    
    $ cargo clean   # 用于清理生成的中间物
 
 build后会生成Cargo.lock文件, Cargo.lock 文件确保构建是可重现的，用于记录相关依赖包的版本和对应的commit_id 等信息。
    
    $ cat Cargo.lock
	[root]
	name = "hello_cargo"
	version = "0.1.0"

    $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
    Running `target/debug/hello_cargo`
	Hello, world!
	
	$ cargo build --release  # 用于生成release相关的程序
	
	
### 2.3 guessing_game 样例

**guessing_game** 用于生成一个随机数字，让用户输入猜测，给出提示。 完整代码如下：
	
首先需要在 Cargo.toml 文件中增加 **rand** 库依赖：
	
	[dependencies]
	rand = "0.3.14" 
	
	$ cargo build
    Updating registry `https://github.com/rust-lang/crates.io-index`
    Compiling libc v0.2.22
    Compiling rand v0.3.15
    Compiling guessing_game v0.1.0 (file:///Users/diwh/Documents/Dev_home/rust_study/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.69 secs
    
    
    $ cargo update 
    1. 忽略 Cargo.lock 文件并计算出所有符合 Cargo.toml 中规格的最新版本。
	2. 如果成功了，Cargo 会把这些版本写入 Cargo.lock 文件。
  
  rust 使用在线的 crates.io 库管理库，因为 **rand** 不是 rust 标准库，因此使用 **cargo build** 的时候会从网络上下载对应的依赖库。Crates.io是 Rust 生态环境中的开发者们向他人贡献他们的开源 Rust 项目的地方。
  
  完整代码如下：
  
```rust
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
```

运行程序：

	$ cargo run
	  Compiling guessing_game v0.1.0 (file:///Users/diwh/Documents/Dev_home/rust_study/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.35 secs
    Running `target/debug/guessing_game`
	Guess the number!
	Please input your guess.
	50
	You guessed: 50
	Too big!
	Please input your guess.
	25
	You guessed: 25
	Too small!
	Please input your guess.
	30
	You guessed: 30
	You win!


查看该项目依赖的lib的文档：

	cargo doc --open    # 会从网络上下载依赖库 rand 的doc，并在本地浏览器打开
  
  
## 3. Syntax and Semantics

### 3.1 Variable Bindings

```rust

let x = 5;
let (x, y) = (1, 2);
let x: i32 = 5;          // 指定 x 类型为 i32
let x = 5; // x: i32     // 通过 comment 方式指明 x 的类型为 i32

```

rust 的 **bindings are immutable**， 默认 rust 变量绑定是不可变，再次赋值会编译报错。

```rust
let mut x = 5; // mut x: i32    // x 为 mutable 变量，可以进行再次赋值
x = 10;
```

rust 允许 变量进行覆盖（shadowed）

```rust
let y = 4;
let y = "I can also be bound to text!"; // `y` is now of a different type.
```

Rust Const: 类型是必须出现，例如 u32， 不能使用 let 定义，需要使用 const 

```rust
const MAX_POINTS: u32 = 100_000;
```

rust 是 静态类型语言，在编译阶段必须知道所有变量的类型。

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

guess: u32 中的 u32则是帮助确定变量类型。

#### Scalar Types:

* integers
* floating-point numbers
* booleans
* characters.

##### Integer Types in Rust

<table><thead><tr><td> Length </td><td> Signed </td><td> Unsigned </td></tr></thead>
<tr><td> 8-bit  </td><td> i8     </td><td> u8       </td></tr>
<tr><td> 16-bit </td><td> i16    </td><td> u16      </td></tr>
<tr><td> 32-bit </td><td> i32    </td><td> u32      </td></tr>
<tr><td> 64-bit </td><td> i64    </td><td> u64      </td></tr>
<tr><td> arch   </td><td> isize  </td><td> usize    </td></tr>
</table>


##### Integer Literals in Rust
<table><thead><tr><td> Number literals  </td><td> Example       </td></tr></thead>
<tr><td> Decimal          </td><td> <code>98_222</code>      </td></tr>
<tr><td> Hex              </td><td> <code>0xff</code>        </td></tr>
<tr><td> Octal            </td><td> <code>0o77</code>        </td></tr>
<tr><td> Binary           </td><td> <code>0b1111_0000</code> </td></tr>
<tr><td> Byte (<code>u8</code> only) </td><td> <code>b'A'</code>        </td></tr>
</table>

#### Compound types
* tuples (类型可以不同)
* arrays (类型必须相同)

#### Tuples
```rust
 let tup: (i32, f64, u8) = (500, 6.4, 1);
 
 let x = tup.0
 let y = tup.1
 let z = tup.2
 
 let (x, y, z) = tup;
 
```

#### Array

Allocated on the stack

```rust
let a = [1, 2, 3, 4, 5];

let first = a[0];
let second = a[1];

let noexist = a[10] // compile ok, but run cause a panic
    
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

```

对于 Array 越界访问，编译的时候不会报错，但是运行的时候会 panic。 

### 3.2 Functions

无返回值函数定义：

	// no return value --> 注释 
	fn print_number(x: i32) {
    	println!("x is: {}", x);
	}

返回值函数定义：

	fn sum(x: i32, y: i32) -> i32 {
   		x + y
	}

rust中函数调用对于函数是否在调用者前定义没有要求，只要被定义了即可。在函数签名中，必须声明每个参数的类型。


#### Statements And Expressions

Statements 是执行一些操作但不返回值的指令:  let y = 6;

Expressions 计算并产生一个值: x + 1


### 3.3 Control Flow

#### if express

```rust
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
```


#### loop && while
```rust

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
```

#### for
```rust

    // for expression
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

```


### 4 Understanding Ownership

Ownership 是 Rust 最独特的功能，它使得 Rust 可以无需垃圾回收（garbage collector）就能保障内存安全。

#### Ownership Rules

* Each value in Rust has a variable that’s called its owner. 每个值都有一个称作owner的变量。
* There can only be one owner at a time.  每次只能有一个owner。
* When the owner goes out of scope, the value will be dropped. 当owner离开作用域，值被丢弃。


#### Ways Variables and Data Interact: Move (Shallow Copy)

```rust
	let x = 5;
	let y = x;  
	
	println!("x {} y {}", x,y)  // 因为x, y都为简单类型 scalar，分配在栈中，所以 x,y 仍然可以用
```

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}", s1);  // !ERROR, 因为 s1有内存分配在了heap上， s1 已经将 owner 转移到了s2，那么s1，将不再能够使用
```

Rust 永远也不会自动创建数据的“深拷贝”。因此，任何自动的复制可以被认为对运行时性能影响较小。

#### Ways Variables and Data Interact: Clone  (Deep Copy)

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

#### Stack-Only Data: Copy

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

Rust 有一个叫做Copy trait 的特殊注解，可以用在类似整型这样的储存在栈上的类型。如果一个类型拥有Copy trait，一个旧的变量在（重新）赋值后仍然可用。Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用Copy trait。如果我们对其值离开作用域时需要特殊处理的类型使用Copy注解，将会出现一个编译时错误。

那么什么类型是Copy的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则，任何简单标量值的组合可以是Copy的，任何不需要分配内存或类似形式资源的类型是Copy的，如下是一些Copy的类型：

* 所有整数类型，比如u32。
* 布尔类型，bool，它的值是true和false。
* 所有浮点数类型，比如f64。
* 元组，当且仅当其包含的类型也都是Copy的时候。(i32, i32)是Copy的，不过(i32, String)就不是。


#### Ownership and Functions

将值传递给函数在语言上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样。返回值可以转移作用域。

变量的所有权总是遵循相同的模式：将值赋值给另一个变量时move它，并且当持有堆中数据值的变量离开作用域时，如果数据的所有权没有被移动到另外一个变量时，其值将通过drop被清理掉。

```rust
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
    println!("s2 {}", s2)


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

```