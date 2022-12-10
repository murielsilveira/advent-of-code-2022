use std::fs;

pub fn solve() {
    let content = fs::read_to_string("../_inputs/day02.txt").unwrap();

    let rounds = content.trim().split('\n').map(|round| {
        let round: Vec<_> = round.split(' ').collect();

        let opponent = match round[0] {
            "A" => "R",
            "B" => "P",
            "C" => "S",
            shape => panic!("invalid shape {}", shape),
        };
        let me = match round[1] {
            "X" => "R",
            "Y" => "P",
            "Z" => "S",
            shape => panic!("invalid shape {}", shape),
        };

        (opponent, me)
    });

    let outcomes = rounds.map(|(opponent, me)| {
        let shape_points = match me {
            "R" => 1,
            "P" => 2,
            "S" => 3,
            shape => panic!("invalid shape {}", shape),
        };

        let round_points = match (opponent, me) {
            ("R", "P") => 6,
            ("P", "S") => 6,
            ("S", "R") => 6,
            ("R", "R") => 3,
            ("P", "P") => 3,
            ("S", "S") => 3,
            ("R", "S") => 0,
            ("P", "R") => 0,
            ("S", "P") => 0,
            game => panic!("invalid game {:?}", game),
        };

        shape_points + round_points
    });

    let total_score: u64 = outcomes.sum();

    println!("[day02.p1] total score: {:?}", total_score);
}
