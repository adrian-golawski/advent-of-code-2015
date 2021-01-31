fn main() {
    let searched_aunt = Aunt {
        id: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let input = include_str!("input.txt");

    let aunts = input
        .lines()
        .map(parse_aunt)
        .filter(|aunt| matches_aunt(aunt, &searched_aunt))
        .collect::<Vec<Aunt>>();

    println!("[Part one]: {}", aunts.get(0).unwrap().id);

    let aunts = input
        .lines()
        .map(parse_aunt)
        .filter(|aunt| matches_aunt_2(aunt, &searched_aunt))
        .collect::<Vec<Aunt>>();

    println!("[Part two]: {}", aunts.get(0).unwrap().id);
}

fn matches_aunt(suspicious_aunt: &Aunt, searched_aunt: &Aunt) -> bool {
    let children = match suspicious_aunt.children {
        Some(v) => searched_aunt.children.unwrap() == v,
        None => true,
    };
    let cats = match suspicious_aunt.cats {
        Some(v) => searched_aunt.cats.unwrap() == v,
        None => true,
    };
    let samoyeds = match suspicious_aunt.samoyeds {
        Some(v) => searched_aunt.samoyeds.unwrap() == v,
        None => true,
    };
    let pomeranians = match suspicious_aunt.pomeranians {
        Some(v) => searched_aunt.pomeranians.unwrap() == v,
        None => true,
    };
    let akitas = match suspicious_aunt.akitas {
        Some(v) => searched_aunt.akitas.unwrap() == v,
        None => true,
    };
    let vizslas = match suspicious_aunt.vizslas {
        Some(v) => searched_aunt.vizslas.unwrap() == v,
        None => true,
    };
    let goldfish = match suspicious_aunt.goldfish {
        Some(v) => searched_aunt.goldfish.unwrap() == v,
        None => true,
    };
    let trees = match suspicious_aunt.trees {
        Some(v) => searched_aunt.trees.unwrap() == v,
        None => true,
    };
    let cars = match suspicious_aunt.cars {
        Some(v) => searched_aunt.cars.unwrap() == v,
        None => true,
    };
    let perfumes = match suspicious_aunt.perfumes {
        Some(v) => searched_aunt.perfumes.unwrap() == v,
        None => true,
    };

    children
        && cats
        && samoyeds
        && pomeranians
        && akitas
        && vizslas
        && goldfish
        && trees
        && cars
        && perfumes
}

fn matches_aunt_2(suspicious_aunt: &Aunt, searched_aunt: &Aunt) -> bool {
    let children = match suspicious_aunt.children {
        Some(v) => searched_aunt.children.unwrap() == v,
        None => true,
    };
    let cats = match suspicious_aunt.cats {
        Some(v) => searched_aunt.cats.unwrap() < v,
        None => true,
    };
    let samoyeds = match suspicious_aunt.samoyeds {
        Some(v) => searched_aunt.samoyeds.unwrap() == v,
        None => true,
    };
    let pomeranians = match suspicious_aunt.pomeranians {
        Some(v) => searched_aunt.pomeranians.unwrap() > v,
        None => true,
    };
    let akitas = match suspicious_aunt.akitas {
        Some(v) => searched_aunt.akitas.unwrap() == v,
        None => true,
    };
    let vizslas = match suspicious_aunt.vizslas {
        Some(v) => searched_aunt.vizslas.unwrap() == v,
        None => true,
    };
    let goldfish = match suspicious_aunt.goldfish {
        Some(v) => searched_aunt.goldfish.unwrap() > v,
        None => true,
    };
    let trees = match suspicious_aunt.trees {
        Some(v) => searched_aunt.trees.unwrap() < v,
        None => true,
    };
    let cars = match suspicious_aunt.cars {
        Some(v) => searched_aunt.cars.unwrap() == v,
        None => true,
    };
    let perfumes = match suspicious_aunt.perfumes {
        Some(v) => searched_aunt.perfumes.unwrap() == v,
        None => true,
    };

    children
        && cats
        && samoyeds
        && pomeranians
        && akitas
        && vizslas
        && goldfish
        && trees
        && cars
        && perfumes
}

fn pick_field(field_name: &str, fields: [(&str, u32); 3]) -> Option<u32> {
    match fields.iter().find(|(name, _)| name == &field_name) {
        Some((_, value)) => Some(*value),
        None => None,
    }
}

fn parse_aunt<'a>(input: &str) -> Aunt {
    peg::parser! {
        grammar aunt_parser() for str {
            rule field() -> (&'input str, u32) =  name:name() ": " amount:num() {
                (name, amount)
            }

            rule name() -> &'input str = n:$(['a'..='z' | 'A'..='Z']+) {
                n
            }

            rule num() -> u32 = n:$(['0'..='9']+) { n.parse().unwrap() }

            pub rule parse() -> Aunt = "Sue " id:num() ": " field1:field() ", " field2:field() ", " field3:field() {
                let fields: [(&'input str, u32); 3] = [field1, field2, field3];

                Aunt {
                    id,
                    children: pick_field("children", fields),
                    cats: pick_field("cats", fields),
                    samoyeds: pick_field("samoyeds", fields),
                    pomeranians: pick_field("pomeranians", fields),
                    akitas: pick_field("akitas", fields),
                    vizslas: pick_field("vizslas", fields),
                    goldfish: pick_field("goldfish", fields),
                    trees: pick_field("trees", fields),
                    cars: pick_field("cars", fields),
                    perfumes: pick_field("perfumes", fields)
                }
            }
        }
    }

    aunt_parser::parse(input).unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Aunt {
    id: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_aunt("Sue 1: children: 1, cats: 8, vizslas: 7"),
            Aunt {
                id: 1,
                children: Some(1),
                cats: Some(8),
                samoyeds: None,
                pomeranians: None,
                akitas: None,
                vizslas: Some(7),
                goldfish: None,
                trees: None,
                cars: None,
                perfumes: None
            }
        );
    }

    #[test]
    fn test_match_aunt() {
        assert_eq!(
            matches_aunt(
                &Aunt {
                    id: 0,
                    children: Some(2),
                    cats: Some(7),
                    samoyeds: Some(2),
                    pomeranians: Some(3),
                    akitas: Some(0),
                    vizslas: Some(0),
                    goldfish: Some(5),
                    trees: Some(3),
                    cars: Some(2),
                    perfumes: Some(1),
                },
                &Aunt {
                    id: 0,
                    children: Some(3),
                    cats: Some(7),
                    samoyeds: Some(2),
                    pomeranians: Some(3),
                    akitas: Some(0),
                    vizslas: Some(0),
                    goldfish: Some(5),
                    trees: Some(3),
                    cars: Some(2),
                    perfumes: Some(1),
                }
            ),
            false
        );
        assert_eq!(
            matches_aunt(
                &Aunt {
                    id: 0,
                    children: Some(3),
                    cats: Some(7),
                    samoyeds: Some(2),
                    pomeranians: None,
                    akitas: Some(0),
                    vizslas: Some(0),
                    goldfish: Some(5),
                    trees: Some(3),
                    cars: Some(2),
                    perfumes: Some(1),
                },
                &Aunt {
                    id: 0,
                    children: Some(3),
                    cats: Some(7),
                    samoyeds: Some(2),
                    pomeranians: Some(3),
                    akitas: Some(0),
                    vizslas: Some(0),
                    goldfish: Some(5),
                    trees: Some(3),
                    cars: Some(2),
                    perfumes: Some(1),
                }
            ),
            true
        );
    }
}

