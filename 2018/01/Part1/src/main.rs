use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut result: i32 = 0;
    let file = File::open("input.txt").unwrap();
    BufReader::new(file).lines().for_each(|file_line|
        result += file_line.unwrap().parse::<i32>().unwrap()
    );
    println!("{}", result);
}
