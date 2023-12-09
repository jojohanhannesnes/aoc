use indicatif::{ParallelProgressIterator, ProgressIterator};
use rayon::prelude::*;

const INPUT: &str = include_str!("inputs/day5.txt");

fn almanac_reader() -> (Vec<i64>, Vec<Vec<Vec<i64>>>) {
    let mut seed_maps: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut candidate = Vec::new();
    let mut counter = 0;
    for (_, data) in INPUT.lines().enumerate() {
        match data {
            seed if data.starts_with("seeds") => {
                candidate = seed.split(':').collect::<Vec<&str>>()[1]
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            }
            _ if data.ends_with("map:") => {
                counter += 1;
                while seed_maps.len() < counter {
                    seed_maps.push(Vec::new()); // to prevent index out of bounds
                }
            }
            maps if data.starts_with(|x: char| x.to_digit(10).is_some()) => {
                let x = maps
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                seed_maps[counter - 1].push(x);
            }
            _ => {}
        }
    }
    (candidate, seed_maps)
}

fn get_corresponding_map(candidate: i64, maps: &mut Vec<Vec<Vec<i64>>>) -> i64 {
    if maps.len() == 0 {
        return candidate;
    }
    let mut result = candidate;
    for map in maps.iter().next().iter() {
        for x in map.iter() {
            if (x[1]..x[1] + x[2]).contains(&candidate) {
                result = x[0] + (candidate - x[1]);
                break;
            }
        }
    }

    maps.remove(0);
    get_corresponding_map(result, maps)
}

pub fn part1() -> i64 {
    let (candidate, almanac) = almanac_reader();
    let x = candidate
        .iter()
        .map(|x| get_corresponding_map(*x, &mut almanac.clone()))
        .collect::<Vec<i64>>();
    println!("x is : {x:?}");
    println!("x is : {}", x.iter().min().unwrap());

    1
}

pub fn part2() -> i64 {
    let (candidate, almanac) = almanac_reader();
    let mut new_c = Vec::new();
    for (i, _) in candidate
        .iter()
        .step_by(2)
        .enumerate()
        .into_iter()
        .progress()
    {
        let second = i * 2;
        new_c.par_extend(candidate[second]..candidate[second] + candidate[second + 1])
    }
    new_c.sort();
    new_c.dedup();
    let x = new_c
        .par_iter()
        .progress()
        .map(|x| get_corresponding_map(*x, &mut almanac.clone()))
        .collect::<Vec<i64>>();

    println!("new c is : {new_c:?}");
    println!("x is : {x:?}");
    println!("x is : {}", x.iter().min().unwrap());

    1
}

// #[test]
// fn test_part1() {
//     assert_eq!(part1(), 26273516)
// }

#[test]
fn test_part2() {
    assert_eq!(part2(), 1)
}
