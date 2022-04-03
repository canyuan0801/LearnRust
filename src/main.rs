use std::cmp::Ordering;
use std::io;
use rand::Rng;
use crate::io_test::read_file;

mod functional;
mod io_test;

fn main() {
    read_file();
}
// this is where drop method is called





fn panic() {
    let v = vec![1, 2, 3];
    v[99];
}

//-----------------above is chap 9

fn vector_2() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),

    }
}

fn vector_1() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v1 = vec![1, 2, 3];



}

// ---------------------------above is chap 8

fn count_coin2(coin: Coin2) {
    let mut count = 0;
    if let Coin2::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn count_coin1(coin: Coin2) {
    let mut count = 0;
    match coin {
        Coin2::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

fn match_() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => ()
    }
}

fn use_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}


fn value_in_cents2(coin: Coin2) -> u32 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Coin1 {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents1(coin: Coin1) -> u32 {
    match coin {
        Coin1::Penny => {
            println!("Lucky Penny");
            1 // yes, must has a value be returned
        }
        Coin1::Nickel => 5,
        Coin1::Dime => 10,
        Coin1::Quarter => 25
    }
}

// above is 控制流运算符 match

fn option() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // this is wrong
    // let sum = x + y;
    // println!("{}", sum);
}

fn enum_4() {
    let m = Message::Write(String::from("hello"));
    m.call()
}

impl Message {
    fn call(&self) {}
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32, i32),
}

fn enum_3() {
    let home = IpAddKind3::V4(127, 0, 0, 1);
    let loopback = IpAddKind3::V6(String::from("::1"));
}

enum IpAddKind3 {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn enum_2() {
    let home = IpAddKind2::V4(String::from("127.0.0.1"));
    let loopback = IpAddKind2::V6(String::from("::1"));
}

enum IpAddKind2 {
    V4(String),
    V6(String),
}

fn enum_1() {
    let home = IpAddr {
        kind: IpAddKind1::V4,
        address: String::from("127.0.0.1"),
    };
}

struct IpAddr {
    kind: IpAddKind1,
    address: String,
}

fn route(ip_type: IpAddKind1) {}

enum IpAddKind1 {
    V4,
    V6,
}

//--------------------chap 6 above

/**
方法, differ from 函数
 */
fn compute_area_of_rectangle_4() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("长方形的面积是:{} square pixels", rect1.area());
}

fn compute_area_of_rectangle_3() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("The info of Rectangle is {:?}", rect1);
    println!("The info of Rectangle is {:#?}", rect1);

    println!("长方形的面积是:{} square pixels", area_2(rect1));
}

fn area_2(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn compute_area_of_rectangle_2() {
    let rect1 = (30, 50);
    println!("长方形的面积是:{} square pixels", area_1((30, 50)));
}

fn area_1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn compute_area_of_rectangle_1() {
    let width1 = 30;
    let height1 = 50;

    println!("长方形的面积是:{} square pixels", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn empty_struct() {}

/**
元组结构体
 */
fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);


fn build_user_from_others() {
    let user1 = build_user(String::from("canyuan0801@outlook.com"), String::from("canyuan"));
    let user2 = User {
        ..user1 // 结构体更新语法
    };

    println!("{}", user2.email);
}

fn build_user(email: String, username: String) ->
User {
    User {
        email, // 字段初始化简写(field init shorthand)
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn slice_string() {
    let s = "Hello";
    let slice = &s[3..s.len()];
    let slice = &s[3..];
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/*fn improved_first_world(s: &str) -> &str {

}*/

fn err_first_world() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    s.clear(); // 错误
    println!("the first word is {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        };
    }
    s.len()
}

fn err_move_exclude() {
    let x = 1;
    let y = x;
    println!("x is {}, y is {}", x, y);
}

fn clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1={}, s2={}", s1, s2);
}

/*fn err_move() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("The value is {}", s1);
}*/

fn append_string() {
    let mut s = String::from("hello rust");
    s.push_str(", let us go");
    println!("{}", s);
}

fn guess_ame() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is {}", secret_number);


    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
