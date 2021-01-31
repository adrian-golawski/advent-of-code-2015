use std::char;

fn main() -> anyhow::Result<()> {
    let mut input = "1113122113".to_string();

    for _ in 0..40 {
        input = parse_string(input);
    }

    println!("[Part one]: {}", input.len());

    for _ in 0..10 {
        input = parse_string(input);
    }

    println!("[Part two]: {}", input.len());

    Ok(())
}

fn parse_string(input: String) -> String {
    let mut parsed = String::new();

    let mut chars = input.chars();

    let mut current_num = chars.next().unwrap();
    let mut counter = 1;

    for c in chars {
        if c == current_num {
            counter += 1;
            continue;
        }

        parsed.push(char::from_digit(counter, 10).unwrap());
        parsed.push(current_num);

        current_num = c;
        counter = 1;
    }

    parsed.push(char::from_digit(counter, 10).unwrap());
    parsed.push(current_num);

    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(parse_string("1".to_string()), "11".to_string());
        assert_eq!(parse_string("11".to_string()), "21".to_string());
        assert_eq!(parse_string("21".to_string()), "1211".to_string());
        assert_eq!(parse_string("1211".to_string()), "111221".to_string());
        assert_eq!(parse_string("111221".to_string()), "312211".to_string());
    }
}

// --- Day 10: Elves Look, Elves Say ---

// Today, the Elves are playing a game called look-and-say. They take turns making sequences by reading aloud the previous sequence and using that reading as the next sequence. For example, 211 is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).

// Look-and-say sequences are generated iteratively, using the previous value as input for the next step. For each step, take the previous value, and replace each run of digits (like 111) with the number of digits (3) followed by the digit itself (1).

// For example:

//     1 becomes 11 (1 copy of digit 1).
//     11 becomes 21 (2 copies of digit 1).
//     21 becomes 1211 (one 2 followed by one 1).
//     1211 becomes 111221 (one 1, one 2, and two 1s).
//     111221 becomes 312211 (three 1s, two 2s, and one 1).

// Starting with the digits in your puzzle input, apply this process 40 times. What is the length of the result?

// Your puzzle input is 1113122113.
