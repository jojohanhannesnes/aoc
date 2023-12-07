const INPUT: &str = include_str!("inputs/day4.txt");

fn convert(data: &str) -> Vec<i64> {
    data.trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn part1() -> i64 {
    INPUT
        .lines()
        .map(|card| {
            let mut counter = 0;
            let data = card.split(['|', ':']).collect::<Vec<&str>>();
            let winnings = convert(data[1]);
            let candidates = convert(data[2]);
            candidates.iter().for_each(|c| {
                if winnings.contains(c) {
                    counter += 1;
                }
            });
            if counter > 0 {
                2_i64.pow(counter - 1)
            } else {
                0
            }
        })
        .sum::<i64>()
}

pub fn part2() -> i32 {
    let mut packs = vec![1; INPUT.lines().count()];
    INPUT.lines().enumerate().for_each(|(i, card)| {
        let mut matched = 0;
        let data = card.split(['|', ':']).collect::<Vec<&str>>();
        let winnings = convert(data[1]);
        let candidates = convert(data[2]);
        candidates.iter().for_each(|c| {
            if winnings.contains(c) {
                matched += 1;
            }
        });
        let total_with_clone = packs[i];
        let add_to_pack = &mut packs[i + 1..i + 1 + matched];
        for i in add_to_pack.iter_mut() {
            *i += total_with_clone;
        }
        // println!("packs when on line {i}: {packs:?}");
    });

    packs.iter().sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 17803)
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 5554894)
}
