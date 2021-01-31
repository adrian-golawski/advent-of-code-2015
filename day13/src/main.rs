use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let relations: Vec<Relation> = include_str!("input.txt")
        .lines()
        .map(parse_relation)
        .collect();
    let mut people: HashSet<&str> = relations.iter().map(|r| r.a).collect();

    let max_happiness: i32 = people
        .iter()
        .map(|x| *x)
        .permutations(people.len())
        .map(|arrangement| calculate_group_happiness(&arrangement, &relations))
        .max()
        .unwrap();

    println!("[Part one]: {}", max_happiness);

    people.insert("me");

    let max_happiness: i32 = people
        .iter()
        .map(|x| *x)
        .permutations(people.len())
        .map(|arrangement| calculate_group_happiness(&arrangement, &relations))
        .max()
        .unwrap();

    println!("[Part two]: {}", max_happiness);
}

fn calculate_group_happiness(people: &[&str], relations: &[Relation]) -> i32 {
    let people_circle = people
        .iter()
        .circular_tuple_windows::<(&&str, &&str, &&str)>();

    people_circle.fold(0, |sum, (left, middle, right)| {
        // dbg!(left, middle, right);
        return sum + calculate_hapiness(middle, (left, right), relations);
    })
}

fn calculate_hapiness(person: &str, (left, right): (&str, &str), relations: &[Relation]) -> i32 {
    relations.iter().fold(0, |sum, relation| {
        if relation.a == person && (relation.b == left || relation.b == right) {
            return sum + relation.change;
        }

        return sum;
    })
}

fn parse_relation<'a>(input: &'a str) -> Relation<'a> {
    peg::parser! {
        grammar relation_parser() for str {
            rule name() -> &'input str = n:$(['a'..='z' | 'A'..='Z']+) {
                n
            }

            rule num() -> i32 = n:$(['0'..='9']+) { n.parse().unwrap() }

            pub rule parse() -> Relation<'input> = a:name() " would " result:name() " " n:num() " happiness units by sitting next to " b:name() "." {
                match result {
                    "lose" => Relation {
                        a,
                        b,
                        change: -n
                    },
                    "gain" => Relation {
                        a,
                        b,
                        change: n
                    },
                    _ => panic!("Unexpected string")
                }
            }
        }
    }

    relation_parser::parse(input).unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Relation<'a> {
    a: &'a str,
    b: &'a str,
    change: i32,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_relation("Alice would gain 54 happiness units by sitting next to Bob."),
            Relation {
                a: "Alice",
                b: "Bob",
                change: 54
            }
        );
        assert_eq!(
            parse_relation("Alice would lose 54 happiness units by sitting next to Bob."),
            Relation {
                a: "Alice",
                b: "Bob",
                change: -54
            }
        );
    }

    #[test]
    fn test_calculate_hapiness() {
        let relations = [
            Relation {
                a: "a",
                b: "b",
                change: 100,
            },
            Relation {
                a: "a",
                b: "c",
                change: 50,
            },
        ];

        assert_eq!(calculate_hapiness("a", ("b", "c"), &relations), 150);
    }
    #[test]
    fn test_calculate_group_hapiness() {
        let relations = [
            Relation {
                a: "a",
                b: "b",
                change: 1,
            },
            Relation {
                a: "a",
                b: "c",
                change: 10,
            },
            Relation {
                a: "b",
                b: "a",
                change: 100,
            },
            Relation {
                a: "b",
                b: "c",
                change: 1000,
            },
            Relation {
                a: "c",
                b: "a",
                change: 10000,
            },
            Relation {
                a: "c",
                b: "b",
                change: 100000,
            },
        ];

        assert_eq!(
            calculate_group_happiness(&["a", "b", "c"], &relations),
            111111
        );
    }
}

// --- Day 13: Knights of the Dinner Table ---

// In years past, the holiday feast with your family hasn't gone so well. Not everyone gets along! This year, you resolve, will be different. You're going to find the optimal seating arrangement and avoid all those awkward conversations.

// You start by writing up a list of everyone invited and the amount their happiness would increase or decrease if they were to find themselves sitting next to each other person. You have a circular table that will be just big enough to fit everyone comfortably, and so each person will have exactly two neighbors.

// For example, suppose you have only four attendees planned, and you calculate their potential happiness as follows:

// Alice would gain 54 happiness units by sitting next to Bob.
// Alice would lose 79 happiness units by sitting next to Carol.
// Alice would lose 2 happiness units by sitting next to David.
// Bob would gain 83 happiness units by sitting next to Alice.
// Bob would lose 7 happiness units by sitting next to Carol.
// Bob would lose 63 happiness units by sitting next to David.
// Carol would lose 62 happiness units by sitting next to Alice.
// Carol would gain 60 happiness units by sitting next to Bob.
// Carol would gain 55 happiness units by sitting next to David.
// David would gain 46 happiness units by sitting next to Alice.
// David would lose 7 happiness units by sitting next to Bob.
// David would gain 41 happiness units by sitting next to Carol.

// Then, if you seat Alice next to David, Alice would lose 2 happiness units (because David talks so much), but David would gain 46 happiness units (because Alice is such a good listener), for a total change of 44.

// If you continue around the table, you could then seat Bob next to Alice (Bob gains 83, Alice gains 54). Finally, seat Carol, who sits next to Bob (Carol gains 60, Bob loses 7) and David (Carol gains 55, David gains 41). The arrangement looks like this:

//      +41 +46
// +55   David    -2
// Carol       Alice
// +60    Bob    +54
//      -7  +83

// After trying every other seating arrangement in this hypothetical scenario, you find that this one is the most optimal, with a total change in happiness of 330.

// What is the total change in happiness for the optimal seating arrangement of the actual guest list?
