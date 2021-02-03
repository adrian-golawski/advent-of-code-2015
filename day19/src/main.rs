use std::{collections::HashSet};

use regex::Regex;

fn main() {
    let replacements: Vec<(&str, &str)> = include_str!("input.txt")
        .lines()
        .map(parse_replacements)
        .collect();

    let molecule = "CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr";

    println!("[Part one] {}", find_replacements(molecule, &replacements));

    println!("[Part two] {}", reduction_count(molecule, &replacements));
}

fn find_replacements<'a>(input: &str, replacements: &[(&str, &str)]) -> usize {
    let mut replacements_found: HashSet<String> = HashSet::new();

    replacements.iter().for_each(|&(from, to)| {
        let re = Regex::new(from).unwrap();

        for i in 0..=input.len() {
            let smaller_input = &input.clone()[i..input.len()];

            re.captures_iter(&smaller_input).for_each(|_| {
                let mut new_input = input[0..i].to_string();
                new_input.push_str(&smaller_input.replacen(from, to, 1));
                replacements_found.insert(new_input.clone());
            });
        }
    });

    replacements_found.iter().count()
}

fn parse_replacements(input: &str) -> (&str, &str) {
    let splits = input.split(" => ").collect::<Vec<&str>>();
    return (splits[0], splits[1]);
}

fn reduction_count(input: &str, replacements: &[(&str, &str)]) -> usize {
    let mut reduction_counter = 0;

    let mut input = input.to_string();

    while input.chars().any(|c| c != 'e') {
        replacements.iter().for_each(|&(to, from)| {
            let re = Regex::new(from).unwrap();

            if re.is_match(&input) {
                input = input.replacen(from, to, 1);
                reduction_counter += 1;
            }
        });
    }

    return reduction_counter;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let inputs = ["Al => ThF", "Al => ThRnFAr", "B => BCa", "B => TiB"];
        let expected = [("Al", "ThF"), ("Al", "ThRnFAr"), ("B", "BCa"), ("B", "TiB")];

        inputs
            .iter()
            .zip(expected.iter())
            .for_each(|(&input, &expected)| {
                assert_eq!(parse_replacements(input), expected);
            });
    }

    #[test]
    fn test_replacement() {
        let replacements = [("H", "HO"), ("H", "OH"), ("O", "HH")];
        assert_eq!(find_replacements("HOH", &replacements), 4);
        assert_eq!(find_replacements("HOHOHO", &replacements), 7);
    }

    #[test]
    fn test_reduction() {
        let replacements = [
            ("e", "H"),
            ("e", "O"),
            ("H", "HO"),
            ("H", "OH"),
            ("O", "HH"),
        ];
        assert_eq!(reduction_count("HOH", &replacements), 3);
        assert_eq!(reduction_count("HOHOHO", &replacements), 6);
    }
}

// --- Day 19: Medicine for Rudolph ---

// Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very brightly, and he needs medicine.

// Red-Nosed Reindeer biology isn't similar to regular reindeer biology;
// Rudolph is going to need custom-made medicine.
// Unfortunately, Red-Nosed Reindeer chemistry isn't similar to regular reindeer chemistry, either.

// The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission plant,
// capable of constructing any Red-Nosed Reindeer molecule you need.
// It works by starting with some input molecule and then doing a series of replacements,
// one per step, until it has the right molecule.

// However, the machine has to be calibrated before it can be used.
// Calibration involves determining the number of molecules that can be generated in one step from a given starting point.

// For example, imagine a simpler machine that supports only the following replacements:

// H => HO
// H => OH
// O => HH

// Given the replacements above and starting with HOH, the following molecules could be generated:

//     HOOH (via H => HO on the first H).
//     HOHO (via H => HO on the second H).
//     OHOH (via H => OH on the first H).
//     HOOH (via H => OH on the second H).
//     HHHH (via O => HH).

// So, in the example above, there are 4 distinct molecules (not five, because HOOH appears twice) after one replacement from HOH.
// Santa's favorite molecule, HOHOHO, can become 7 distinct molecules (over nine replacements: six from H, and three from O).

// The machine replaces without regard for the surrounding characters.
// For example, given the string H2O, the transition H => OO would result in OO2O.

// Your puzzle input describes all of the possible replacements and, at the bottom,
// the medicine molecule for which you need to calibrate the machine.
// How many distinct molecules can be created after all the different ways you can do one replacement on the medicine molecule?
