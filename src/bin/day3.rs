use std::iter::Iterator;

fn main() {
    let input = advent_of_code_2022::load_day("3");

    println!("{}", sum_priorities_per_line(&input));
    println!("{}", sum_priorities_per_three_lines(&input));
}

fn sum_priorities_per_line(input: &str) -> u32 {
    input
        .split("\n")
        .map(common)
        .filter_map(|r| r.ok())
        .map(convert)
        .sum()
}

fn sum_priorities_per_three_lines(input: &str) -> u32 {
    input
        .split("\n")
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| common_n_lines(chunk.to_vec()))
        .filter_map(|r| r.ok())
        .map(convert)
        .sum()
}

fn common(line: &str) -> Result<char, &'static str> {
    let (first, second) = line.split_at(line.len() / 2);
    let mut c = Err("no chars");
    for a in first.chars() {
        if second.contains(a) {
            c = Ok(a);
        }
    }

    c
}

fn common_n_lines(lines: Vec<&str>) -> Result<char, &'static str> {
    let (head, tail) = lines.split_at(1);
    let mut c = Err("notchars");
    for a in head.first().unwrap().chars() {
        if tail.iter().all(|line| line.contains(a)) {
            c = Ok(a)
        }
    }
    c
}

fn convert(c: char) -> u32 {
    if c.is_lowercase() {
        return c as u32 - 96;
    }
    c as u32 - 64 + 26
}

#[cfg(test)]
mod tests {
    use crate::{common, common_n_lines, convert, sum_priorities_per_line};

    #[test]
    fn test_compartment_split() {
        let test = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let c = common(test);
        assert_eq!('p', c.unwrap());
    }

    #[test]
    fn test_convert_letter_to_priority() {
        let c = 'p';
        assert_eq!(16, convert(c));
    }

    #[test]
    fn test_it_run() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(157, sum_priorities_per_line(&input))
    }

    #[test]
    fn test_three_lines() {
        let test_a = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg"#;
        let input = test_a.split("\n").collect::<Vec<_>>();
        assert_eq!('r', common_n_lines(input).unwrap());

        let test_b = r#"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        let input = test_b.split("\n").collect::<Vec<_>>();
        assert_eq!('Z', common_n_lines(input).unwrap());
    }
}
