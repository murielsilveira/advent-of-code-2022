use std::fs;

pub fn solve() {
    let content = fs::read_to_string("../_inputs/day02.txt").unwrap();

    let rounds: Vec<_> = content
        .trim()
        .split('\n')
        .map(|line| {
            let round: Vec<_> = line.split(' ').collect();
            (round[0], round[1])
        })
        .collect();

    let intuitive_rounds = rounds.iter().map(compute_intuitive_round);
    let strategic_rounds = rounds.iter().map(compute_strategic_round);

    let intuitive_score: u64 = intuitive_rounds.map(compute_round_points).sum();
    let strategic_score: u64 = strategic_rounds.map(compute_round_points).sum();

    println!("[day02.p1] intuitive score: {:?}", intuitive_score);
    println!("[day02.p2] strategic score: {:?}", strategic_score);
}

fn compute_intuitive_round((opponent, me): &(&str, &str)) -> (&'static str, &'static str) {
    let opponent = match *opponent {
        "A" => "R",
        "B" => "P",
        "C" => "S",
        shape => panic!("invalid shape {}", shape),
    };
    let me = match *me {
        "X" => "R",
        "Y" => "P",
        "Z" => "S",
        shape => panic!("invalid shape {}", shape),
    };

    (opponent, me)
}

fn compute_strategic_round((opponent, me): &(&str, &str)) -> (&'static str, &'static str) {
    let opponent = match *opponent {
        "A" => "R",
        "B" => "P",
        "C" => "S",
        shape => panic!("invalid shape {}", shape),
    };
    let instruction = match *me {
        "X" => "L",
        "Y" => "D",
        "Z" => "W",
        shape => panic!("invalid shape {}", shape),
    };
    let me = match (opponent, instruction) {
        ("R", "L") => "S",
        ("R", "D") => "R",
        ("R", "W") => "P",
        ("P", "L") => "R",
        ("P", "D") => "P",
        ("P", "W") => "S",
        ("S", "L") => "P",
        ("S", "D") => "S",
        ("S", "W") => "R",
        game => panic!("invalid game {:?}", game),
    };

    (opponent, me)
}

fn compute_round_points((opponent, me): (&str, &str)) -> u64 {
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
}
