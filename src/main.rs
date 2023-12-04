use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("input/day_01.txt") {
        Ok(i) => i,
        Err(err) => panic!("{err}"),
    };

    let day_01 = day_01(input);
    println!("{}", day_01);
}

fn day_01(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|character| character.to_digit(10));
            let first = it.next().expect("should be a number") as i32;

            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<i32>()
            .expect("Should be a valid number")
        })
        .sum::<i32>()
}
