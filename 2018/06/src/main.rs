use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let file = read_to_string("input.txt")
        .expect("File not found!");
    let input: Vec<&str> = file.split("\r\n").collect();
    let coordinates: Vec<(i32, i32)> = input.iter()
        .map(|string| get_coordinate(string))
        .collect();

    let x_min = coordinates.iter().min_by_key(|(x, _)| x).unwrap().0;
    let x_max = coordinates.iter().max_by_key(|(x, _)| x).unwrap().0;
    let y_min = coordinates.iter().min_by_key(|(_, y)| y).unwrap().1;
    let y_max = coordinates.iter().max_by_key(|(_, y)| y).unwrap().1;

    println!("Part1: {}", part1(&coordinates, (x_min, y_min), (x_max, y_max)));
    println!("Part2: {}", part2(&coordinates, (x_min, y_min), (x_max, y_max)));
}

fn part1(coordinates: &Vec<(i32, i32)>, min: (i32, i32), max: (i32, i32)) -> i32 {
    let mut coordinate_areas: HashMap<(i32, i32), i32> = HashMap::new();
    let mut infinite_coordinates: Vec<(i32, i32)> = Vec::new();

    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            let mut closest_coordinate = None;
            let mut closest_distance = 100000;

            for &(cx, cy) in coordinates {
                let distance = (cx - x).abs() + (cy - y).abs();

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_coordinate = Some((cx, cy));
                } else if distance == closest_distance {
                    closest_coordinate = None;
                }
            }

            if let Some(c) = closest_coordinate {
                *coordinate_areas.entry(c).or_insert(0) += 1;
                if x == min.0 || x == max.0 || y == min.1 || y == max.1 {
                    infinite_coordinates.push(c);
                }
            }
        }
    }

    coordinate_areas = coordinate_areas.iter()
        .filter(|(coord, _)| !infinite_coordinates.contains(&coord))
        .map(|(coord, area)| (*coord, *area))
        .collect();

    let highest_area = coordinate_areas.iter()
        .max_by_key(|((_, _), area)| *area).unwrap();

    *highest_area.1
}

fn part2(coordinates: &Vec<(i32, i32)>, min: (i32, i32), max: (i32, i32)) -> i32 {
    let mut area = 0;

    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if coordinates.iter()
                .map(|&(cx, cy)| (x - cx).abs() + (y - cy).abs())
                .sum::<i32>() < 10000 {
                area += 1;
            }
        }
    }

    area
}

fn get_coordinate(string: &str) -> (i32, i32) {
    let parts: Vec<&str> = string.split(",").collect();
    let x = parts[0].trim().parse::<i32>().unwrap();
    let y = parts[1].trim().parse::<i32>().unwrap();
    (x, y)
}