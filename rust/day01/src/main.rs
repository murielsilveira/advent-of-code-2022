use std::fs;

fn main() {
    let content = fs::read_to_string("../../_inputs/day01.txt").unwrap();

    let mut elves = content
        .trim()
        .split("\n\n")
        .map(|elf| {
            let calories: u32 = elf
                .split("\n")
                .map(|cal_str| cal_str.parse::<u32>().unwrap())
                .sum();

            calories
        })
        .collect::<Vec<_>>();

    elves.sort();

    let top_one_calories = &elves[elves.len() - 1..].iter().sum::<u32>();
    let top_three_calories_sum = &elves[elves.len() - 3..].iter().sum::<u32>();

    println!("day01_1 {:?}", top_one_calories);
    println!("day01_2 {:?}", top_three_calories_sum);
}
