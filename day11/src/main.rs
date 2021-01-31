use std::collections::HashSet;

fn main() {
    let input = "vzbxkghb".to_string();

    let next_password = get_next_password(input);

    println!("[Part one]: {}", next_password);
    println!("[Part two]: {}", get_next_password(next_password));
}

fn get_next_password(input: String) -> String {
    let mut new_password = increment_password(input);

    while !has_increasing_letters(&new_password)
        || !has_two_pairs_of_letters(&new_password)
        || has_forbidden_letters(&new_password)
    {
        new_password = increment_password(new_password);
    }

    new_password
}

fn has_increasing_letters(input: &String) -> bool {
    for n in 0..=input.len() - 3 {
        let c1 = input.chars().nth(n).unwrap() as u32;
        let c2 = input.chars().nth(n + 1).unwrap() as u32;
        let c3 = input.chars().nth(n + 2).unwrap() as u32;

        if c1 + 1 == c2 && c1 + 2 == c3 {
            return true;
        };
    }

    false
}

fn has_forbidden_letters(input: &String) -> bool {
    input.contains('l') || input.contains('o') || input.contains('i')
}

fn has_two_pairs_of_letters(input: &String) -> bool {
    let mut pairs: HashSet<u32> = HashSet::new();
    for n in 0..=input.len() - 2 {
        let c1 = input.chars().nth(n).unwrap() as u32;
        let c2 = input.chars().nth(n + 1).unwrap() as u32;

        if c1 == c2 {
            pairs.insert(c1);
        };
    }

    pairs.len() >= 2
}

fn increment_password(input: String) -> String {
    let inversed_input = input.chars().rev().collect::<String>();
    let mut chars = inversed_input.chars();

    let mut result = String::new();

    let (mut next_char, mut overflow) = get_next_char(chars.next().unwrap());
    result.push(next_char);

    for c in chars {
        if overflow {
            let result = get_next_char(c);
            overflow = result.1;
            next_char = result.0;
        } else {
            next_char = c;
        }

        result.push(next_char);
    }

    result.chars().rev().collect()
}

fn get_next_char(c: char) -> (char, bool) {
    return if c == 'z' {
        ('a', true)
    } else {
        (std::char::from_u32(c as u32 + 1).unwrap_or(c), false)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_char() {
        assert_eq!(get_next_char('a'), ('b', false));
        assert_eq!(get_next_char('z'), ('a', true));
    }

    #[test]
    fn test_increment() {
        assert_eq!(increment_password("a".to_string()), "b".to_string());
        assert_eq!(increment_password("az".to_string()), "ba".to_string());
        assert_eq!(
            increment_password("bzzzzzz".to_string()),
            "caaaaaa".to_string()
        );
    }

    #[test]
    fn test_get_next_password() {
        assert_eq!(
            get_next_password("abcdefgh".to_string()),
            "abcdffaa".to_string()
        );
        assert_eq!(
            get_next_password("ghijklmn".to_string()),
            "ghjaabcc".to_string()
        );
    }

    #[test]
    fn test_has_increasing_letters() {
        assert_eq!(has_increasing_letters(&"abc".to_string()), true);
        assert_eq!(has_increasing_letters(&"abd".to_string()), false);
    }
    #[test]
    fn test_has_forbidden_letters() {
        assert_eq!(has_forbidden_letters(&"iol".to_string()), true);
        assert_eq!(has_forbidden_letters(&"abc".to_string()), false);
    }
    #[test]
    fn test_has_two_pairs_of_letters() {
        assert_eq!(has_two_pairs_of_letters(&"aabb".to_string()), true);
        assert_eq!(has_two_pairs_of_letters(&"abab".to_string()), false);
        assert_eq!(has_two_pairs_of_letters(&"abba".to_string()), false);
    }
}

// --- Day 11: Corporate Policy ---

// Santa's previous password expired, and he needs help choosing a new one.

// To help him remember his new password after the old one expires, Santa has devised a method of coming up with a password based on the previous one. Corporate policy dictates that passwords must be exactly eight lowercase letters (for security reasons), so he finds his new password by incrementing his old password string repeatedly until it is valid.

// Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter to the left until one doesn't wrap around.

// Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional password requirements:

//     Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
//     Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
//     Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.

// For example:

//     hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
//     abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
//     abbcegjk fails the third requirement, because it only has one double letter (bb).
//     The next password after abcdefgh is abcdffaa.
//     The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.

// Given Santa's current password (your puzzle input), what should his next password be?

// Your puzzle input is vzbxkghb.
