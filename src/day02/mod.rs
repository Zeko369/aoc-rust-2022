use std::fs;

const FILENAME: &str = "src/day02/input.txt";

pub fn run() {
    println!("Day02");

    let content = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    let lines = content.lines();

    let mut score1 = 0;
    let mut score2 = 0;

    lines.for_each(|line| {
        let mut letters = line.split(" ");
        let (them, us) = (letters.next().unwrap(), letters.next().unwrap());

        score1 += match us {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            &_ => todo!(),
        };

        // A/X - rock, B/Y - paper, C/Z - scissors

        score1 += match us {
            "X" => match them {
                "A" => 3,
                "B" => 0,
                "C" => 6,
                &_ => todo!(),
            },
            "Y" => match them {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                &_ => todo!(),
            },
            "Z" => match them {
                "A" => 0,
                "B" => 6,
                "C" => 3,
                &_ => todo!(),
            },
            &_ => todo!(),
        };

        score2 += match us {
            "Z" => match them {
                "A" => 2 + 6,
                "B" => 3 + 6,
                "C" => 1 + 6,
                &_ => todo!(),
            },
            "Y" => match them {
                "A" => 1 + 3,
                "B" => 2 + 3,
                "C" => 3 + 3,
                &_ => todo!(),
            },
            "X" => match them {
                "A" => 3 + 0,
                "B" => 1 + 0,
                "C" => 2 + 0,
                &_ => todo!(),
            },
            &_ => todo!(),
        };
    });

    println!("First: {}", score1);
    println!("Second: {}", score2);
}
