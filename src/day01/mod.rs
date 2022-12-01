use std::fs;

const FILENAME: &str = "src/day01/input.txt";

pub fn run() {
    println!("Day01");

    let content = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    let lines = content.lines();

    let mut count = 0;
    let mut vec = Vec::new();

    lines.for_each(|f| {
        if f == "" {
            vec.push(count);
            count = 0;
            return;
        }

        count += f.parse::<i32>().unwrap();
    });

    vec.sort();
    vec.reverse();

    println!("First: {}", vec[0]);

    let first_three: i32 = vec[0..3].iter().sum();
    println!("Second: {}", first_three);
}
