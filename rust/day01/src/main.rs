use std::fs;

fn main() {
    let content = fs::read_to_string("../../_inputs/day01.txt").unwrap();

    let max_calories = content
        .trim()
        .split("\n\n")
        .map(|elf| {
            let calories: u32 = elf
                .split("\n")
                .map(|cal_str| cal_str.parse::<u32>().unwrap())
                .sum();

            calories
        })
        .max()
        .unwrap();

    println!("{}", max_calories);
}
