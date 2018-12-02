use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").unwrap();
    let numbers: Vec<i32> = BufReader::new(file).lines().map(|file_line|
        file_line.unwrap().parse::<i32>().unwrap()
    ).collect();

    let mut common_frequency_found = false;
    let mut past_frequencies = Vec::new();
    past_frequencies.push(0);
    while !common_frequency_found {
        for number in &numbers {
            let frequency = past_frequencies.last().unwrap() + number;
            if past_frequencies.contains(&frequency) {
                common_frequency_found = true;
                println!("{}", frequency);
                break;
            }
            past_frequencies.push(frequency);
        }
    }
}
