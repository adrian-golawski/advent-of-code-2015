fn main() -> anyhow::Result<()> {
    let s: Vec<(bool, bool)> = include_str!("input.txt")
        .lines()
        .map(|line| (is_nice_1(line), is_nice_2(line)))
        .collect();
    println!("[Part one]: {}", s.iter().filter(|x| x.0).count());
    println!("[Part two]: {}", s.iter().filter(|x| x.1).count());

    Ok(())
}

fn is_nice_1(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let vowels = chars
        .iter()
        .filter(|&c| "aeiou".contains(&c.to_string()))
        .count();
    let double = chars.windows(2).any(|c| c[0] == c[1]);

    let forbidden = ["ab", "cd", "pq", "xy"];
    let contains_forbidden = forbidden.iter().any(|&c| input.contains(&c));
    return vowels >= 3 && double && !contains_forbidden;
}

fn is_nice_2(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let pairs = chars
        .windows(2)
        .map(|c| c.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    let mut paired = false;

    for (i, my_str) in pairs.clone().iter().enumerate() {
        let pairs_clone: Vec<String> = pairs
            .clone()
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i && *j + 1 != i && *j != i + 1)
            .map(|(_, s)| s.clone())
            .collect();

        if pairs_clone.iter().any(|pair| pair == my_str) {
            paired = true;
            break;
        }
    }

    let overlap = chars.windows(3).any(|c| c[0] == c[2]);
    return overlap && paired;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_1() {
        assert_eq!(is_nice_1("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice_1("aaa"), true);
        assert_eq!(is_nice_1("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice_1("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice_1("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_is_nice_2() {
        assert_eq!(is_nice_2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_2("xxyxx"), true);
        assert_eq!(is_nice_2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_2("ieodomkazucvgmuy"), false);
    }
}

// --- Day 5: Doesn't He Have Intern-Elves For This? ---

// Santa needs help figuring out which strings in his text file are naughty or nice.

// A nice string is one with all of the following properties:

//     It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
//     It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
//     It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

// For example:

//     ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
//     aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
//     jchzalrnumimnmhp is naughty because it has no double letter.
//     haegwjzuvuyypxyu is naughty because it contains the string xy.
//     dvszwmarrgswjxmb is naughty because it contains only one vowel.

// How many strings are nice?

// --- Part Two ---

// Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice.
// None of the old rules apply, as they are all clearly ridiculous.

// Now, a nice string is one with all of the following properties:

//     It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
//     It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

// For example:

//     qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
//     xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
//     uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
//     ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

// How many strings are nice under these new rules?
