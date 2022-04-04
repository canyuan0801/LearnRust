/**
 *  生命周期 see in chap10
 *
 * @author  canyuan
 */
#[test]
fn test_lifecycle1() {
    let str1 = String::from("long string is long");
    {
        let str2 = String::from("str2");
        let result = longest(str1.as_str(), str2.as_str());
        println!("The longest str is {}", result);
    }
}

// 这段代码会报错，如果longest返回的是str1还好，返回的是str2，那么就会报错
// #[test]
// fn test_lifecycle() {
//     let str1 = String::from("xyz");
//     let result;
//     {
//         let str2 = String::from("this is very long");
//         result = longest(str1.as_str(), str2.as_str());
//     }
//     println!("The longest str is {}", result);
// }

/**
这段代码不能通过编译，原因在于：传进去两个字符串切片(引用)，然后又返回引用，如果在调用这个longest函数之后
原先具有对x,y有所有权的变量离开作用域，那么仍然使用返回的&str将是一个悬垂引用。
这就是说我们不知道x,y的生命周期是如何与返回值的生命周期相关联的。
 */
// fn longest1(x: &str, y: &str) -> &str{
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }



/**
这段代码表示该函数所获取的两个字符串切片参数和返回的字符串切片的存活时间，必须不短于给定的生命周期'a
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}