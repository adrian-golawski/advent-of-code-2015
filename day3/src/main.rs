use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let s = include_str!("input.txt");
    println!("[Part one]: {}", deliver(s));
    println!("[Part two]: {}", double_deliver(s));

    Ok(())
}

fn deliver(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    input.chars().fold((0, 0), |(x, y), c| {
        let new_position = match c {
            '>' => (x + 1, y),
            '^' => (x, y + 1),
            '<' => (x - 1, y),
            'v' => (x, y - 1),
            _ => panic!("Unexpected character"),
        };
        visited.insert(new_position);
        new_position
    });

    visited.len()
}

fn double_deliver(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .step_by(2)
        .fold(((0, 0), (0, 0)), |(santa, robot), c| {
            let santa_new_position = match &c[0] {
                '>' => (santa.0 + 1, santa.1),
                '^' => (santa.0, santa.1 + 1),
                '<' => (santa.0 - 1, santa.1),
                'v' => (santa.0, santa.1 - 1),
                _ => panic!("Unexpected character"),
            };
            visited.insert(santa_new_position);

            let robot_new_position = match &c[1] {
                '>' => (robot.0 + 1, robot.1),
                '^' => (robot.0, robot.1 + 1),
                '<' => (robot.0 - 1, robot.1),
                'v' => (robot.0, robot.1 - 1),
                _ => panic!("Unexpected character"),
            };
            visited.insert(robot_new_position);

            (santa_new_position, robot_new_position)
        });

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deliver() {
        assert_eq!(deliver(">"), 2);
        assert_eq!(deliver("^>v<"), 4);
        assert_eq!(deliver("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_double_deliver() {
        assert_eq!(double_deliver("^v"), 3);
        assert_eq!(double_deliver("^>v<"), 3);
        assert_eq!(double_deliver("^v^v^v^v^v"), 11);
    }
}

// --- Day 3: Perfectly Spherical Houses in a Vacuum ---

// Santa is delivering presents to an infinite two-dimensional grid of houses.

// He begins by delivering a present to the house at his starting location,
// and then an elf at the North Pole calls him via radio and tells him where to move next.
// Moves are always exactly one house to the north (^), south (v), east (>), or west (<).
// After each move, he delivers another present to the house at his new location.

// However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off,
// and Santa ends up visiting some houses more than once. How many houses receive at least one present?

// For example:

//     > delivers presents to 2 houses: one at the starting location, and one to the east.
//     ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
//     ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.

// --- Part Two ---

// The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

// Santa and Robo-Santa start at the same location (delivering two presents to the same starting house),
// then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

// This year, how many houses receive at least one present?

// For example:

//     ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
//     ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
//     ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
