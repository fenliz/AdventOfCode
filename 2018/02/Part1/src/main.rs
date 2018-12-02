use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").unwrap();
    let file_lines: Vec<String> = BufReader::new(file).lines().map(|file_line|
        file_line.unwrap()
    ).collect();

    let mut number_of_doublets = 0;
    let mut number_of_triplets = 0;
    for line in file_lines {
        let mut has_doublet = false;
        let mut has_triplet = false;

        for char in line.chars() {
            let count = line.matches(char).count();
            if count == 2 {
                has_doublet = true;
            } else if count == 3 {
                has_triplet = true;
            }
        }

        if has_doublet {
            number_of_doublets += 1;
        }
        if has_triplet {
            number_of_triplets += 1;
        }
    }

    println!("{}", number_of_doublets * number_of_triplets);
}
