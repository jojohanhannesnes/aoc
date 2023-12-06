const INPUT: &str = include_str!("inputs/day3.txt");

pub fn part1() -> i64 {
    let data = INPUT
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut lists: Vec<String> = Vec::new();
    let mut lists_not_included: Vec<String> = Vec::new();
    let mut temporary_lists = String::new();

    for (line, data_per_line) in data.iter().enumerate() {
        let mut is_adjacent_symbol = false;
        for (position, c) in data_per_line.iter().enumerate() {
            let right_position = position + 1;
            let bottom_position = line + 1;
            match c {
                digits if c.is_digit(10) => {
                    let string_digits = digits.to_string();
                    let new_digits = string_digits.as_str();
                    temporary_lists.push_str(new_digits);
                    // top
                    if let Some(prev_top) = line.checked_sub(1) {
                        let x = data[prev_top][position];
                        if !x.is_digit(10) && x != '.' && !is_adjacent_symbol {
                            is_adjacent_symbol = true;
                        }

                        // left
                        if let Some(prev_left) = position.checked_sub(1) {
                            let x = data[prev_top][prev_left];
                            if !x.is_digit(10) && x != '.' && !is_adjacent_symbol {
                                is_adjacent_symbol = true;
                            }

                            let y = data[line][prev_left];
                            if !y.is_digit(10) && y != '.' && !is_adjacent_symbol {
                                is_adjacent_symbol = true;
                            }
                        }

                        // right
                        if right_position <= data_per_line.len() - 1 {
                            let x = data[prev_top][right_position];
                            if !x.is_digit(10) && x != '.' && !is_adjacent_symbol {
                                is_adjacent_symbol = true;
                            }

                            let y = data[line][right_position];
                            if !y.is_digit(10) && y != '.' && !is_adjacent_symbol {
                                is_adjacent_symbol = true;
                            }
                        }
                    }

                    // bot
                    if bottom_position <= data.len() - 1 {
                        let x = data[bottom_position][position];
                        if !x.is_digit(10) && x != '.' && !is_adjacent_symbol {
                            is_adjacent_symbol = true;
                        }

                        // left
                        if let Some(prev_left) = position.checked_sub(1) {
                            let x = data[bottom_position][prev_left];
                            if !x.is_digit(10) && x != '.' && !is_adjacent_symbol {
                                is_adjacent_symbol = true;
                            }

                            let y = data[line][prev_left];
                            if !y.is_digit(10) && y != '.' && !is_adjacent_symbol {
                                is_adjacent_symbol = true;
                            }
                        }

                        // right
                        if right_position <= data_per_line.len() - 1 {
                            let x = data[bottom_position][right_position];
                            if !x.is_digit(10) && x != '.' && !is_adjacent_symbol {
                                is_adjacent_symbol = true;
                            }

                            let y = data[line][right_position];
                            if !y.is_digit(10) && y != '.' && !is_adjacent_symbol {
                                is_adjacent_symbol = true;
                            }
                        }
                    }
                }
                _ => {
                    if is_adjacent_symbol {
                        lists.push(std::mem::take(&mut temporary_lists));
                        is_adjacent_symbol = false;
                    } else {
                        if !temporary_lists.is_empty() {
                            lists_not_included.push(std::mem::take(&mut temporary_lists));
                        }
                    }
                }
            }
        }
        // eol number
        if is_adjacent_symbol {
            lists.push(std::mem::take(&mut temporary_lists));
        } else {
            if !temporary_lists.is_empty() {
                lists_not_included.push(std::mem::take(&mut temporary_lists));
            }
        }
    }
    lists.iter().map(|x| x.parse::<i64>().unwrap()).sum::<i64>()
}

fn find_number(line_data: &Vec<char>, position: usize) -> i64 {
    let mut full_number = String::new();

    // Iterate forward
    let mut i = position;
    while i < line_data.len() && line_data[i].is_digit(10) {
        full_number.push(line_data[i]);
        i += 1;
    }

    // Iterate backward
    let mut i = position.wrapping_sub(1); // handle 0
    while i < line_data.len() && line_data[i].is_digit(10) {
        full_number = line_data[i].to_string() + &full_number;
        i = i.wrapping_sub(1);
    }
    full_number.parse::<i64>().unwrap()
}

