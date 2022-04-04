use std::{fs, process};
use std::error::Error;


pub fn read_file() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(
        // 闭包参数写在两条竖线之间
        |err| {
        println!("Problem parsing arguments:{}", err);
        // 传递非0值给父进程以退出程序
        process::exit(1)
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // 这里使用if let是因为返回的是()，没有必要调用unwrap_or_else把这个必定是()的值取出来
    if let Err(e) = run(config) {
        println!("Application error:{}", e);
        process::exit(1);
    }
}

/**
返回的ok中有空元组，是为了产生函数副作用，而不是为了返回任何有用的值
*/
fn run(config: Config) -> Result<(), Box<dyn Error>>{
    // ?可以将错误值返回给函数的调用者来进行处理
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

/**
'a是一个显式生命周期
 */
pub fn search<'a> (query:&str, contents:&'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        // do something with line
        if line.contains(query) {
            // 使用line执行某些操作
            results.push(line)
        }
    }
    results
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    /**
        &'static是字符串字面量的类型, see in chap 10
    运行失败会返回&'static 的str
    */
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        // 这里使用clone的原因是:main才是程序参数args的所有者，如果我们在运行过程中夺取了它的控制权，就会违反借用规则
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}


// 我们移除了字符串切片&str


/*
todo 什么是借用规则
*/