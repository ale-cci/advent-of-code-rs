use std::cmp;
fn main() {
    let input = include_str!("./input.txt");
    let sol_p1 = part_1(input);
    println!("Solution pt1: {sol_p1}");

    let sol_p2 = part_2(input);
    println!("Solution pt2: {sol_p2}");
}

fn part_1(input: &str) -> u32 {
    let mut sum_of_valid = 0;
    for line in input.split("\n") {
        if line == "" {
            continue;
        }
        let (header, values) = line.split_once(": ").unwrap();
        let (_, game_id) = header.split_once(" ").unwrap();
        let game_id = game_id.parse::<u32>().unwrap();

        let mut valid = true;
        for game in values.split("; ") {
            valid &= is_valid_config(game);
            if !valid {
                break;
            }
        }

        if valid {
            sum_of_valid += game_id;
        }
    }

    return sum_of_valid;
}

fn part_2(input: &str) -> u32 {
    let mut cubes_sum = 0;
    for line in input.split("\n") {
        if line == "" {
            continue;
        }
        let (_, gameline) = line.split_once(": ").unwrap();

        cubes_sum += fewest_number(gameline);
    }

    return cubes_sum;
}

fn is_valid_config(extr: &str) -> bool {
    for chunk in extr.split(", ") {
        let mut id_color = chunk.split(char::is_whitespace);

        let count = id_color.next().unwrap().parse::<u8>().unwrap();
        let color = id_color.next().unwrap();

        let max_allowed = match color {
            "red" => 12,
            "green" => 13,
            "blue" => 14,
            _ => panic!("Unrecognized color {}", color),
        };

        if count > max_allowed {
            return false;
        }
    }
    true
}

fn fewest_number(gameline: &str) -> u32 {
    let mut colors = [0, 0, 0];
    for game in gameline.split("; ") {

        for cubes in game.split(", ") {
            let (count, color) = cubes.split_once(" ").unwrap();

            let color_id = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!("unhandled color {color}"),
            };

            let count = count.parse::<u32>().unwrap();
            colors[color_id] = std::cmp::max(colors[color_id], count);
        }
    }

    return colors[0] * colors[1] * colors[2];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_possible_game() {
        let got = is_valid_config("3 blue, 4 red");
        assert_eq!(got, true);
    }

    #[test]
    fn calculates_possible_games() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let got = part_1(&input);
        assert_eq!(got, 8);
    }

    #[test]
    fn calculates_fewer_number_of_possible_cubes() {
        let input = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let got = fewest_number(input);
        assert_eq!(got, 48);
    }
}

