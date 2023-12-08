use maplit::hashmap;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn get_string_number(code: String) -> i32 {
    let numbers = hashmap! {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    };

    let mut result = Default::default();

    for (key, value) in numbers {
        if code.contains(key) {
            result = value;
        }
    }

    return result;
}

fn get_numbers(code: String) -> i32 {
    let mut first = Default::default();
    let mut last = Default::default();

    let mut possible_numbers = String::new();

    for char in code.chars() {
        let int_char = char.to_digit(10);

        if int_char.is_some() {
            first = int_char.unwrap() as i32;
            break;
        }

        possible_numbers.push(char);
        let num = get_string_number(possible_numbers.clone());

        if num != Default::default() {
            first = num;
            possible_numbers = String::new();
            break;
        }
    }

    for char in code.chars().rev() {
        let int_char = char.to_digit(10);

        if int_char.is_some() {
            last = int_char.unwrap() as i32;
            break;
        }

        possible_numbers.push(char);
        let num = get_string_number(possible_numbers.clone().chars().rev().collect());

        if num != Default::default() {
            last = num;
            break;
        }
    }

    let sfirst = first.to_string();
    let slast = last.to_string();

    let result = sfirst + &slast;

    return result.parse::<i32>().unwrap();
}

fn read_file(file: String) -> i32 {
    let lines = read_lines(&file);

    let mut sum = 0;

    for line in lines {
        let number = get_numbers(line);
        sum += number;
    }

    return sum;
}

pub fn day1() {
    println!("Day 1: {}", read_file("day1.txt".to_string()));
}
