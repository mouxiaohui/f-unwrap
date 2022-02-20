mod finder;
mod run;
mod ui;

use std::env;
use std::io;

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
            env::set_current_dir(path)?;

            let current_dir = env::current_dir()?.display().to_string();
            run::run(&current_dir)?
        }
        None => run::run(&current_dir)?,
    };

    Ok(())
}
