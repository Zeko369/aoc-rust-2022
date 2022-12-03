use std::collections::HashSet;
use std::fs;

const FILENAME: &str = "src/day03/input.txt";

fn get_score(set: HashSet<char>) -> u32 {
    let mut score = 0;

    for item in set {
        let val = item.clone() as u32;
        if val >= 97 {
            score += val - 97 + 1;
        } else {
            score += val - 65 + 26 + 1;
        }
    }

    return score;
}

fn get_unique(items: Vec<Vec<char>>) -> HashSet<char> {
    let mut final_set: HashSet<char> = HashSet::new();
    for chr in &items[0] {
        let exists = items[1..]
            .into_iter()
            .all(|l| l.into_iter().any(|c| c == chr));

        if exists {
            final_set.insert(*chr);
        }
    }

    return final_set;
}

pub fn run() {
    println!("Day03");

    let content = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    let mut lines = Vec::new();
    content.lines().into_iter().for_each(|l| lines.push(l));

    let mut score1 = 0;
    let mut score2 = 0;

    for i in 0..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        score1 += get_score(get_unique(vec![
            chars[0..lines[i].len() / 2].to_vec(),
            chars[lines[i].len() / 2..].to_vec(),
        ]));

        if i % 3 == 0 {
            score2 += get_score(get_unique(vec![
                lines[i].chars().collect(),
                lines[i + 1].chars().collect(),
                lines[i + 2].chars().collect(),
            ]))
        }
    }

    println!("First: {}", score1);
    println!("Second: {}", score2);
}
