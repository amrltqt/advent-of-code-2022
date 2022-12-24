fn main() {
    let input = advent_of_code_2022::load_day("6");
    part1(&input);
    part2(&input);
}

fn is_unique(seq: &str) -> bool {
    let mut seens = Vec::new();
    for c in seq.chars() {
        if seens.contains(&c) {
            return false
        }
        seens.push(c);
    };
    return true;
}

fn have_unique_seq(seq: &str, size: usize) -> usize {
    for i in 0..seq.len() - size {
        if is_unique(&seq[i..i + size]) {
            return i + size
        }
    }
    seq.len()
}

fn part1(input: &str) {
    let value = input.split("\n")
         .map(|line| have_unique_seq(line, 4))
         .min()
         .unwrap();

    println!("{}", value);
}

fn part2(input: &str) {
    let value = input.split("\n")
         .map(|line| have_unique_seq(line, 14))
         .min()
         .unwrap();

    println!("{}", value);
}


#[cfg(test)]
mod tests {
    use crate::{is_unique, have_unique_seq};

    #[test]
    fn test_seq_unique() {
        assert_eq!(is_unique("abcd"), true);
        assert_eq!(is_unique("abcb"), false);
    }

    #[test]
    fn test_have_unique_seq() {
        assert_eq!(have_unique_seq("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    
        assert_eq!(have_unique_seq("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(have_unique_seq("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(have_unique_seq("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(have_unique_seq("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }
}