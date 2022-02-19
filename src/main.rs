use clap::{Arg, Command};

const OPT_PATH: &str = "PATH";

fn main() {
    // 获取命令行参数
    let matches = Command::new("f-unwrap")
        .about("Check the use of the 'unwrap()' function in your project")
        .arg(Arg::new(OPT_PATH).help("Folder to check").index(1).default_value("."))
        .get_matches();

    match matches.value_of(OPT_PATH) {
        Some(path) => run(path),
        None => run(".")
    };
}

fn run(path: &str) {
    println!("{}", path);
}