pub fn part2() -> i64 {
    let data = INPUT
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut lists: Vec<i64> = Vec::new();
    let mut temp_gear: Vec<i64> = Vec::new();

    for (line, data_per_line) in data.iter().enumerate() {
        for (position, c) in data_per_line.iter().enumerate() {
            let right_position = position + 1;
            let bottom_position = line + 1;
            let mut gear_counter = 0;
            match c {
                _ if *c == '*' => {
                    // top
                    if let Some(prev_top) = line.checked_sub(1) {
                        let x = data[prev_top][position];
                        if x.is_digit(10) {
                            let gears = find_number(&data[prev_top], position);
                            if !temp_gear.contains(&gears) {
                                temp_gear.push(gears);
                                gear_counter += 1;
                                if gear_counter == 2 {
                                    continue;
                                }
                            }
                        }

                        // top left
                        if let Some(prev_left) = position.checked_sub(1) {
                            let x = data[prev_top][prev_left];
                            if x.is_digit(10) {
                                let gears = find_number(&data[prev_top], prev_left);
                                if !temp_gear.contains(&gears) {
                                    temp_gear.push(gears);
                                    gear_counter += 1;
                                    if gear_counter == 2 {
                                        continue;
                                    }
                                }
                            }

                            // left
                            let z = data[line][prev_left];
                            if z.is_digit(10) {
                                let gears = find_number(&data[line], prev_left);
                                if !temp_gear.contains(&gears) {
                                    temp_gear.push(gears);
                                    gear_counter += 1;
                                    if gear_counter == 2 {
                                        continue;
                                    }
                                }
                            }
                        }

                        // top right
                        if right_position <= data_per_line.len() - 1 {
                            let x = data[prev_top][right_position];
                            if x.is_digit(10) {
                                let gears = find_number(&data[prev_top], right_position);
                                if !temp_gear.contains(&gears) {
                                    temp_gear.push(gears);
                                    gear_counter += 1;
                                    if gear_counter == 2 {
                                        continue;
                                    }
                                }
                            }

                            // right
                            let y = data[line][right_position];
                            if y.is_digit(10) {
                                let gears = find_number(&data[line], right_position);
                                if !temp_gear.contains(&gears) {
                                    temp_gear.push(gears);
                                    gear_counter += 1;
                                    if gear_counter == 2 {
                                        continue;
                                    }
                                }
                            }
                        }
                    }

                    // bot
                    if bottom_position <= data.len() - 1 {
                        let x = data[bottom_position][position];
                        if x.is_digit(10) {
                            let gears = find_number(&data[bottom_position], position);
                            if !temp_gear.contains(&gears) {
                                temp_gear.push(gears);
                                gear_counter += 1;
                                if gear_counter == 2 {
                                    continue;
                                }
                            }
                        }

                        // bot left
                        if let Some(prev_left) = position.checked_sub(1) {
                            let x = data[bottom_position][prev_left];
                            if x.is_digit(10) {
                                let gears = find_number(&data[bottom_position], prev_left);
                                if !temp_gear.contains(&gears) {
                                    temp_gear.push(gears);
                                    gear_counter += 1;
                                    if gear_counter == 2 {
                                        continue;
                                    }
                                }
                            }

                            // left
                            let z = data[line][prev_left];
                            if z.is_digit(10) {
                                let gears = find_number(&data[line], prev_left);
                                if !temp_gear.contains(&gears) {
                                    temp_gear.push(gears);
                                    gear_counter += 1;
                                    if gear_counter == 2 {
                                        continue;
                                    }
                                }
                            }
                        }

                        // bot right
                        if right_position <= data_per_line.len() - 1 {
                            let x = data[bottom_position][right_position];
                            if x.is_digit(10) {
                                let gears = find_number(&data[bottom_position], right_position);
                                if !temp_gear.contains(&gears) {
                                    temp_gear.push(gears);
                                    gear_counter += 1;
                                    if gear_counter == 2 {
                                        continue;
                                    }
                                }
                            }

                            // right
                            let y = data[line][right_position];
                            if y.is_digit(10) {
                                let gears = find_number(&data[line], right_position);
                                if !temp_gear.contains(&gears) {
                                    temp_gear.push(gears);
                                    gear_counter += 1;
                                    if gear_counter == 2 {
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            if temp_gear.len() == 2 {
                lists.push(temp_gear.iter().fold(1, |acc, &x| acc * x));
            }
            temp_gear.clear();
        }
    }
    lists.iter().sum::<i64>()
}

#[test]
fn test_part1() {
    assert_eq!(part1(), 535351);
}

#[test]
fn test_part2() {
    assert_eq!(part2(), 87287096);
}
