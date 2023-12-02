fn main() {
    let input = include_str!("./input.txt");

    let solution_1 = part1(input);
    println!("Result p1: {solution_1}");

    let solution_2 = part2(input);
    println!("Result p2: {solution_2}");
}


fn into_digit(s: &str) -> String {
    let l = s.chars().find(char::is_ascii_digit).unwrap();
    let r = s.chars().rfind(char::is_ascii_digit).unwrap();
    return format!("{l}{r}");
}

fn part1(input: &str) -> String {
    let mut sum: u32 = 0;
    for digit in input.split("\n") {
        if digit == "" {
            continue
        }
        let digit = into_digit(digit);
        sum += digit.parse::<u32>().unwrap();
    }
    return sum.to_string();
}

fn extract_digits(line: &str) -> Vec<u8> {
    let spelled = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut results = Vec::with_capacity(2);

    for (pos, chr) in line.char_indices() {
        if chr.is_ascii_digit() {
            // found a digit
            results.push(chr.to_string().parse::<u8>().unwrap());
        }

        let (_, newstr) = line.split_at(pos);
        for (idx, s) in spelled.iter().enumerate() {
            if newstr.starts_with(s) {
                results.push(idx as u8 + 1);
                break;
            }
        }
    }
    return results;
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.split("\n") {
        if line == "" {
            continue
        }

        let digits = extract_digits(&line);
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();

        let d = first * 10 + last;
        sum += d as u32;
    }
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_input_sample() {
        let input = "1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet\
        ";
        let res = part1(input);

        assert_eq!(res, "142");
    }


    #[test]
    fn pass_input_p2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let res = part2(input);

        assert_eq!(res, "281");
    }
}
