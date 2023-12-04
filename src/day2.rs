use regex::Regex;

const INPUT: &str = include_str!("inputs/day2.txt");

fn parser(re: &Regex, lists: &str, limit: i32) -> bool {
    re.find_iter(lists)
        .map(|x| {
            x.as_str()
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .all(|x| x <= limit)
}

fn parser_part2(re: &Regex, lists: &str) -> i32 {
    re.find_iter(lists)
        .map(|x| {
            x.as_str()
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .max()
        .unwrap()
}
pub fn part1() -> i32 {
    let red_regex = Regex::new(r"\d+ red").unwrap();
    let green_regex = Regex::new(r"\d+ green").unwrap();
    let blue_regex = Regex::new(r"\d+ blue").unwrap();
    INPUT
        .lines()
        .filter_map(|x| {
            let (game, lists) = x.split_once(':').unwrap();
            let red = parser(&red_regex, lists, 12);
            let green = parser(&green_regex, lists, 13);
            let blue = parser(&blue_regex, lists, 14);
            if red && green && blue {
                let (_, game) = game.split_at(5);
                Some(game.parse::<i32>().unwrap())
            } else {
                None
            }
        })
        .sum::<i32>()
}

pub fn part2() -> i32 {
    let red_regex = Regex::new(r"\d+ red").unwrap();
    let green_regex = Regex::new(r"\d+ green").unwrap();
    let blue_regex = Regex::new(r"\d+ blue").unwrap();
    INPUT
        .lines()
        .map(|x| {
            let (_, lists) = x.split_once(':').unwrap();
            let max_red = parser_part2(&red_regex, lists);
            let max_green = parser_part2(&green_regex, lists);
            let max_blue = parser_part2(&blue_regex, lists);
            max_red * max_green * max_blue
        })
        .sum::<i32>()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 2486);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 87984);
}
