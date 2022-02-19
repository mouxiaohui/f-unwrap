use std::env;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

use clap::{Arg, Command};

const OPT_PATH: &str = "PATH";

fn main() -> io::Result<()> {
    let current_dir = env::current_dir()?.display().to_string();

    // 获取命令行参数
    let matches = Command::new("f-unwrap")
        .about("Check the use of the 'unwrap()' function in your project")
        .arg(Arg::new(OPT_PATH).help("Project path").index(1))
        .get_matches();

    match matches.value_of(OPT_PATH) {
        Some(mut path) => {
            if path == "." {
                path = &current_dir;
            }
            run(path)?
        }
        None => run(&current_dir)?,
    };

    Ok(())
}

fn run(path: &str) -> io::Result<()> {
    // 获取文件夹下内容
    let dir_items: Vec<PathBuf> = match read_dir(path) {
        Ok(val) => val.map(|f| f.unwrap().path()).collect(),
        Err(err) => return Err(err),
    };

    for item in dir_items {
        if item.display().to_string().ends_with("src") {
            println!("{:?}", item);
        };
    }

    Ok(())
}
