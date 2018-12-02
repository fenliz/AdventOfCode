use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut result: i32 = 0;
    let file = File::open("input.txt").unwrap();
    for file_line in BufReader::new(file).lines() {
        match file_line {
            Ok(string) => result += string.parse::<i32>().unwrap(),
            _ => ()
        }
    }
    println!("{}", result);
}
