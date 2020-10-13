use clap::{App, Arg};

// clap 库学习
// doc https://docs.rs/clap/3.0.0-beta.2/clap/index.html
fn main() {
    let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::new("INPUT")
            .about("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::new("v")
            .short('v')
            .multiple(true)
            .about("Sets the level of verbosity"))
        .subcommand(App::new("test")
            .about("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::new("debug")
                .short('d')
                .about("print debug information verbosely")))
        .get_matches();

    // 如果用户提供、则获取该值作为config，或者默认使用 “default.conf”
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // 在这里调用.unwrap（）是安全的，因为需要“ INPUT”（如果不需要“ INPUT”，
    // 可以使用 “if let” 有条件地获取值）
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // 根据用户使用“详细”标志的次数来改变输出
    // (比如 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // 你可以通过以下方式处理有关子命令的信息：按名称请求它们的匹配（如下所示）
    // 仅请求正在使用的名称或两者同时请求
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // 其他程序逻辑...
}