// --- Day 16: Aunt Sue ---

// Your Aunt Sue has given you a wonderful gift, and you'd like to send her a thank you card. However, there's a small problem: she signed it "From, Aunt Sue".

// You have 500 Aunts named "Sue".

// So, to avoid sending the card to the wrong person, you need to figure out which Aunt Sue (which you conveniently number 1 to 500, for sanity) gave you the gift. You open the present and, as luck would have it, good ol' Aunt Sue got you a My First Crime Scene Analysis Machine! Just what you wanted. Or needed, as the case may be.

// The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few specific compounds in a given sample, as well as how many distinct kinds of those compounds there are. According to the instructions, these are what the MFCSAM can detect:

//     children, by human DNA age analysis.
//     cats. It doesn't differentiate individual breeds.
//     Several seemingly random breeds of dog: samoyeds, pomeranians, akitas, and vizslas.
//     goldfish. No other kinds of fish.
//     trees, all in one group.
//     cars, presumably by exhaust or gasoline or something.
//     perfumes, which is handy, since many of your Aunts Sue wear a few kinds.

// In fact, many of your Aunts Sue have many of these. You put the wrapping from the gift into the MFCSAM. It beeps inquisitively at you a few times and then prints out a message on ticker tape:

// children: 3
// cats: 7
// samoyeds: 2
// pomeranians: 3
// akitas: 0
// vizslas: 0
// goldfish: 5
// trees: 3
// cars: 2
// perfumes: 1

// You make a list of the things you can remember about each Aunt Sue. Things missing from your list aren't zero - you simply don't remember the value.

// What is the number of the Sue that got you the gift?

// --- Part Two ---

// As you're about to send the thank you note, something in the MFCSAM's instructions catches your eye. Apparently, it has an outdated retroencabulator, and so the output from the machine isn't exact values - some of them indicate ranges.

// In particular, the cats and trees readings indicates that there are greater than that many (due to the unpredictable nuclear decay of cat dander and tree pollen), while the pomeranians and goldfish readings indicate that there are fewer than that many (due to the modial interaction of magnetoreluctance).

// What is the number of the real Aunt Sue?
