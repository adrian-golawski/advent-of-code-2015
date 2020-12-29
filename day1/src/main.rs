fn main() -> anyhow::Result<()> {
    let s = include_str!("input.txt");

    println!("[Part one]: {}", calculate_floor(s));
    println!("[Part two]: {}", find_basement(s).unwrap());

    Ok(())
}

fn calculate_floor(input: &str) -> i32 {
    return input.chars().fold(0, |level, c| match c {
        '(' => level + 1,
        ')' => level - 1,
        _ => panic!("Unexpected character"),
    });
}

fn find_basement(input: &str) -> Option<usize> {
    let mut level = 0;
    for (i, c) in input.chars().enumerate() {
        level = match c {
            '(' => level + 1,
            ')' => level - 1,
            _ => panic!("Unexpected character"),
        };
        if level == -1 {
            return Some(i + 1);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_floor() {
        assert_eq!(calculate_floor("(())"), 0);
        assert_eq!(calculate_floor("((("), 3);
        assert_eq!(calculate_floor("(()(()("), 3);
        assert_eq!(calculate_floor("())"), -1);
        assert_eq!(calculate_floor("))("), -1);
        assert_eq!(calculate_floor(")))"), -3);
        assert_eq!(calculate_floor(")())())"), -3);
    }
    #[test]
    fn test_find_basement() {
        assert_eq!(find_basement(")"), Some(1));
        assert_eq!(find_basement("()())"), Some(5));
    }
}

// --- Day 1: Not Quite Lisp ---

// Santa was hoping for a white Christmas, but his weather machine's "snow" function is powered by stars,
// and he's fresh out! To save Christmas, he needs you to collect fifty stars by December 25th.

// Collect stars by helping Santa solve puzzles. Two puzzles will be made available on each day in the Advent calendar;
// the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

// Here's an easy puzzle to warm you up.

// Santa is trying to deliver presents in a large apartment building, but he can't find the right floor - the directions he got are a little confusing.
// He starts on the ground floor (floor 0) and then follows the instructions one character at a time.

// An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.

// The apartment building is very tall, and the basement is very deep; he will never find the top or bottom floors.

// For example:

//     (()) and ()() both result in floor 0.
//     ((( and (()(()( both result in floor 3.
//     ))((((( also results in floor 3.
//     ()) and ))( both result in floor -1 (the first basement level).
//     ))) and )())()) both result in floor -3.

// To what floor do the instructions take Santa?

// --- Part Two ---

// Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1). The first character in the instructions has position 1, the second character has position 2, and so on.

// For example:

//     ) causes him to enter the basement at character position 1.
//     ()()) causes him to enter the basement at character position 5.

// What is the position of the character that causes Santa to first enter the basement?
