fn sum_elves_calories(input: &str) -> i32 {
    input
        .split("\n")
        .map(str::parse::<i32>)
        .filter_map(|res| res.ok())
        .fold(0, |before, current| before + current)
}

fn part1(input: &str) {
    let max = input
        .split("\n\n")
        // Each elves is associated to a sum
        .map(sum_elves_calories)
        .fold(
            0,
            |before, current| {
                if current > before {
                    current
                } else {
                    before
                }
            },
        );
    println!("{}", max);
}

fn part2(input: &str) {
    let mut calories_per_elf = input
        .split("\n\n")
        // Each elves is associated to a sum
        .map(sum_elves_calories)
        .collect::<Vec<_>>();

    calories_per_elf.sort();

    let mut reversed = calories_per_elf.iter().rev();
    println!(
        "Top 3 {}",
        reversed.next().unwrap() + reversed.next().unwrap() + reversed.next().unwrap(),
    );
}

fn main() {
    let input = advent_of_code_2022::load_day("1");
    part1(&input);
    part2(&input);
}
