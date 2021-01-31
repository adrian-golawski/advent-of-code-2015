fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(parse_reindeer)
        .collect::<Vec<Reindeer>>();

    let test_time = 2503;

    let best_distance: i32 = input
        .iter()
        .map(|reindeer| calculate_distance(reindeer, test_time))
        .max()
        .unwrap();

    println!("[Part one]: {}", best_distance);

    let best_reindeer_score = calculate_best_score(&input, test_time);

    println!("[Part two]: {}", best_reindeer_score);
}

fn calculate_best_score(reindeers: &[Reindeer], seconds: i32) -> i32 {
    let mut reindeers_with_scores: Vec<(Reindeer, i32)> = reindeers
        .iter()
        .map(|reindeer| (reindeer.clone(), 0))
        .collect();

    for s in 1..=seconds {
        let best_distance: i32 = reindeers_with_scores
            .iter()
            .map(|&(reindeer, _)| calculate_distance(&reindeer, s))
            .max()
            .unwrap();

        reindeers_with_scores = reindeers_with_scores
            .iter()
            .map(|&(reindeer, score)| { 
                if best_distance == calculate_distance(&reindeer, s) {
                    (reindeer, score + 1)
                } else {
                    (reindeer, score)
                }
            })
            .collect();
    }

    reindeers_with_scores
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1
}

fn calculate_distance(reindeer: &Reindeer, seconds: i32) -> i32 {
    let mut seconds_left = seconds;
    let mut distance = 0;

    while seconds_left > reindeer.time + reindeer.rest {
        distance += reindeer.time * reindeer.speed;
        seconds_left -= reindeer.time + reindeer.rest
    }

    return if seconds_left > reindeer.time {
        distance + reindeer.time * reindeer.speed
    } else {
        distance + reindeer.speed * seconds_left
    };
}

fn parse_reindeer<'a>(input: &'a str) -> Reindeer<'a> {
    peg::parser! {
        grammar reindeer_parser() for str {
            rule name() -> &'input str = n:$(['a'..='z' | 'A'..='Z']+) {
                n
            }

            rule num() -> i32 = n:$(['0'..='9']+) { n.parse().unwrap() }

            pub rule parse() -> Reindeer<'input> = name:name() " can fly " speed:num() " km/s for " time:num() " seconds, but then must rest for " rest:num() " seconds." {
                Reindeer {
                    name,
                    speed,
                    time,
                    rest
                }
            }
        }
    }

    reindeer_parser::parse(input).unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Reindeer<'a> {
    name: &'a str,
    speed: i32,
    time: i32,
    rest: i32,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_best_score() {
        assert_eq!(
            calculate_best_score(
                &[
                    Reindeer {
                        name: "Comet",
                        speed: 14,
                        time: 10,
                        rest: 127
                    },
                    Reindeer {
                        name: "Dancer",
                        speed: 16,
                        time: 11,
                        rest: 162
                    }
                ],
                1000
            ),
            689
        )
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_reindeer(
                "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."
            ),
            Reindeer {
                name: "Comet",
                speed: 14,
                time: 10,
                rest: 127
            }
        );
    }

    #[test]
    fn test_calculate_distance() {
        // assert_eq!(
        //     calculate_distance(
        //         &Reindeer {
        //             name: "Comet",
        //             speed: 1,
        //             time: 100,
        //             rest: 100
        //         },
        //         100
        //     ),
        //     100
        // );
        // assert_eq!(
        //     calculate_distance(
        //         &Reindeer {
        //             name: "Comet",
        //             speed: 1,
        //             time: 100,
        //             rest: 100
        //         },
        //         200
        //     ),
        //     100
        // );
        assert_eq!(
            calculate_distance(
                &Reindeer {
                    name: "Comet",
                    speed: 14,
                    time: 10,
                    rest: 127
                },
                1000
            ),
            1120
        );
    }
}

// --- Day 14: Reindeer Olympics ---

// This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must rest occasionally to recover their energy. Santa would like to know which of his reindeer is fastest, and so he has them race.

// Reindeer can only either be flying (always at their top speed) or resting (not moving at all), and always spend whole seconds in either state.

// For example, suppose you have the following Reindeer:

//     Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
//     Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.

// After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten seconds, Comet has gone 140 km, while Dancer has gone 160 km. On the eleventh second, Comet begins resting (staying at 140 km), and Dancer continues on for a total distance of 176 km. On the 12th second, both reindeer are resting. They continue to rest until the 138th second, when Comet flies for another ten seconds. On the 174th second, Dancer flies for another 11 seconds.

// In this example, after the 1000th second, both reindeer are resting, and Comet is in the lead at 1120 km (poor Dancer has only gotten 1056 km by that point). So, in this situation, Comet would win (if the race ended at 1000 seconds).

// Given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, what distance has the winning reindeer traveled?

// --- Part Two ---

// Seeing how reindeer move in bursts, Santa decides he's not pleased with the old scoring system.

// Instead, at the end of each second, he awards one point to the reindeer currently in the lead. (If there are multiple reindeer tied for the lead, they each get one point.) He keeps the traditional 2503 second time limit, of course, as doing otherwise would be entirely ridiculous.

// Given the example reindeer from above, after the first second, Dancer is in the lead and gets one point. He stays in the lead until several seconds into Comet's second burst: after the 140th second, Comet pulls into the lead and gets his first point. Of course, since Dancer had been in the lead for the 139 seconds before that, he has accumulated 139 points by the 140th second.

// After the 1000th second, Dancer has accumulated 689 points, while poor Comet, our old champion, only has 312. So, with the new scoring system, Dancer would win (if the race ended at 1000 seconds).

// Again given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, how many points does the winning reindeer have?
