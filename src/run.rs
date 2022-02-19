use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

pub fn run(path: &str) -> io::Result<()> {
    // 获取文件夹下内容
    let dir_items: Vec<PathBuf> = match read_dir(&path) {
        Ok(val) => val
            .map(|f| {
                f.unwrap().path()
            })
            .collect(),
        Err(err) => return Err(err),
    };

    let mut is_cargo_toml = false;
    let mut is_src_file = (false, 0);
    let mut rs_files: Vec<PathBuf> = Vec::new();
    // 判断是否为rust项目(以同时存在 `src/` 和 `Cargo.toml` 为依据)
    for (i, item) in dir_items.iter().enumerate() {
        let dispaly = item.display().to_string();
        if dispaly.ends_with("src") && item.is_dir() {
            is_src_file = (true, i);
        } else if dispaly.ends_with("Cargo.toml") {
            is_cargo_toml = true;
        };
    }

    if is_cargo_toml && is_src_file.0 {
        // 递归获取 `.rs` 文件
        find_rs_file(&dir_items[is_src_file.1], &mut rs_files)?;
        println!("{:#?}", rs_files);
    } else {
        println!(
            "Could not find `Cargo.toml` or `src` in `{}`",
            path
        );
    }

    Ok(())
}

fn find_rs_file(path: &PathBuf, rs_files: &mut Vec<PathBuf>) -> io::Result<()> {
    let dir_items: Vec<PathBuf> = match read_dir(path) {
        Ok(val) => val.map(|f| f.unwrap().path()).collect(),
        Err(err) => return Err(err),
    };

    for item in dir_items {
        if item.is_file() && item.extension().unwrap() == "rs" {
            rs_files.push(item);
        } else if item.is_dir() {
            find_rs_file(&item, rs_files)?;
        }
    }

    Ok(())
}
