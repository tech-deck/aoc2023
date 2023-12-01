use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
    fs,
    io::{BufRead, BufReader, Read},
};

fn main() {
    println!("part one answer is: {}", solve_part_one("1").unwrap());
    println!("part two answer is: {}", solve_part_two("1").unwrap());
}

fn solve_part_one(file_name: &str) -> Result<u32, Box<dyn Error>> {
    let file = fs::File::open(format!("data/{file_name}.txt"))?;
    let mut reader = BufReader::new(file);

    let mut res = 0;
    for line in reader.lines() {
        let line = line?;
        let first_digit = line.chars().find(|ch| ch.is_numeric()).unwrap();
        let last_digit = line.chars().rev().find(|ch| ch.is_numeric()).unwrap();
        let number = u32::from_str_radix(&format!("{first_digit}{last_digit}"), 10)?;
        res += number;
    }
    Ok(res)
}
#[test]
fn test_part_one() {
    assert_eq!(solve_part_one(&"1_pt1_example").unwrap(), 142);
}

fn solve_part_two(file_name: &str) -> Result<u32, Box<dyn Error>> {
    let file = fs::File::open(format!("data/{file_name}.txt"))?;
    let mut reader = BufReader::new(file);

    let valid_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut res = 0;
    for line in reader.lines() {
        let line = line?;
        let mut digit_positions = BTreeMap::new();
        for (i, &digit) in valid_digits.iter().enumerate() {
            let digit_as_number = (i + 1) as u8;
            for (i, _) in line.match_indices(digit) {
                digit_positions.insert(i, digit_as_number);
            }
        }
        line.chars().enumerate().for_each(|(i, ch)| {
            if ch.is_numeric() {
                digit_positions.insert(i, ch as u8 - '0' as u8);
            }
        });
        println!("digit_positions: {:?}", digit_positions);
        let digits = digit_positions.values().cloned().collect::<Vec<u8>>();
        let res_part =
            u32::from_str_radix(&format!("{}{}", digits[0], digits[digits.len() - 1]), 10)?;
        res += res_part;
    }
    Ok(res)
}

#[test]
fn test_solve_part_two() {
    assert_eq!(solve_part_two("1_pt2_example").unwrap(), 281);
}
