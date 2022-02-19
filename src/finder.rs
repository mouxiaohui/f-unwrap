use std::io;
use std::path::PathBuf;

use grep::regex::RegexMatcher;
use grep::searcher::sinks::UTF8;
use grep::searcher::Searcher;

// .unwrap()
const RS_UNWRAP_PATTERN: &str = r#"(.unwrap\(\))"#;
const MATCH_INDICES: &str = ".unwrap()";

#[derive(Debug)]
pub struct RsFile {
    name: String,
    unwrap_location: Vec<UnwrapLocation>,
}

#[derive(Debug)]
pub struct UnwrapLocation {
    pub row: u64,
    pub column: u64,
}

pub fn find_unwraps(paths: &Vec<PathBuf>) -> io::Result<Vec<RsFile>> {
    let mut rs_file = Vec::new();
    for path in paths {
        if let Some(f) = parse_file(path) {
            rs_file.push(f);
        };
    }

    Ok(rs_file)
}

fn parse_file(path: &PathBuf) -> Option<RsFile> {
    // 创建匹配器
    let matcher = match RegexMatcher::new(RS_UNWRAP_PATTERN) {
        Ok(m) => m,
        Err(err) => panic!("{}", err),
    };

    let mut unwrap_location = Vec::new();
    Searcher::new()
        .search_path(
            matcher,
            path,
            UTF8(|row, line| {
                // 查询 .unwrap() 在一行的第几列
                for (u, _) in line.match_indices(MATCH_INDICES).collect::<Vec<_>>() {
                    unwrap_location.push(UnwrapLocation {
                        row,
                        column: u as u64,
                    });
                }

                Ok(true)
            }),
        )
        .unwrap();

    let filename = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => path.display().to_string(),
    };

    Some(RsFile {
        name: filename,
        unwrap_location: unwrap_location,
    })
}
