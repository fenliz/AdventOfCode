use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt")
        .expect("File not found!");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut previous_characters = String::new();

    for character in input.chars() {
        if previous_characters.is_empty() {
            previous_characters.push(character);
        } else {
            if let Some(previous_character) = previous_characters.pop() {
                if !will_react(character, previous_character) {
                    previous_characters.push(previous_character);
                    previous_characters.push(character);
                }
            }
        }
    }

    previous_characters.len()
}

fn will_react(c: char, c2: char) -> bool {
    c != c2 && c.to_ascii_lowercase() == c2.to_ascii_lowercase()
}

fn part2(input: &str) -> usize {
    "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|character| {
            let string_without_character = input
                .chars()
                .filter(|x| character != x.to_ascii_lowercase())
                .collect::<String>();

            part1(&string_without_character)
        })
        .min()
        .unwrap()
}