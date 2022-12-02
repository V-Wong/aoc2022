pub fn solve_a(input: &str) -> i32 {
    input
        .split('\n')
        .map(|line| {
            let op = line.split(' ').next().unwrap();
            let my = line.split(' ').nth(1).unwrap();

            score(my) + outcome(op, my)
        })
        .sum()
}

pub fn solve_b(input: &str) -> i32 {
    input
        .split('\n')
        .map(|line| {
            let op = line.split(' ').next().unwrap();
            let my = line.split(' ').nth(1).unwrap();

            match my {
                "X" => 0 + score(lose_against(op)),
                "Y" => 3 + score(draw_against(op)),
                "Z" => 6 + score(win_against(op)),
                _ => panic!(),
            }
        })
        .sum()
}

fn score(my: &str) -> i32 {
    match my {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!(),
    }
}

fn outcome(op: &str, my: &str) -> i32 {
    match (op, my) {
        ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        _ => panic!(),
    }
}

fn win_against(op: &str) -> &'static str {
    match op {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        _ => panic!(),
    }
}

fn draw_against(op: &str) -> &'static str {
    match op {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => panic!(),
    }
}

fn lose_against(op: &str) -> &'static str {
    match op {
        "A" => "Z",
        "B" => "X",
        "C" => "Y",
        _ => panic!(),
    }
}
