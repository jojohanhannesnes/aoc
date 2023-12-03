use aho_corasick::AhoCorasick;

const INPUT: &str = include_str!("inputs/day1.txt");

pub fn part1() -> i32 {
    INPUT
        .lines()
        .map(|data| {
            let first = data.chars().find(|x| x.is_digit(10)).unwrap();
            let last = data.chars().rev().find(|x| x.is_digit(10)).unwrap();
            format!("{}{}", first, last).parse::<i32>().unwrap()
        })
        .sum()
}

pub fn part2() -> i32 {
    let patterns: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();
    INPUT
        .lines()
        .map(|haystack| {
            let matches = ac.find_overlapping_iter(haystack).collect::<Vec<_>>();
            let first = if matches.iter().nth(0).unwrap().pattern().as_usize() < 9 {
                patterns[matches.iter().nth(0).unwrap().pattern().as_usize() + 9]
            } else {
                patterns[matches.iter().nth(0).unwrap().pattern().as_usize()]
            };
            let last = if matches.iter().last().unwrap().pattern().as_usize() < 9 {
                patterns[matches.iter().last().unwrap().pattern().as_usize() + 9]
            } else {
                patterns[matches.iter().last().unwrap().pattern().as_usize()]
            };
            format!("{}{}", first, last).parse::<i32>().unwrap()
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 55090)
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 54845)
}
