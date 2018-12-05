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
//
//    let file = File::open("input.txt").unwrap();
//    let mut file_lines: Vec<String> = BufReader::new(file).lines().map(|file_line|
//        file_line.unwrap()
//    ).collect();
//    file_lines.sort_by(|a, b| {
//        let a_parts: Vec<&str> = a.split(' ').collect();
//        let mut a_date_string: String = a_parts[0].chars().skip(1).collect();
//        let a_time_string: String = a_parts[1].chars().take(5).collect();
//        a_date_string = format!("{} {}:00", a_date_string, a_time_string);
//        let a_datetime = NaiveDateTime::parse_from_str(&a_date_string, "%Y-%m-%d %H:%M:%S").unwrap();
//
//        let b_parts: Vec<&str> = b.split(' ').collect();
//        let mut b_date_string: String = b_parts[0].chars().skip(1).collect();
//        let b_time_string: String = b_parts[1].chars().take(5).collect();
//        b_date_string = format!("{} {}:00", b_date_string, b_time_string);
//        let b_datetime = NaiveDateTime::parse_from_str(&b_date_string, "%Y-%m-%d %H:%M:%S").unwrap();
//
//        a_datetime.timestamp().partial_cmp(&b_datetime.timestamp()).unwrap()
//    });
//
//    println!("ASD");
//
//    let mut guard_sleep_minutes: HashMap<usize, i32> = HashMap::new();
//    let mut guard_sleep_schedule: HashMap<usize, Vec<i32>> = HashMap::new();
//    let mut time_asleep = 0;
//    let mut current_guard = 0;
//    for line in file_lines {
//        let parts: Vec<&str> = line.split(' ').collect();
//
//        if parts[3].contains("#") {
//            let guard_string: String = parts[3].chars().skip(1).collect();
//            current_guard = guard_string.parse::<usize>().unwrap();
//            if !guard_sleep_schedule.contains_key(&current_guard) {
//                guard_sleep_schedule.insert(current_guard, vec!(0; 60));
//                guard_sleep_minutes.insert(current_guard, 0);
//            }
//        }
//
//        let time_string: String = parts[1].chars().skip(3).take(2).collect();
//        let time = time_string.parse::<i32>().unwrap();
//
//        if parts[3].contains("asleep") {
//            time_asleep = time;
//        } else if parts[3].contains("up") {
//            let mut time_up = time;
//
//            for i in time_asleep..time_up {
//                let mut guard_schedule = guard_sleep_schedule.get_mut(&current_guard).unwrap();
//                guard_schedule[i as usize] += 1;
//                let mut guard_minutes = guard_sleep_minutes.get_mut(&current_guard).unwrap();
//                *guard_minutes += 1;
//            }
//        }
//    }
//
//    for guard in guard_sleep_schedule {
//        println!("#{}, {:?}", guard.0, guard.1);
//    }
//
//    let most_sleeper = guard_sleep_minutes.iter().max_by(|(x,x2), (y, y2)| x2.cmp(&y2)).unwrap();
//    println!("{:?}", guard_sleep_schedule[most_sleeper.0]);
//    let most_slept_minute = guard_sleep_schedule.get_mut(most_sleeper.0).unwrap().iter().max_by(|x, y| x.cmp(&y)).unwrap();
//    println!("{}", most_slept_minute);
//    println!("#{}", most_sleeper.0);
}
