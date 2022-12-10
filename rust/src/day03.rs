use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve() {
    let content = fs::read_to_string("../_inputs/day03.txt").unwrap();

    let items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let priorities: HashMap<char, usize> = items
        .chars()
        .enumerate()
        .map(|(index, letter)| (letter, index + 1))
        .collect();

    let priorities_sum = content
        .trim()
        .lines()
        .map(|items| {
            let size = items.len();

            if size % 2 != 0 {
                panic!("invalid rucksack content, items should be even");
            }

            let (compartment_one, compartment_two) = items.split_at(size / 2);

            let compartment_one: HashSet<_> = compartment_one.chars().collect();
            let compartment_two: HashSet<_> = compartment_two.chars().collect();

            let item_on_both_compartments = compartment_one
                .into_iter()
                .filter(|item| compartment_two.contains(item))
                .collect::<Vec<_>>()[0];

            *priorities
                .get(&item_on_both_compartments)
                .unwrap_or_else(|| panic!("invalid item {}", item_on_both_compartments))
        })
        .collect::<Vec<_>>()
        .iter()
        .sum::<usize>();

    println!("[day03.p1] priorities sum: {}", priorities_sum);
}
