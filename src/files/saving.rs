use std::fs::File;
use std::io::{self, Read, Write};

pub fn slice_to_string(slice: &[u32]) -> String {
    slice
        .iter()
        .map(|highscore|highscore.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(
            |nb|nb.parse::<u32>().ok()
        )
        .collect()
}

pub fn load_highscores_and_lines(file_name: &str) -> Option<(Vec<u32>, Vec<u32>)>{
    if let Ok(content) = read_from_file(file_name) {
        let mut lines = content.splitn(2, "\n").map(|line|line_to_slice(line)).collect::<Vec<_>>();
        if lines.len() == 2 {
            let (num_lines, highscores) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((highscores, num_lines))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = match File::create(file_name) {
        Ok(value) => value,
        Err(error) => return Err(error)
    };
    f.write_all(content.as_bytes())
}


pub fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

pub fn save_highscore_and_lines(highscore: &[u32], num_of_lines: &[u32], file_name: &str) -> bool {
    let s_highscores = slice_to_string(highscore);
    let s_num_of_lines = slice_to_string(num_of_lines);
    write_into_file(format!("{}\n{}\n", s_highscores, s_num_of_lines).as_str(), file_name).is_ok()
}