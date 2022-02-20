use std::{
    io::{stdout, Write},
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::Duration,
};

use colored::Colorize;

use crate::{finder::RsFile, run::Package};

const FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn draw(cargo_toml: &Package, rs_files: &Vec<RsFile>) {
    println!("{:?}", cargo_toml);
    println!("{:?}", rs_files);
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
                let f = format!("{}", FRAMES[i]).red();
                let _ = stdout.write(f.as_bytes());
                let _ = stdout.flush();
                i += 1;
                thread::sleep(interval);
            };
        }
    });
}
