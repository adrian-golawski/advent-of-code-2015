use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let mut distances = HashSet::new();
    let mut cities = HashSet::new();

    include_str!("input.txt")
        .lines()
        .map(parse_distance)
        .for_each(|d| {
            distances.insert(d.clone());
            distances.insert(Distance {
                from: d.to,
                to: d.from,
                distance: d.distance,
            });
            distances.insert(Distance {
                from: "root",
                to: d.from,
                distance: 0,
            });
            distances.insert(Distance {
                from: "root",
                to: d.to,
                distance: 0,
            });

            cities.insert(d.from);
            cities.insert(d.to);
        });

    let (min, max) = solve_rec(&cities, &distances, "root", HashSet::new());

    println!("[Part one]: {}", min);
    println!("[Part two]: {}", max);

    Ok(())
}

fn parse_distance<'a>(input: &'a str) -> Distance<'a> {
    peg::parser! {
        grammar distance_parser() for str {
            rule name() -> &'input str = n:$(['a'..='z' | 'A'..='Z']+) {
                n
            }

            rule num() -> u64 = n:$(['0'..='9']+) { n.parse().unwrap() }

            pub rule parse() -> Distance<'input> = from:name() " to " to:name() " = " distance:num() {
                Distance {
                    from,
                    to,
                    distance
                }
            }
        }
    }

    distance_parser::parse(input).unwrap()
}

fn solve_rec(
    cities: &HashSet<&str>,
    distances: &HashSet<Distance>,
    current: &str,
    visited: HashSet<&str>,
) -> (u64, u64) {
    if visited.len() == cities.len() {
        return (0, 0);
    }
    let mut min = u64::MAX;
    let mut max = 0;
    for city in cities.iter() {
        if visited.contains(city) {
            continue;
        }
        let mut visited = visited.clone();
        visited.insert(*city);
        let t = solve_rec(&cities, &distances, city, visited);
        let t_min = t.0
            + distances
                .iter()
                .find(|d| d.from == current && d.to == *city)
                .unwrap()
                .distance;
        if t_min < min {
            min = t_min;
        }
        let t_max = t.1
            + distances
                .iter()
                .find(|d| d.from == current && d.to == *city)
                .unwrap()
                .distance;

        if t_max > max {
            max = t_max;
        }
    }
    return (min, max);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Distance<'a> {
    from: &'a str,
    to: &'a str,
    distance: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_distance("Faerun to Tristram = 65"),
            Distance {
                from: "Faerun",
                to: "Tristram",
                distance: 65
            }
        );
    }
}

// --- Day 9: All in a Single Night ---

// Every year, Santa manages to deliver all of his presents in a single night.

// This year, however, he has some new locations to visit;
// his elves have provided him the distances between every pair of locations.
// He can start and end at any two (different) locations he wants, but he must visit each location exactly once.
// What is the shortest distance he can travel to achieve this?

// For example, given the following distances:

// London to Dublin = 464
// London to Belfast = 518
// Dublin to Belfast = 141

// The possible routes are therefore:

// Dublin -> London -> Belfast = 982
// London -> Dublin -> Belfast = 605
// London -> Belfast -> Dublin = 659
// Dublin -> Belfast -> London = 659
// Belfast -> Dublin -> London = 605
// Belfast -> London -> Dublin = 982

// The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is 605 in this example.

// What is the distance of the shortest route?
