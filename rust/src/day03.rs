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

    let ruckstacks: Vec<_> = content
        .trim()
        .lines()
        .map(|line| {
            if line.len() % 2 != 0 {
                panic!("invalid rucksack content, items should be even");
            }
            line
        })
        .collect();

    if ruckstacks.len() % 3 != 0 {
        panic!("invalid rucksacks, should be able to form groups of 3");
    }

    let priorities_sum = ruckstacks
        .iter()
        .map(|items| {
            let size = items.len();
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

    let group_priorities_sum = ruckstacks
        .chunks_exact(3)
        .map(|group| {
            let one: HashSet<_> = group[0].chars().collect();
            let two: HashSet<_> = group[1].chars().collect();
            let three: HashSet<_> = group[2].chars().collect();

            let common_item = one
                .into_iter()
                .filter(|item| two.contains(item) && three.contains(item))
                .collect::<Vec<_>>()[0];

            *priorities
                .get(&common_item)
                .unwrap_or_else(|| panic!("invalid item {}", common_item))
        })
        .collect::<Vec<_>>()
        .iter()
        .sum::<usize>();

    println!("[day03.p1] priorities sum: {}", priorities_sum);
    println!("[day03.p2] group priorities sum: {}", group_priorities_sum);
}
