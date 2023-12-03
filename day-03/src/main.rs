use std::collections::HashMap;
fn main() {
    let input = include_str!("./input.txt");
    println!("Solution part1: {}", solve_part1(input));
    println!("Solution part2: {}", solve_part2(input));
}


fn solve_part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;

    let mut upd_total = |value: u32 | {
        total += value;
    };


    for (y, line) in lines.iter().enumerate() {
        let line = line.to_string();

        let mut curr_number = 0;
        let mut is_valid = false;

        for (x, chr) in line.chars().enumerate() {
            if let Some(d) = chr.to_digit(10) {
                curr_number = curr_number * 10 + d as u32;
            } else if is_valid {
                upd_total(curr_number);
                curr_number = 0;
                is_valid = false;
                continue;
            } else {
                curr_number = 0;
                continue;
            }

            // check for validity
            let tx = x as i64;
            let ty = y as i64;

            let debug = x == 2 && y == 4;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    let px = tx + dx;
                    let py = ty + dy;
                    if debug {
                        println!("{py}:{px}");
                    }

                    if px < 0 || py < 0 || px as usize >= line.len() || py as usize >= lines.len() || (px == 0 && py == 0) {
                        continue;
                    }

                    let chr = lines[py as usize].chars().nth(px as usize).unwrap();
                    if debug {
                        println!("{py}:{px} = {chr}");
                    }
                    if !chr.is_digit(10) && chr != '.' {
                        is_valid = true;
                    }
                }
            }
        }

        if is_valid {
            upd_total(curr_number);
        }
    }
    total
}

fn solve_part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;

    let mut gear_locations: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut upd_location = |pos: (usize, usize), value: u32 | {
        if let Some(gears) = gear_locations.get_mut(&pos) {
            gears.push(value);
        } else {
            gear_locations.insert(pos, vec![value]);
        }
        total += value;
    };


    for (y, line) in lines.iter().enumerate() {
        let line = line.to_string();

        let mut curr_number = 0;
        let mut gear: Option<(usize, usize)> = None; // gear location for the number

        for (x, chr) in line.chars().enumerate() {
            if let Some(d) = chr.to_digit(10) {
                curr_number = curr_number * 10 + d as u32;

            } else if let Some(pos) = gear {
                upd_location(pos, curr_number);
                curr_number = 0;
                gear = None;
                continue;

            } else {
                curr_number = 0;
                gear = None;
                continue;
            }

            // check for validity
            let tx = x as i64;
            let ty = y as i64;

            let debug = x == 2 && y == 4;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    let px = tx + dx;
                    let py = ty + dy;
                    if debug {
                        println!("{py}:{px}");
                    }

                    if px < 0 || py < 0 || px as usize >= line.len() || py as usize >= lines.len() || (px == 0 && py == 0) {
                        continue;
                    }

                    let chr = lines[py as usize].chars().nth(px as usize).unwrap();

                    if chr == '*' {
                        gear = Some((px as usize, py as usize));
                    }

                }
            }
        }

        if let Some(pos) = gear {
            upd_location(pos, curr_number);
        }
    }

    let mut total = 0;
    for values in gear_locations.values() {
        if values.len() == 1 {
            continue
        }
        assert!(values.len() == 2);

        println!("Values: {values:?}");
        total += values[0] * values[1]
    }
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let got = solve_part1(input);

        assert_eq!(got, 4361);
    }


    #[test]
    fn part2_sample() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let got = solve_part2(input);

        assert_eq!(got, 467835);
    }
}
