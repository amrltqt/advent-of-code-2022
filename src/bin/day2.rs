enum Shape {
    Rock,
    Paper,
    Scissor,
}

enum Result {
    Win,
    Draw,
    Loss,
}

impl From<&str> for Result {
    fn from(raw: &str) -> Self {
        match raw {
            "Z" => Self::Win,
            "Y" => Self::Draw,
            "X" => Self::Loss,
            _ => panic!("wtf?"),
        }
    }
}

impl From<&str> for Shape {
    fn from(raw: &str) -> Self {
        match raw {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => panic!("wtf?"),
        }
    }
}

fn score(opponent: Shape, me: Shape) -> i32 {
    match opponent {
        Shape::Rock => match me {
            Shape::Rock => 1 + 3,
            Shape::Paper => 2 + 6,
            Shape::Scissor => 3 + 0,
        },
        Shape::Paper => match me {
            Shape::Rock => 1 + 0,
            Shape::Paper => 2 + 3,
            Shape::Scissor => 3 + 6,
        },
        Shape::Scissor => match me {
            Shape::Rock => 1 + 6,
            Shape::Paper => 2 + 0,
            Shape::Scissor => 3 + 3,
        },
    }
}

fn part1(input: &str) {
    let total = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut drain = line.split(" ");
            let opponent = Shape::from(drain.next().unwrap());
            let me = Shape::from(drain.next().unwrap());

            score(opponent, me)
        })
        .fold(0, |b, c| b + c);

    println!("{}", total);
}

fn score_part2(opponent: Shape, result: Result) -> i32 {
    match opponent {
        Shape::Rock => match result {
            Result::Win => 2 + 6,
            Result::Draw => 1 + 3,
            Result::Loss => 3 + 0,
        },
        Shape::Paper => match result {
            Result::Win => 3 + 6,
            Result::Draw => 2 + 3,
            Result::Loss => 1 + 0,
        },
        Shape::Scissor => match result {
            Result::Win => 1 + 6,
            Result::Draw => 3 + 3,
            Result::Loss => 2 + 0,
        },
    }
}

fn part2(input: &str) {
    let total = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut drain = line.split(" ");
            let opponent = Shape::from(drain.next().unwrap());
            let result = Result::from(drain.next().unwrap());

            score_part2(opponent, result)
        })
        .fold(0, |b, c| b + c);

    println!("{}", total);
}

fn main() {
    let input = advent_of_code_2022::load_day("2");

    part1(&input);
    part2(&input);
}
