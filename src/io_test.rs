use std::fs;

pub fn read_file() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went  wrong reading the file");
    println!("With test:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments")
        }

        // 这里使用clone的原因是:main才是程序参数args的所有者，如果我们在运行过程中夺取了它的控制权，就会违反借用规则
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}


// 我们移除了字符串切片&str


/*
todo 什么是借用规则
*/