extern crate chrono;

use std::collections::HashMap;
use std::fs::read_to_string;
use chrono::NaiveDateTime;

fn get_timestamp(record: &str) -> i64 {
    let datetime_string = record.chars().take(18).collect::<String>();
    NaiveDateTime::parse_from_str(&datetime_string, "[%Y-%m-%d %H:%M]").unwrap().timestamp()
}

fn get_minute(record: &str) -> u32 {
    let time_string = record.chars().skip(15).take(2).collect::<String>();
    time_string.parse::<u32>().unwrap()
}

fn get_guard_id(record: &str) -> usize {
    let parts = record.split(" ").collect::<Vec<&str>>();
    let guard_id_string = parts[3].chars().skip(1).collect::<String>();
    guard_id_string.parse::<usize>().unwrap()
}

fn part1(guard_sleep_schedule: &HashMap<usize, HashMap<u32, u32>>) -> usize {
    let guard_with_most_sleep = *guard_sleep_schedule
        .iter()
        .map(|(id, minutes)| (id, minutes.values().sum::<u32>()))
        .max_by_key(|(_, minutes)| *minutes)
        .unwrap().0;
    let the_guards_sleepiest_minute = *guard_sleep_schedule[&guard_with_most_sleep]
        .iter()
        .max_by_key(|(_, times)| *times)
        .unwrap().0 as usize;

    guard_with_most_sleep * the_guards_sleepiest_minute
}

fn part2(guard_sleep_schedule: &HashMap<usize, HashMap<u32, u32>>) -> usize {
    let (guard, minute) = guard_sleep_schedule
        .iter()
        .map(|(id, minutes)| (id, minutes.iter().max_by_key(|(_, times)| *times).unwrap()))
        .max_by_key(|(_, (_, times))| *times)
        .map(|(id, (min, _))| (*id, *min as usize))
        .unwrap();

    guard * minute
}

fn main() {
    let input = read_to_string("input.txt")
        .expect("File not found!");

    let mut records = input.split("\n").collect::<Vec<&str>>();
    records.sort_by(|a, b| {
        let a_timestamp = get_timestamp(a);
        let b_timestamp = get_timestamp(b);
        a_timestamp.partial_cmp(&b_timestamp).unwrap()
    });

    let mut guard_sleep_schedule: HashMap<usize, HashMap<u32, u32>> = HashMap::new();
    let mut time_when_guard_went_to_sleep = 0;
    let mut current_guard_on_duty = 0;
    for record in records {
        let command = record.chars().skip(19).collect::<String>();
        match command.trim() {
            "falls asleep" => time_when_guard_went_to_sleep = get_minute(record),
            "wakes up" => {
                for i in time_when_guard_went_to_sleep..get_minute(record) {
                    *guard_sleep_schedule.entry(current_guard_on_duty)
                        .or_default()
                        .entry(i)
                        .or_insert(0) += 1;
                }
            }
            _ => current_guard_on_duty = get_guard_id(record),
        }
    }

    println!("Part1: {}", part1(&guard_sleep_schedule));
    println!("Part2: {}", part2(&guard_sleep_schedule));
}
