use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process::exit;

fn main() {
    let file = File::open("input.txt").unwrap();
    let file_lines: Vec<String> = BufReader::new(file).lines().map(|file_line|
        file_line.unwrap()
    ).collect();

    let mut number_of_doublets = 0;
    let mut number_of_triplets = 0;

    for i in 0..file_lines.first().unwrap().len() {
        let mut partial_lines = Vec::new();
        for line in &file_lines {
            let mut prefix: String = line.chars().take(i).collect();
            let mut suffix: String = line.chars().skip(i + 1).collect();
            let mut partial_line = prefix + &suffix;

            if partial_lines.contains(&partial_line) {
                println!("{}", partial_line);
                exit(0);
            }
            partial_lines.push(partial_line);
        }
    }
}
