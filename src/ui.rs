use std::io::{stdout, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use colored::{ColoredString, Colorize};

use crate::{finder::RsFile, run::Package};

const FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn draw(cargo_toml: &Package, rs_files: &Vec<RsFile>) {
    // 打印项目信息
    draw_package_info(cargo_toml);

    let mut total_unwraps = 0;
    for file in rs_files {
        total_unwraps += file.unwraps();
    }
    println!("\nTotal: {}", total_unwraps.to_string().yellow());

    for file in rs_files {
        let path_string = file.path.display().to_string();
        let file_name = match file.path.file_name() {
            Some(file_name) => {
                format!(" > {}", file_name.to_str().unwrap()).truecolor(110, 81, 254)
            }
            None => format!(" > {}", path_string).truecolor(110, 81, 254),
        };
        println!("{}", file_name);

        for (i, location) in file.unwrap_location.iter().enumerate() {
            // 为每行 .unwrap() 标黄
            let split_line = location
                .line
                .trim()
                .split(".unwrap()")
                .collect::<Vec<&str>>();
            let mut line_color_string: Vec<ColoredString> =
                split_line.iter().map(|s| s.green()).collect();
            line_color_string.insert(1, ".unwrap()".yellow().underline());

            print!(
                "  {:>2}. {}:{}:{} => ",
                i + 1,
                path_string,
                location.row,
                location.column,
            );
            for line in line_color_string {
                print!("{}", line);
            }
            println!("")
        }
    }
}

fn draw_package_info(cargo_toml: &Package) {
    println!("{}", cargo_toml.name.on_truecolor(110, 81, 254));
    println!("version: {}", cargo_toml.version.truecolor(110, 81, 254));
    println!("edition: {}", cargo_toml.edition.truecolor(110, 81, 254));

    if let Some(author) = &cargo_toml.author {
        println!("author: {}", author.truecolor(110, 81, 254));
    } else if let Some(authors) = &cargo_toml.authors {
        let mut aus = String::from("[");
        for author in authors {
            aus.push_str(&format!("\"{}\", ", author));
        }
        aus.push(']');
        println!("authors: {}", aus.truecolor(110, 81, 254));
    }
}

pub struct Loading {
    sender: Option<Sender<String>>,
    interval: Duration,
}

impl Loading {
    pub fn new(interval: Duration) -> Self {
        Self {
            sender: None,
            interval,
        }
    }

    pub fn start(&mut self) {
        let (sender, receiver) = mpsc::channel();
        self.sender = Some(sender);
        loading(receiver, self.interval);
    }

    pub fn end(&mut self) {
        if let Some(sender) = &self.sender {
            sender.send("END".to_string()).unwrap();
            let mut stdout = stdout();
            let _ = stdout.write(b"\x1B[2K\x1B[0G");
            let _ = stdout.flush();
        }
    }
}

pub fn loading(receiver: Receiver<String>, interval: Duration) {
    let mut stdout = stdout();

    let mut i = 0;

    thread::spawn(move || {
        loop {
            if let Ok(signal) = receiver.try_recv() {
                if signal == "END" {
                    break;
                };
            } else {
                if i > FRAMES.len() - 1 {
                    i = 0
                };

                // 清除上次输出
                let _ = stdout.write(b"\x1B[2K\x1B[0G");
                let f = format!("{}", FRAMES[i]);
                let _ = stdout.write(f.as_bytes());
                let _ = stdout.flush();
                i += 1;
                thread::sleep(interval);
            };
        }
    });
}
